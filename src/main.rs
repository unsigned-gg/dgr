//! dgr — mathematical and machine-sympathetic directed acyclic graph (DAG) runtime.
//!
//! `dgr check` lints a run-state document against CONTRACT.md v3,
//! `dgr view` draws it, `dgr stats` reports flow analytics over it.

mod check;
mod contract;
mod liveness;
mod message;
mod model;
mod picker;
mod render;
mod scale;
mod select;
mod stats;
mod style;
mod view;

use std::process::ExitCode;

const USAGE: &str = "\
dgr — mathematical and machine-sympathetic directed acyclic graph runtime

USAGE:
  dgr check <run.json> [--json] [--strict]
      lint a run-state document against the contract
  dgr view [run.json] [--snapshot [--compact]] [--width N] [--select ID]
      draw the run: interactive terminal DAG (j/k · tab · enter · m message · ? · q),
      watches the file for changes; --snapshot prints one frame to stdout;
      --compact gives that snapshot the normal four-row browse inspector.
      Without a path: $DGR_RUN, then .dgr/run.json / run.json under cwd
      Set DGR_WORKING_GLYPH=* for the ASCII working-state fallback.
  dgr stats <run.json> [--json]
      flow analytics over per-attempt timestamps: age, time-in-state,
      rework rate, WIP, critical path + naive ETA
  dgr --skill
      print the producer skill — the agent-facing instructions for
      writing and maintaining a run file.

check exit codes: 0 clean (warnings allowed unless --strict), 1 findings, 2 usage/IO
";

/// Bundled at build time so the printed skill always matches this binary's
/// release — and so a tree that shipped the binary without the skill fails
/// to compile rather than shipping an agent-facing hole.
const SKILL: &str = include_str!("../skills/dgr-producer/SKILL.md");

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("check") => cmd_check(&args[1..]),
        Some("view") => cmd_view(&args[1..]),
        Some("stats") => stats::run(&args[1..]),
        Some("--skill") => {
            print!("{SKILL}");
            ExitCode::SUCCESS
        }
        Some("--version" | "-V") => {
            println!(
                "dgr {} (contract v3; reads v1/v2)",
                env!("CARGO_PKG_VERSION")
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprint!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn cmd_view(args: &[String]) -> ExitCode {
    let mut path: Option<String> = None;
    let mut snapshot = false;
    let mut compact_snapshot = false;
    let mut width: Option<usize> = None;
    let mut select: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--snapshot" => snapshot = true,
            "--compact" => compact_snapshot = true,
            "--width" => match it.next().and_then(|v| v.parse().ok()) {
                Some(v) => width = Some(v),
                None => {
                    eprintln!("dgr view: --width needs a number");
                    return ExitCode::from(2);
                }
            },
            "--select" => select = it.next().cloned(),
            other if !other.starts_with('-') && path.is_none() => path = Some(other.into()),
            other => {
                eprintln!("dgr view: unexpected argument {other:?}\n\n{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    if width.is_some_and(|width| width > scale::MAX_FRAME_WIDTH) {
        eprintln!(
            "dgr view: --width exceeds the {} column limit",
            scale::MAX_FRAME_WIDTH
        );
        return ExitCode::from(2);
    }
    if compact_snapshot && !snapshot {
        eprintln!("dgr view: --compact requires --snapshot");
        return ExitCode::from(2);
    }
    let path = path.unwrap_or_else(discover_run_file);
    view::run(view::ViewArgs {
        path,
        snapshot,
        compact_snapshot,
        width,
        select,
    })
}

/// Where is the run file? Explicit beats implicit: `$DGR_RUN`,
/// then `.dgr/run.json` / `run.json` under current working directory.
fn discover_run_file() -> String {
    if let Ok(p) = std::env::var("DGR_RUN") {
        if !p.is_empty() {
            return p;
        }
    }
    let base = std::env::current_dir()
        .map(|d| d.display().to_string())
        .unwrap_or_else(|_| ".".into());
    for cand in [".dgr/run.json", "run.json"] {
        let p = format!("{base}/{cand}");
        if std::path::Path::new(&p).exists() {
            return p;
        }
    }
    format!("{base}/run.json")
}

fn cmd_check(args: &[String]) -> ExitCode {
    let mut path: Option<&str> = None;
    let mut json_out = false;
    let mut strict = false;
    for a in args {
        match a.as_str() {
            "--json" => json_out = true,
            "--strict" => strict = true,
            other if !other.starts_with('-') && path.is_none() => path = Some(other),
            other => {
                eprintln!("dgr check: unexpected argument {other:?}\n\n{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let Some(path) = path else {
        eprint!("{USAGE}");
        return ExitCode::from(2);
    };

    let raw = match scale::read_limited(path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("dgr check: {e}");
            return ExitCode::from(2);
        }
    };
    let doc: contract::Doc = match serde_json::from_str(&raw) {
        Ok(d) => d,
        Err(e) => {
            // A type-level parse failure is itself a finding: report it in
            // both output modes with serde's line:column location.
            if json_out {
                println!(
                    "{}",
                    serde_json::json!([{
                        "level": "error", "code": "E001",
                        "path": format!("{}:{}:{}", path, e.line(), e.column()),
                        "msg": format!("not a contract document: {e}"),
                    }])
                );
            } else {
                println!(
                    "ERROR   E001 {path}:{}:{} — not a contract document: {e}",
                    e.line(),
                    e.column()
                );
            }
            return ExitCode::from(1);
        }
    };

    if let Err(msg) = scale::enforce_document(&doc) {
        if json_out {
            println!(
                "{}",
                serde_json::json!([{
                    "level": "error", "code": "E002", "path": path, "msg": msg,
                }])
            );
        } else {
            println!("ERROR   E002 {path} — {msg}");
        }
        return ExitCode::from(1);
    }

    let report = check::check(&doc);

    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&report.findings).expect("findings serialize")
        );
    } else {
        for f in &report.findings {
            let level = match f.level {
                check::Level::Error => "ERROR  ",
                check::Level::Warning => "warning",
            };
            println!("{level} {} {} — {}", f.code, f.path, f.msg);
        }
        let (e, w) = (report.errors(), report.warnings());
        if e == 0 && w == 0 {
            println!(
                "✓ clean — {} tasks, {} attempts, {} events",
                report.tasks, report.attempts, report.events
            );
        } else {
            println!(
                "{} error{}, {} warning{} — {} tasks, {} attempts, {} events",
                e,
                if e == 1 { "" } else { "s" },
                w,
                if w == 1 { "" } else { "s" },
                report.tasks,
                report.attempts,
                report.events
            );
        }
    }

    let failed = report.errors() > 0 || (strict && report.warnings() > 0);
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

const CONFIG_FILES: &[&str] = &["qbit.yml", "qbit.yaml", "qbit.toml"];

fn sandbox_dir() -> PathBuf {
    PathBuf::from("dev-sandbox")
}

fn ensure_sandbox() -> std::io::Result<()> {
    let dir = sandbox_dir();
    if !dir.exists() { fs::create_dir_all(&dir)?; }
    sync_config_files()?;
    Ok(())
}

fn reset_sandbox() -> std::io::Result<()> {
    let dir = sandbox_dir();
    if dir.exists() { fs::remove_dir_all(&dir)?; }
    ensure_sandbox()
}

fn run_qbit_in_sandbox(input: &str) -> std::io::Result<i32> {
    let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    if parts.is_empty() { return Ok(0); }

    sync_config_files()?;

    let sandbox = sandbox_dir();
    let sandbox_abs = sandbox
        .canonicalize()
        .unwrap_or_else(|_| sandbox.clone());

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin").arg("qbit-cli")
        .arg("--")
        .current_dir(&sandbox)
        .env("QBIT_PROJECT_ROOT", sandbox_abs)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    for p in &parts { cmd.arg(p); }

    eprintln!("[dev] running in sandbox: qbit {}", parts.join(" "));
    let status = cmd.status()?;
    Ok(status.code().unwrap_or(1))
}

fn normalize_for_internal(raw: &str) -> String {
    raw.trim().trim_matches(':').to_lowercase()
}

#[derive(Clone, Copy, Debug)]
struct WatchConfig { debounce_ms: u64 }

struct WatchState {
    enabled: bool,
    last_cmd: Option<String>,
    _watcher: Option<RecommendedWatcher>, // keep watcher alive
}

impl WatchState {
    fn new() -> Self { Self { enabled: false, last_cmd: None, _watcher: None } }
}

fn setup_watcher(tx: Sender<()>) -> notify::Result<RecommendedWatcher> {
    // notify v8 style: closure receives Result<Event>
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(ev) = res {
            let interesting = matches!(
                ev.kind,
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
            );
            if interesting {
                let _ = tx.send(());
            }
        }
    })?;

    // Watch common roots
    let roots: &[&Path] = &[
        Path::new("."),             // helps reliability on Windows
        Path::new("src"),
        Path::new("Cargo.toml"),
        Path::new("Cargo.lock"),
    ];
    for r in roots {
        if r.exists() {
            watcher.watch(r, RecursiveMode::Recursive)?;
        }
    }

    Ok(watcher)
}

fn sync_config_files() -> std::io::Result<()> {
    let sandbox = sandbox_dir();
    for file in CONFIG_FILES {
        let src = Path::new(file);
        let dst = sandbox.join(file);
        if src.exists() {
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(src, &dst)?;
        } else if dst.exists() {
            fs::remove_file(&dst)?;
        }
    }
    Ok(())
}

fn drain_with_debounce(rx: &Receiver<()>, debounce_ms: u64) {
    let deadline = Instant::now() + Duration::from_millis(debounce_ms);
    while Instant::now() < deadline {
        if rx.recv_timeout(Duration::from_millis(40)).is_ok() {
            // keep draining within window
        }
    }
}

fn main() {
    if let Err(e) = ensure_sandbox() {
        eprintln!("[dev] failed to prepare sandbox: {e}");
        std::process::exit(1);
    }

    println!("qbit-dev — sandbox ready at: {}", sandbox_dir().to_string_lossy());
    println!("Type ':reset'/'reset', ':exit'/'exit', ':watch [on|off]', ':help'.\n");

    let cfg = WatchConfig { debounce_ms: 500 };
    let (tx, rx) = channel::<()>();
    let mut state = WatchState::new();

    match setup_watcher(tx.clone()) {
        Ok(w) => state._watcher = Some(w),
        Err(e) => eprintln!("[dev] watcher init failed (disabled): {e}"),
    }

    loop {
        if state.enabled {
            if rx.recv_timeout(Duration::from_millis(10)).is_ok() {
                drain_with_debounce(&rx, cfg.debounce_ms);
                if let Some(cmd) = state.last_cmd.as_ref() {
                    eprintln!("[dev] change detected — re-running: {cmd}");
                    match run_qbit_in_sandbox(cmd) {
                        Ok(code) if code != 0 => eprintln!("[dev] qbit exited with code {code}"),
                        Ok(_) => {}
                        Err(e) => eprintln!("[dev] failed to run qbit: {e}"),
                    }
                } else {
                    eprintln!("[dev] change detected but no previous command.");
                }
            }
        }

        print!("qbit-dev> ");
        io::stdout().flush().ok();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { continue; }
        let raw_input = line.trim();
        if raw_input.is_empty() { continue; }

        let internal = normalize_for_internal(raw_input);

        match internal.as_str() {
            "exit" | "quit" | "q" => { println!("[dev] bye"); break; }
            "reset" => {
                match reset_sandbox() {
                    Ok(_) => println!("[dev] sandbox reset"),
                    Err(e) => eprintln!("[dev] reset failed: {e}"),
                }
            }
            "help" => {
                println!(
                    "[dev] commands:
  - :reset | reset            clear dev-sandbox
  - :exit  | exit | quit      quit
  - :watch                    show watch status
  - :watch on                 enable hot-reload
  - :watch off                disable hot-reload
  - :help  | help             show this help
  - any other text            run 'qbit <args>' inside dev-sandbox"
                );
            }
            "watch" => println!("[dev] watch is {}", if state.enabled { "ON" } else { "OFF" }),
            "watch on" | "watch:on" => { state.enabled = true; println!("[dev] watch ON"); }
            "watch off" | "watch:off" => { state.enabled = false; println!("[dev] watch OFF"); }
            _ => {
                state.last_cmd = Some(raw_input.to_string());
                match run_qbit_in_sandbox(raw_input) {
                    Ok(code) if code != 0 => eprintln!("[dev] qbit exited with code {code}"),
                    Ok(_) => {}
                    Err(e) => eprintln!("[dev] failed to run qbit: {e}"),
                }
            }
        }
    }
}

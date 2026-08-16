mod cli;
mod config;
mod developers;
mod os;
mod tools;
mod utils;

fn main() {
    if let Ok(root) = std::env::var("QBIT_PROJECT_ROOT") {
        if let Err(e) = std::env::set_current_dir(&root) {
            eprintln!("warning: failed to switch to sandbox at {root}: {e}");
        }
    }

    run_update_check_best_effort();

    cli::run();
}

/// Runs the at-most-once-per-24h update check before normal command
/// dispatch. This is intentionally "best effort": the underlying
/// GitHub client uses a short (3s) timeout, and every possible
/// failure mode (network error, rate limit, cache I/O error) is
/// swallowed here rather than propagated. The only visible side
/// effect when a newer version is available is a single line to
/// stderr, so it can never pollute machine-readable stdout output.
///
/// If a check fails, the failure reason is only surfaced when
/// `QBIT_UPDATE_DEBUG=1` is set — by default it stays fully silent.
fn run_update_check_best_effort() {
    use os::update::CheckResult;

    let result = os::update::check_if_due();
    result.log_failure_if_debug_enabled();

    if let CheckResult::UpdateAvailable { latest } = result {
        eprintln!(
            "A new version of qbit is available: {latest} (run `qbit upgrade` to install it)"
        );
    }
    // NotDue, Disabled, UpToDate, and CheckFailed (without debug
    // logging) are all deliberately silent: none of them should ever
    // reach stdout, and none of them should block or fail the main
    // command.
}

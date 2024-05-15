use cmd_lib::run_cmd;

/// Enters a new cmd with all the env variables and active paths set
pub fn make_shell() -> cmd_lib::CmdResult {
    run_cmd! (
        cmd .
    )?;
    Ok(())
}

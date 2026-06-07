use super::SharedCliOptions;

#[test]
fn inherit_exec_root_options_carries_ignore_system_config() {
    let root = SharedCliOptions {
        ignore_system_config: true,
        ..Default::default()
    };
    let mut exec = SharedCliOptions::default();

    exec.inherit_exec_root_options(&root);

    assert!(exec.ignore_system_config);
}

#[test]
fn apply_subcommand_overrides_carries_ignore_system_config() {
    let subcommand = SharedCliOptions {
        ignore_system_config: true,
        ..Default::default()
    };
    let mut root = SharedCliOptions::default();

    root.apply_subcommand_overrides(subcommand);

    assert!(root.ignore_system_config);
}

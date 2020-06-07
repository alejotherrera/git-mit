use pb_hook_test_helper::assert_output;

#[test]
fn help_printed_when_no_arguments_passed() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-git-hooks", vec![]);
    let expected = &format!(
        r#"pb-git-hooks {}
Billie Thompson <billie+pb-git-hooks@billiecodes.com>
A command for enabling and disabling git lints

USAGE:
    pb-git-hooks [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --scope <scope>     [default: local]  [possible values: local, global]

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    lint    Manage active lints
"#,
        env!("CARGO_PKG_VERSION")
    );

    assert_output(&output, "", expected, false)
}
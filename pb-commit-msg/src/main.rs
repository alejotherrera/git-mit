extern crate pb_commit_message_lints;

use std::{env, fs};

use clap::{crate_authors, crate_version, App, Arg};
use git2::{Config, Repository};

use pb_commit_message_lints::{
    get_lint_configuration,
    has_duplicated_trailers,
    has_missing_pivotal_tracker_id,
    Lints,
};

#[repr(i32)]
enum ExitCode {
    DuplicatedTrailers = 1,
    PivotalTrackerIdMissing,
}

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(COMMIT_FILE_PATH_NAME)
                .help(
                    "Path to a temporary file that contains the commit message written by the \
                     developer",
                )
                .index(1)
                .required(true),
        )
        .get_matches();

    let commit_file_path = matches.value_of(COMMIT_FILE_PATH_NAME).unwrap();
    let commit_message =
        fs::read_to_string(commit_file_path).expect("Something went wrong reading the file");

    let current_dir = env::current_dir().expect("Unable to retrieve current directory");

    let get_repository_config = |x: Repository| x.config();
    let get_default_config = |_| Config::open_default();
    let git_config = Repository::discover(current_dir)
        .and_then(get_repository_config)
        .or_else(get_default_config)
        .expect("Couldn't load any git config");

    let checks =
        get_lint_configuration(&git_config).expect("Couldn't parse the configuration in git");

    for check in checks {
        match check {
            Lints::DuplicatedTrailers => {
                if let Some(trailers) = has_duplicated_trailers(&commit_message) {
                    exit_duplicated_trailers(&commit_message, &trailers);
                }
            }
            Lints::PivotalTrackerIdMissing => {
                if let Some(()) = has_missing_pivotal_tracker_id(&commit_message) {
                    exit_missing_pivotal_tracker_id(&commit_message);
                }
            }
        }
    }

    Ok(())
}

fn exit_missing_pivotal_tracker_id(commit_message: &str) {
    eprintln!(
        r#"
{}

Your commit is missing a Pivotal Tracker Id

You can fix this by adding the Id in one of the styles below to the commit message
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]
"#,
        commit_message
    );

    std::process::exit(ExitCode::PivotalTrackerIdMissing as i32);
}

fn exit_duplicated_trailers(commit_message: &str, trailers: &[String]) {
    let mut fields = FIELD_SINGULAR;
    if trailers.len() > 1 {
        fields = FIELD_PLURAL
    }

    eprintln!(
        r#"
{}

Your commit cannot have the same name duplicated in the "{}" {}

You can fix this by removing the duplicated field when you commit again
"#,
        commit_message,
        trailers.join("\", \""),
        fields
    );

    std::process::exit(ExitCode::DuplicatedTrailers as i32);
}

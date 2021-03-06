use thiserror::Error;

use mit_commit::CommitMessageError;
use mit_commit_message_lints::{external, mit, relates};

#[derive(Error, Debug)]
pub(crate) enum MitPrepareCommitMessageError {
    #[error("{0}")]
    MitCommitMessageLintsError(#[from] CommitMessageError),
    #[error("Failed to read mit config from `{0}`:\n{1}")]
    Io(String, String),
    #[error("failed to generate relates-to with `{0}`: {1}")]
    Exec(String, String),
    #[error("Expected commit file path")]
    MissingCommitFilePath,
    #[error("{0}")]
    AuthorWrite(#[from] mit::VcsError),
    #[error("{0}")]
    RelatesToWrite(#[from] relates::VcsError),
    #[error("{0}")]
    ReadFromVcs(#[from] external::Error),
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl MitPrepareCommitMessageError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> MitPrepareCommitMessageError {
        MitPrepareCommitMessageError::Io(source, format!("{}", error))
    }

    pub(crate) fn new_exec(source: String, error: &std::io::Error) -> MitPrepareCommitMessageError {
        MitPrepareCommitMessageError::Exec(source, format!("{}", error))
    }
}

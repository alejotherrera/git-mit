use crate::errors::MitCommitMessageLintsError;
use crate::external::vcs::Vcs;
use crate::lints::lib::Lints;

/// # Errors
///
/// Errors if writing to the VCS config fails
pub fn set_status(
    lints: Lints,
    vcs: &mut dyn Vcs,
    status: bool,
) -> Result<(), MitCommitMessageLintsError> {
    lints
        .config_keys()
        .into_iter()
        .try_for_each(|lint| vcs.set_str(&lint, &status.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests_can_enable_lints_via_a_command {
    use crate::external::vcs::InMemory;
    use crate::lints::cmd::set_status::set_status;
    use crate::lints::lib::{Lint, Lints};
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn we_can_enable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        set_status(
            Lints::new(vec![Lint::PivotalTrackerIdMissing]),
            &mut config,
            true,
        )
        .unwrap();

        let expected = "true".to_string();
        let actual = strings
            .get("pb.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }

    #[test]
    fn we_can_disable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        set_status(
            Lints::new(vec![Lint::PivotalTrackerIdMissing]),
            &mut config,
            false,
        )
        .unwrap();

        let expected = "false".to_string();
        let actual = strings
            .get("pb.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }
}
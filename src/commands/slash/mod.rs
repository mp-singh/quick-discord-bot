use core::fmt;
use std::str::FromStr;

pub mod events;
pub mod ping;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubCommand {
    Provision,
    UnProvision,
    List,
}

impl FromStr for SubCommand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "provision" => Ok(Self::Provision),
            "unprovision" => Ok(Self::UnProvision),
            "list" => Ok(Self::List),
            _ => Err(()),
        }
    }
}

impl fmt::Display for SubCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubCommand::Provision => write!(f, "provision"),
            SubCommand::UnProvision => write!(f, "unprovision"),
            SubCommand::List => write!(f, "list"),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sub_command() {
        assert_eq!(SubCommand::Provision.to_string(), "provision");
        assert_eq!(SubCommand::UnProvision.to_string(), "unprovision");
        assert_eq!(SubCommand::List.to_string(), "list");
    }

    #[test]
    fn test_sub_command_from_str() {
        assert_eq!(
            SubCommand::from_str("provision").unwrap(),
            SubCommand::Provision
        );
        assert_eq!(
            SubCommand::from_str("unprovision").unwrap(),
            SubCommand::UnProvision
        );
        assert_eq!(SubCommand::from_str("list").unwrap(), SubCommand::List);
    }
}

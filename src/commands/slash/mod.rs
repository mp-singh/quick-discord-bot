use core::fmt;
use std::str::FromStr;

pub mod chatgpt;
pub mod events;
pub mod general;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commands {
    KF2,
    Ping,
    ChatGPT,
    Ip,
    Joke,
    Yomama,
    Excuse,
    ChuckNorris,
    Now,
    Trump,
    Flip,
    Face,
    NASA,
}

impl FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kf2" => Ok(Commands::KF2),
            "ping" => Ok(Commands::Ping),
            "chatgpt" => Ok(Commands::ChatGPT),
            "ip" => Ok(Commands::Ip),
            "joke" => Ok(Commands::Joke),
            "yomama" => Ok(Commands::Yomama),
            "excuse" => Ok(Commands::Excuse),
            "chucknorris" => Ok(Commands::ChuckNorris),
            "now" => Ok(Commands::Now),
            "trump" => Ok(Commands::Trump),
            "flip" => Ok(Commands::Flip),
            "face" => Ok(Commands::Face),
            "nasa" => Ok(Commands::NASA),
            _ => Err(format!("{} is not a valid command", s)),
        }
    }
}

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
    #[allow(unused_imports)]
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

    #[test]
    fn test_command_from_str() {
        assert_eq!(Commands::from_str("kf2").unwrap(), Commands::KF2);
        assert_eq!(Commands::from_str("ping").unwrap(), Commands::Ping);
    }
}

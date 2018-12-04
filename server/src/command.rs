use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(PartialEq)]
pub enum Command {
  MESSAGE(String),
  NICKNAME(String),
  INVALID
}

// pub struct Command {
//   pub ctype: CommandType,
//   pub args: String
// }

impl Command {
  pub fn as_msg(&self, curr_user: String) -> String {
    match self {
      Command::NICKNAME(args) => {
        let mut temp = curr_user;
        temp.push_str(&" has changed nickname to ".to_string());
        temp.push_str(&args);
        return temp;
      },
      Command::MESSAGE(msg) => {
        let mut temp = curr_user;
        temp.push_str(&": ".to_string());
        temp.push_str(&msg);
        return temp;
      },
      _ => return "Invalid command".to_string()
    }
  }
}
impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !s.starts_with("/") {
      return Err(Error::new(ErrorKind::Other, "Command should start with /"));
    }
    
    let temp: Vec<&str> = s.trim_left_matches('/').split(' ').collect();
    let args = temp[1..].join(" ");
    let command = match temp[0] {
      "nickname" => Command::NICKNAME(args),
      "msg" => Command::MESSAGE(args),
      _ => Command::INVALID
    };

    if command == Command::INVALID {
      return Err(Error::new(ErrorKind::Other, "Invalid command type"));
    }

    
    Ok(command)
  }
}
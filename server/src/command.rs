use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(PartialEq)]
pub enum CommandType {
  MESSAGE,
  NICKNAME,
  INVALID
}

pub struct Command {
  pub ctype: CommandType,
  pub args: String
}

impl Command {
  pub fn as_msg(&self, curr_user: String) -> String {
    match self.ctype {
      CommandType::NICKNAME => {
        let mut temp = curr_user;
        temp.push_str(&" has changed nickname to ".to_string());
        temp.push_str(&self.args);
        return temp;
      },
      CommandType::MESSAGE => {
        let mut temp = curr_user;
        temp.push_str(&": ".to_string());
        temp.push_str(&self.args);
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
    

    let temp: Vec<&str> = s.trim_start_matches('/').split(' ').collect();
    let ctype = match temp[0] {
      "nickname" => CommandType::NICKNAME,
      "msg" => CommandType::MESSAGE,
      _ => CommandType::INVALID
    };

    if ctype == CommandType::INVALID {
      return Err(Error::new(ErrorKind::Other, "Invalid command type"));
    }

    let args = temp[1..].join(" ");
    Ok(Command { ctype: ctype, args: args })
  }
}
use std::str::FromStr;

#[derive(PartialEq)]
enum CommandType {
  MESSAGE,
  NICKNAME,
  INVALID
}

pub struct Command {
  ctype: CommandType,
  args: String
}

impl FromStr for Command {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {

    if !s.starts_with("/") {
      return Err(());
    }
    
    let temp: Vec<&str> = s.trim_start_matches('/').split(' ').collect();
    let ctype = match temp[0] {
      "nickname" => CommandType::NICKNAME,
      "msg" => CommandType::MESSAGE,
      _ => CommandType::INVALID
    };

    if ctype == CommandType::INVALID {
      return Err(());
    }

    let args = temp[1..].join(" ");
    Ok(Command { ctype: ctype, args: args })
  }
}
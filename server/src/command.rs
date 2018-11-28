use std::str::FromStr;
// use server::*;

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
  // pub fn update_server_model(&self, server: ServerModel, user_id: i32) -> String {
  //   if self.ctype == CommandType::NICKNAME {
  //     return server.change_nickname(user_id, &self.args);
  //   }
  //   return server.get_nickname(user_id);
  // }

  pub fn as_msg(&self, curr_user: String) -> String {
    match self.ctype {
      CommandType::NICKNAME => {
        let mut temp = curr_user;
        temp.push_str(&"has changed nickname to ".to_string());
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
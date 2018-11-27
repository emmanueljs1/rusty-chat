use std::collections::HashMap;

pub struct ServerModel {
  users: HashMap<i32, String>
}

impl ServerModel {
  pub fn new() -> ServerModel {
    ServerModel {
      users: HashMap::new()
    }
  }

  pub fn register_user(&mut self) -> i32 {
    println!("register user called");
    let id = self.generate_unique_id();
    let mut nickname = "User".to_string();
    let id_string = id.to_string();
    nickname.push_str(&id_string);
    self.users.insert(id, nickname);
    id
  }

  pub fn get_nickname(&self, id: i32) -> String {
    self.users.get(&id).expect("ID not found").to_string()
  }

  pub fn change_nickname(&mut self, id: i32, new_name: &str) {
    self.users.insert(id, new_name.to_string());
  }

  pub fn remove_user(&mut self, id: i32) {
    self.users.remove(&id);
  }

  fn generate_unique_id(&self) -> i32 {
    let mut available_int = 0;
    for i in 0..self.users.len() {
      if self.users.contains_key(&(i as i32)) {
        available_int += 1;
      } else {
        break;
      }
    }
    available_int
  }
}
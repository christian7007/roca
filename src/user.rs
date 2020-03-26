//! The user module allows to interact with OpenNebula users

use crate::client::ClientXMLRPC;
use xmlrpc::Value;

pub struct User {
    id: u32,
    body: String
}

impl User {

    pub fn new(id: u32) -> User {
        User{
            id,
            body: String::new(),
        }
    }

    pub fn info(&mut self, client: ClientXMLRPC) -> Result<(), String>{
        let req = client.new_request("one.user.info").arg(0);
        let rc  = client.call(req);

        match rc {
            Ok(val) => {
                match val {
                    Value::Array(response) => {
                        self.body = String::from(response[1].as_str().unwrap());
                    },
                    _ => panic!("Bad response type"),
                };
            },
            Err(val) => {
                return Err(val.to_string());
            }
        }

        Ok(())
    }

    pub fn print(&self) {
        println!("{}\n", self.body);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn user_info() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2")
        );

        let mut user = User::new(0);

        user.info(client);

        user.print();
    }
}
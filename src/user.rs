//! The user module allows to interact with OpenNebula users

use crate::client::ClientXMLRPC;

#[allow(dead_code)]
pub struct User {
    id: u32,
    body: String
}

#[allow(dead_code)]
impl User {

    pub fn new(id: u32) -> User {
        User{
            id,
            body: String::new(),
        }
    }

    pub fn info(&mut self, client: ClientXMLRPC) -> Result<(), String>{
        let req      = client.new_request("one.user.info").arg(0);
        let response = client.call(req);

        match response {
            Ok(resp) => {
                // TODO, check rc
                match resp.get_str(1) {
                    Some(body) => {
                        self.body = String::from(body);
                        Ok(())
                    },
                    _ => Err(String::from("The position required does not math the type."))
                }
            },
            Err(e) => Err(e)
        }
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

        match user.info(client) {
            Ok(()) => user.print(),
            _ => panic!("Error on user info"),
        }
    }
}
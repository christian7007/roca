//! The user module allows to interact with OpenNebula users

use crate::client::ClientXMLRPC;

#[allow(dead_code)]
pub struct User {
    client: ClientXMLRPC,
    id: i32,
    body: String,
}

#[allow(dead_code)]
impl User {

    pub fn new(client: ClientXMLRPC, id: i32) -> User {
        User{
            client,
            id,
            body: String::new(),
        }
    }

    // TODOs:
    // - enum form auth_drv options
    // - more helpers without some options (i.e groups, auth_drv)
    // - add last parameter: array of groups. Currently the client only support Array(Vec<Value>)
    pub fn allocate(client: &ClientXMLRPC, name: &str, passwd: &str, auth_drv: &str) -> Result<i32, String> {
        let req = client.new_request("one.user.allocate").arg(name)
                                                         .arg(passwd)
                                                         .arg(auth_drv);
        let response = client.call(req);

        match response {
            Ok(resp) => {
                if resp.rc() {
                    if let Some(value) = resp.get_int(1) {
                        return Ok(value);
                    }
                } else if let Some(value) = resp.get_str(1) {
                        return Err(String::from(value));
                }
            },

            Err(e) => return Err(e)
        };

        Err(String::from("The position required does not math the type."))
    }

    pub fn delete(client: &ClientXMLRPC, id: i32) -> Result<(), String> {
        let req = client.new_request("one.user.delete").arg(id);
        let response = client.call(req);

        match response {
            Ok(resp) => {
                if resp.rc() {
                    Ok(())
                }else {
                    match resp.get_str(1) {
                        Some(body) => Err(String::from(body)),
                        _ => Err(String::from("The position required does not math the type."))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn info(&mut self) -> Result<(), String>{
        let req      = self.client.new_request("one.user.info").arg(0);
        let response = self.client.call(req);

        match response {
            Ok(resp) => {
                match resp.get_str(1) {
                    Some(body) => {
                        if resp.rc() {
                            self.body = String::from(body);
                            Ok(())
                        } else {
                            Err(String::from(body))
                        }
                    },
                    _ => Err(String::from("The position required does not math the type."))
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn passwd(&self, new_passd: i32) -> Result<(), String> {
        let req      = self.client.new_request("one.user.passwd").arg(new_passd);
        let response = self.client.call(req);

        match response {
            Ok(resp) => {
                if resp.rc() {
                    Ok(())
                }else {
                    match resp.get_str(1) {
                        Some(body) => Err(String::from(body)),
                        _ => Err(String::from("The position required does not math the type."))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn login(&self, name: &str, token: &str, period: i32, gid: i32) -> Result<String, String> {
        let req      = self.client.new_request("one.user.login").arg(name)
                                                                .arg(token)
                                                                .arg(period)
                                                                .arg(gid);
        let response = self.client.call(req);

        match response {
            Ok(resp) => {
                match resp.get_str(1) {
                    Some(token) => {
                        if resp.rc() {
                            Ok(String::from(token))
                        } else {
                            Err(String::from(token))
                        }
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

        let mut user = User::new(client, 0);

        match user.info() {
            Ok(()) => user.print(),
            _ => panic!("Error on user info"),
        }
    }

    #[test]
    fn user_allocate_delete() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2")
        );

        // Create the user
        let response = User::allocate(&client, "test-alloc", "test-alloc", "");

        let user_id = match response {
            Ok(id) => id,
            _ => panic!("Error allocating the user"),
        };

        println!("User ID (test-rust): {}", user_id);

        assert!(user_id > 0);

        // Delete the user
        let response = User::delete(&client, user_id);

        assert_eq!(response, Ok(()));
    }

    #[test]
    fn user_login() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2")
        );

        // Create the user
        let name = "test-login";
        let response = User::allocate(&client, name, "password", "");

        let user_id = match response {
            Ok(id) => id,
            _ => panic!("Error allocating the user"),
        };

        let user = User::new(client, user_id);

        // Test loging
        let response = user.login(name, "", 60, 0);

        match response {
            Ok(_) => {}
            Err(e) => panic!(e),
        };

        // Delete the user
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2")
        );

        let response = User::delete(&client, user_id);

        assert_eq!(response, Ok(()));
    }
}
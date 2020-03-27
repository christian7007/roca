//! The client module acts as a wrapper of XML-RPC client to add OpenNebula related helpers

use xmlrpc;
use xmlrpc::{Request, Value};

/// Struct for storing Client related information
pub struct ClientXMLRPC {
    auth: String,
    endpoint: String,
}

pub struct Response {
    args: Vec<Value>,
}

#[allow(dead_code)]
impl ClientXMLRPC {

    // TODO, defines method for reading oen_auth

    pub fn new(auth: String, endpoint: String) -> ClientXMLRPC {
        ClientXMLRPC {
            auth,
            endpoint,
        }
    }

    pub fn new_request<'a>(&self, name: &'a str) -> Request<'a> {
        Request::new(name).arg(self.auth.clone())
    }

    //Try to import https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
    // if works open a PR

    pub fn call(&self, request: Request) -> Result<Response, String>{

        match request.call_url(&self.endpoint) {
            Ok(rc) => Response::new(rc),
            Err(_) => Err(String::from("Cannot contact the server."))
        }
    }
}

#[allow(dead_code)]
impl Response {

    pub fn new(response: Value) -> Result<Response, String> {
        match response {
            Value::Array(args) => {
                Ok(Response{
                    args,
                })
            },
            _ => Err(String::from("Bad response type."))
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    /// Helpers
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_bool(&self, position: usize) -> Option<bool> {
        self.args[position].as_bool()
    }

    pub fn get_int(&self, position: usize) -> Option<i32> {
        self.args[position].as_i32()
    }

    pub fn get_str(&self, position: usize) -> Option<&str> {
        self.args[position].as_str()
    }

    pub fn rc(&self) -> bool{
        match self.get_bool(0) {
            Some(rc) => rc,
            _ => false
        }
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_client() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:badpassword"),
            String::from("http://localhost:2633/RPC2")
        );

        let req = client.new_request("one.vn.info").arg(0);
        let request_result = client.call(req).unwrap();

        assert_eq!(request_result.rc(), false);
    }

    #[test]
    fn one_rc() {
        let client = ClientXMLRPC::new(
            String::from("oneadmin:opennebula"),
            String::from("http://localhost:2633/RPC2")
        );

        let req = client.new_request("one.user.info").arg(0);
        let request_result = client.call(req).unwrap();

        assert_eq!(request_result.rc(), true);
    }
}
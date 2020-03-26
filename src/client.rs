//! The client module acts as a wrapper of XML-RPC client to add OpenNebula related helpers

use xmlrpc;
use xmlrpc::{Request, Value};

/// Struct for storing Client related information
pub struct ClientXMLRPC {
    auth: String,
    endpoint: String,
}

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

    pub fn call(&self, request: Request) -> Result<Value, xmlrpc::Error>{
        let rc = request.call_url(&self.endpoint)?;

        Ok(rc)
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

        let rc = match request_result {
            Value::Array(response) => response,
            _ => panic!("Bas response type"),
        };

        assert_eq!(
            rc[1].as_str().unwrap(),
            "[one.vn.info] User couldn\'t be authenticated, aborting call."
        );
    }
}
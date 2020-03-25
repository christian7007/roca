//! The client module acts as a wrapper of XML-RPC client to add OpenNebula related helpers

use xmlrpc;
use xmlrpc::Request;

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
        xmlrpc::Request::new(name).arg(self.auth.clone())
    }

    //Try to import https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
    // if works open a PR

    pub fn call(&self, request: xmlrpc::Request) {
        let rc = request.call_url(&self.endpoint);

        println!("Result: {:?}", rc);
    }
}

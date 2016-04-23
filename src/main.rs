extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

fn main() {
    let client = Client::new();
    let res = client.get("http://localhost:3000")
        .header(Connection::close())
        .send();
    
    match res {
        Ok(mut v) => {
            println!("{:?}", v);
            let mut body = String::new();
            v.read_to_string(&mut body).unwrap();
            println!("{}", body);
        },
        Err(_) => println!("Error!") 
    }
}
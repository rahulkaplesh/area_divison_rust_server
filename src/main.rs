mod server;
use std::{rc::Rc};

fn main() {
    let server_ad = server::Area_Divison_Server::new("127.0.0.1:9800");
    if let Ok(mut server_ad_s) = server_ad {
        server_ad_s.register_get_route("/", || {
            println!("I am here !");
        });
        server_ad_s.start_operations();
    }
    println!("Server Started!!");
}

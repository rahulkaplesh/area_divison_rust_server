use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{self};
use std::io::prelude::*;
use threadpool::ThreadPool;
use std::collections::HashMap;
use std::{rc::Rc};
use std::borrow::Borrow;
use std::fs;

type RouteFuncToBeExec = Rc<dyn Fn() + 'static>;

pub struct Area_Divison_Server {
    tcp_listener: TcpListener,
    _worker_pool: ThreadPool,
    get_route_map: HashMap<String, RouteFuncToBeExec>,
    put_route_map: HashMap<String, RouteFuncToBeExec>,
    post_route_map: HashMap<String, RouteFuncToBeExec>
}

impl Area_Divison_Server {
    pub fn new(server_string: &str) -> io::Result<Area_Divison_Server> {
        let tcp_listener = TcpListener::bind(server_string).unwrap();
        let pool = ThreadPool::new(4);
        println!("Server to be run at : {}", server_string);
        Ok(Area_Divison_Server{
            tcp_listener,
            _worker_pool: pool,
            get_route_map: HashMap::new(),
            put_route_map: HashMap::new(),
            post_route_map: HashMap::new()
        })
    }

    pub fn register_get_route<F>(&mut self, routes_path: &str, f: F)
    where
        F: Fn() + 'static,
    {
        self.get_route_map.insert(routes_path.to_string(), Rc::new(f));
    }

    pub fn register_put_route<F>(&mut self, routes_path: &str, f: F)
    where
        F: Fn() + 'static,
    {
        self.put_route_map.insert(routes_path.to_string(), Rc::new(f));
    }

    pub fn register_post_route<F>(&mut self, routes_path: &str, f: F)
    where
        F: Fn() + 'static,
    {
        self.post_route_map.insert(routes_path.to_string(), Rc::new(f));
    }

    pub fn start_operations(&self) {
        for stream in self.tcp_listener.incoming() {
            let mut stream_req = stream.unwrap();
            let mut buffer = [0; 1024];
            stream_req.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("Request : {}", request);
            if request.to_ascii_lowercase().contains("get") {
                println!("Recieved a get request");
                let request_vec: Vec<&str> = request.split('\n').collect();
                let header = request_vec[0].to_string();
                let path: Vec<&str> = header.split(' ').collect();
                println!("{}", path[1]);
                if let Some(func) = self.get_route_map.get(path[1]) {
                    func()
                } else {
                    let status_line = "HTTP/1.1 404 NOT FOUND\r\n";
                    let contents = fs::read_to_string("404.html").unwrap();
                    let response = format!(
                        "{}\r\nContent-Length: {}\r\n\r\n{}",
                        status_line,
                        contents.len(),
                        contents
                    );
                    stream_req.write(response.as_bytes()).unwrap();
                    stream_req.flush().unwrap();
                }
            } else if request.to_ascii_lowercase().contains("put") {
                println!("Recieved a put request");
            } else if request.to_ascii_lowercase().contains("post") {
                println!("Recieved a post request");
            }
        }
    }

}
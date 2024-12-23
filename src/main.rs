use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};
fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    println!("Server listening on: {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let request_info: Vec<&str> = request_line.split(" ").collect();
    let method = request_info[0];
    let endpoint = request_info[1];
    let protocol = request_info[2];

    if protocol == "HTTP/1.1" {
        if method == "GET" {
            let endpoint = if endpoint == "/" {
                "/index"
            } else {
                endpoint
            };

            let read_file_result = fs::read_to_string(format!("resources/routes{endpoint}.html"));
            let (status_line, contents): (String, String) = match read_file_result {
                Ok(s) => ("HTTP/1.1 200 OK".to_string(), s),
                Err(_error) => ("HTTP/1.1 404 NOT FOUND".to_string(), fs::read_to_string(format!("notfound.html")).unwrap()),
            };

            let length = contents.len();

            let respone = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(respone.as_bytes()).unwrap();
        }
    } else {
        println!("Error! Unrecognized protocol: {}", protocol);
    }
}

use std::{
    net::TcpStream, io::{prelude::*, BufRead, BufReader},
    fs
};

pub struct Response {
    status_code: u16,
    status_message: String,
    headers: Vec<String>,
    body: String
}

const HTTP_VERSION: &str = "HTTP/1.1";
const CRLF: &str = "\r\n";
const OK_HTML: &str = "assets/OK.html";
const NOT_FOUND_HTML: &str = "assets/404.html";

impl Response {
    pub fn new(status_code: u16) -> Response{
        return Response{status_code, status_message: String::from(""), headers: vec![], body: String::from("")};
    }

    pub fn set_status_code(&mut self, status_code: u16) {
        self.status_code = status_code;
    }

    pub fn set_status_message(&mut self, status_message: String) {
        self.status_message = status_message;
    }

    pub fn add_header(&mut self, header_key: String, header_val: String) {
        self.headers.push(format!("{header_key}: {header_val}"));
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
        self.headers.push(format!("Content-Length: {}", self.body.len()));
    }

    pub fn get_response(&self) -> String {
        let mut res = String::from("");
        res.push_str(format!("{} {} {}", HTTP_VERSION, self.status_code, self.status_message).as_str());
        
        for header in &self.headers {
            res.push_str(CRLF);
            res.push_str(header);
        }

        res.push_str(CRLF);
        res.push_str(CRLF);

        res.push_str(self.body.as_str());
        return res;
    }
}

pub fn handle_stream(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);

    let req_line = buf.lines().next().unwrap().unwrap();
    let mut path_iter = req_line.split_whitespace();

    let method = path_iter.next().unwrap();
    let path = path_iter.next().unwrap();

    let (status_code, status_message, body) = match method {
        "GET" => match path {
            "/" => (200, String::from("OK"), fs::read_to_string(OK_HTML).unwrap()),
            _ => (404, String::from("NOT_FOUND"), fs::read_to_string(NOT_FOUND_HTML).unwrap())
        },
        _ => (404, String::from("NOT_FOUND"), fs::read_to_string(NOT_FOUND_HTML).unwrap())
    };
    
    let mut res = Response::new(status_code);
    res.set_status_message(status_message);
    res.set_body(body);

    stream.write_all(res.get_response().as_bytes()).unwrap();
}


use log::*;
use simple_logger;
use std::fs;
use std::sync::Arc;
use threadpool::ThreadPool;
use tiny_http::{Header, Method, Request, Response, Server};

const MAX_THREADS: usize = 4;
const SERVER_ADDR: &str = "127.0.0.1:3000";

fn main() -> Result<(), i32> {
    // * Inits
    //logger
    simple_logger::init().unwrap();
    //server
    let main_serv = Server::http(SERVER_ADDR).unwrap();
    //shared thread server
    let server = Arc::new(main_serv);
    //pool
    let pool = ThreadPool::new(MAX_THREADS);

    // * Threads creation
    for worker_id in 0..MAX_THREADS {
        let thread_server = Arc::clone(&server);
        pool.execute(move || handle_server(thread_server, worker_id));
    }
    pool.join();
    Ok(())
}

// * Handling the server: accept requests, handle them and wether respond or handle the error
fn handle_server(thread_server: Arc<Server>, worker_id: usize) {
    for request in thread_server.incoming_requests() {
        debug!("New work for worker {}", worker_id);
        match handle_request(request) {
            Ok(()) => continue,
            Err(e) => handle_error(e),
        };
    }
}

fn handle_request(request: Request) -> Result<(), ErrorType> {
    match request.method() {
        Method::Get => {
            info!("GET request from {}", request.remote_addr());
            handle_method_get(request)
        }
        uk_method => Err(ErrorType::UnknownMethod(uk_method.clone())),
    }
}

fn handle_method_get(request: Request) -> Result<(), ErrorType> {
    let url = [
        "server",
        match match_path(request.url()) {
            Some(u) => u,
            None => return Ok(()),
        },
        "index.html",
    ]
    .join("/");

    trace!(target: "disks", "Serving page {}", &url);
    let html_content = match fs::read_to_string(&url) {
        Ok(c) => c,
        Err(_) => return Err(ErrorType::FileNotFound(url)),
    };

    // * Create response
    let content_type_header = Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
    let mut response = Response::from_string(html_content);
    response.add_header(content_type_header);
    match request.respond(response) {
        Ok(()) => Ok(()),
        Err(_) => Err(ErrorType::CantRespond),
    }
}

// * Path white_/black_listing
fn match_path(url: &str) -> Option<&str> {
    match url {
        //? Authorized paths
        "/" => Some("/"),
        //? Blacklisted paths
        "/favicon.ico" => None,
        //? Other paths: 404 error
        bad_path => {
            warn!("404 : {:?} does not exists", bad_path);
            Some("/errors/404")
        }
    }
}

// * Error handling
enum ErrorType {
    UnknownMethod(Method),
    FileNotFound(String),
    CantRespond,
}

fn handle_error(e: ErrorType) {
    match e {
        ErrorType::UnknownMethod(method) => {
            warn!("unknown method {}, do nothing", method);
        }
        ErrorType::FileNotFound(file) => {
            error!("file not found {}", file);
        }
        ErrorType::CantRespond => {
            error!("can't respond, unknown error");
        }
    }
}

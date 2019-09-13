use log::*;
use simple_logger;
use std::fs;
use std::sync::Arc;
use threadpool::ThreadPool;
use tiny_http::*;

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
        Method::Put => {
            info!("PUT request from {}", request.remote_addr());
            handle_method_put(request)
        }
        uk_method => Err(ErrorType::UnknownMethod(uk_method.clone())),
    }
}

// * Basic page and assets loading
fn handle_method_get(request: Request) -> Result<(), ErrorType> {
    let (path, code) = match match_path(request.url()) {
        Some(r) => r,
        None => return Ok(()),
    };
    let url = ["server", path].join("/");

    trace!(target: "disks", "Serving page {}", &url);
    let html_content = match fs::read_to_string(&url) {
        Ok(c) => c,
        Err(_) => return Err(ErrorType::FileNotFound(url)),
    };

    // * Create response
    let content_type_header = Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
    let response = Response::from_string(html_content)
        .with_header(content_type_header)
        .with_status_code(code);
    match request.respond(response) {
        Ok(()) => Ok(()),
        Err(_) => Err(ErrorType::CantRespond),
    }
}

// * Socket method : respond accordingly to that
fn handle_method_put(request: Request) -> Result<(), ErrorType> {
    if request.url() != "/socket/message" {
        return Err(ErrorType::BadPutRequest(request.url().to_string()));
    }

    // * Create response
    let content_type_header =
        Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
    let response =
        Response::from_string("{\"res\": \"THIS IS A MESSAGE\"}").with_header(content_type_header);
    match request.respond(response) {
        Ok(()) => Ok(()),
        Err(_) => Err(ErrorType::CantRespond),
    }
}

// * Path white_/black_listing
fn match_path(url: &str) -> Option<(&str, i32)> {
    match url {
        //? Authorized paths
        "/" => Some(("/index.html", 200)),
        //? Javascript paths
        "/js/socket.js" => Some(("js/socket.js", 200)),
        //? Blacklisted paths
        "/favicon.ico" => None,
        //? Other paths: 404 error
        bad_path => {
            warn!("404 : {:?} does not exists", bad_path);
            Some(("/errors/404/index.html", 404))
        }
    }
}

// * Error handling
enum ErrorType {
    UnknownMethod(Method),
    FileNotFound(String),
    BadPutRequest(String),
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
        ErrorType::BadPutRequest(request) => {
            error!("PUT request does not cover message {}", request);
        }
        ErrorType::CantRespond => {
            error!("can't respond, unknown error");
        }
    }
}

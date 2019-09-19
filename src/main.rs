use log::*;
use simplelog::*;
use std::fs;
use std::sync::Arc;
use threadpool::ThreadPool;
use tiny_http::*;

const MAX_THREADS: usize = 4;
const SERVER_ADDR: &str = "127.0.0.1:3000";

// ! MAIN

fn main() -> Result<(), i32> {
    // * Inits
    //logger
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            std::fs::File::create("server_run.log").unwrap(),
        ),
    ])
    .unwrap();

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
    println!("Finished");
    Ok(())
}

// ! SERVER

// * Handling the server: accept requests, handle them and wether respond or handle the error
fn handle_server(thread_server: Arc<Server>, _worker_id: usize) {
    for request in thread_server.incoming_requests() {
        //debug!("New work for worker {}", worker_id);
        match handle_request(request) {
            Ok(()) => continue,
            Err(e) => handle_error(e),
        };
    }
}

fn handle_request(request: Request) -> Result<(), ErrorType> {
    match request.method() {
        // TODO reject truly other requests (now: 500 server error)
        Method::Get => {
            info!(
                "GET request from {} : {:?}",
                request.remote_addr(),
                request.url()
            );
            handle_method_get(request)
        }
        Method::Put => {
            info!(
                target: "socket",
                "PUT request from {} : {:?}",
                request.remote_addr(),
                request.url()
            );
            handle_method_put(request)
        }
        uk_method => Err(ErrorType::UnknownMethod(uk_method.clone())),
    }
    // TODO catch errors and send back an adapted response ?
    // (althought 500-server-side error is great too)
}

// ! HTTP GET

// * Page and assets loading : GET method
fn handle_method_get(request: Request) -> Result<(), ErrorType> {
    let (path, code, mime_type) = match_path(request.url());
    let url = if mime_type == "text/html" {
        ["server", path, "index.html"].join("/")
    } else {
        ["server", path].join("/")
    };

    trace!(target: "disks", "Serving page {}", &url);

    // * Checking for 404 error
    let html_content = match fs::read_to_string(&url) {
        Ok(c) => c,
        Err(_) => {
            // If 404, try to open 404.html
            warn!(target: "client", "404 : {:?} does not exists", url);
            match fs::read_to_string("server/errors/404.html") {
                Ok(c) => c,
                Err(_) => return Err(ErrorType::FileNotFound("/errors/404.html".to_string())),
            }
        }
    };

    // * Create response
    // Mime type
    let content_type_header =
        Header::from_bytes(&b"Content-Type"[..], mime_type.as_bytes()).unwrap();
    // Content, headers and status code
    let response = Response::from_string(html_content)
        .with_header(content_type_header)
        .with_status_code(code);
    match request.respond(response) {
        Ok(()) => Ok(()),
        Err(_) => Err(ErrorType::CantRespond),
    }
}

// * Path white_/black_listing
fn match_path(url: &str) -> (&str, i32, &str) {
    let (mime_type, is_unknown) = match match_mime_type(url) {
        Ok(m) => (m, false),
        Err(e) => {
            handle_error(e);
            ("text/html", true)
        }
    };
    trace!("Mime type = {}", mime_type);

    if is_unknown {
        warn!(target: "client", "404 : {:?} does not exists", url);
        ("/errors/404.html", 404, mime_type)
    } else {
        (url, 200, mime_type)
    }
}

// * Match mime type
fn match_mime_type(url: &str) -> Result<&str, ErrorType> {
    // ! Permits debugging in browser
    if url.ends_with(".map") {
        return Ok("application/octet-stream");
    };
    let splitted_url: Vec<&str> = url.split('.').collect();
    match splitted_url.len() {
        1 => Ok("text/html"),
        2 => {
            match splitted_url[1] {
                // Web
                "html" => Ok("text/html"),
                "js" => Ok("text/javascript"),
                "css" => Ok("text/css"),
                // Images
                "apng" => Ok("image/apng"),
                "bmp" => Ok("image/bmp"),
                "gif" => Ok("image/gif"),
                "ico" | "cur" => Ok("image/x-icon"),
                "png" => Ok("image/png"),
                "jpeg" | "jpg" => Ok("image/jpeg"),
                "svg" => Ok("image/svg+xml"),
                "tiff" | "tif" => Ok("image/tiff"),
                "webp" => Ok("image/webp"),
                // Fonts
                "otf" => Ok("font/otf"),
                "sfnt" => Ok("font/sfnt"),
                "ttf" => Ok("font/ttf"),
                "woff" => Ok("font/woff"),
                "woff2" => Ok("font/woff2"),
                // Unknown
                m => Err(ErrorType::CantDecideMime(m.to_string())),
            }
        }
        _ => Err(ErrorType::InvalidUrlSeparator),
    }
}

// ! HTTP PUT

// * Socket method : respond accordingly to that
fn handle_method_put(mut request: Request) -> Result<(), ErrorType> {
    // * Verify that message is effectively our
    if request.url() != "/socket/message" {
        return Err(ErrorType::BadPutRequest(request.url().to_string()));
    }

    // * Get message name
    let mut message_name = String::new();
    // permits dropping temp values
    {
        let message_headers = request.headers().to_vec();
        // TODO implement another way to get name of message (too much cpu time)
        for header in message_headers {
            let header = format!("{}", header);
            let header: Vec<&str> = header.split(':').collect();
            let mut header = header.iter();
            let first_header = *header.next().unwrap();

            if "Message-Name" == first_header {
                message_name = header.next().unwrap().to_string();
            }
        }
    }
    if message_name.is_empty() {
        return Err(ErrorType::EmptyNameSocketMsg);
    }

    // * Get message content
    let mut content = String::new();
    match request.as_reader().read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => return Err(ErrorType::CannotReadSocketContent(message_name)),
    };
    let contains_content = !content.is_empty();

    // ! DEBUG print message name and content
    debug!(
        "Name of Socket request :\n{:?}\nContent :\n{}",
        message_name, content
    );

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

// ! ERRORS

// * Error handling
enum ErrorType {
    // Get
    UnknownMethod(Method),
    FileNotFound(String),
    CantDecideMime(String),
    InvalidUrlSeparator,
    CantRespond,
    // Socket
    BadPutRequest(String),
    CannotReadSocketContent(String),
    EmptyNameSocketMsg,
}

#[rustfmt::skip]
fn handle_error(e: ErrorType) {
    match e {
        ErrorType::UnknownMethod(method) =>         warn!("unknown method {:?}, do nothing", method),
        ErrorType::FileNotFound(file) =>            error!("file not found {:?}", file),
        ErrorType::CantDecideMime(mime) =>          warn!("Can't infer type for {:?}", mime),
        ErrorType::InvalidUrlSeparator =>           error!("invalid url request number of separators, 404 error"),
        ErrorType::CantRespond =>                   error!("can't respond, unknown error"),
        ErrorType::BadPutRequest(request) =>        error!("PUT request does not cover message {:?}", request),
        ErrorType::CannotReadSocketContent(name) => error!("Cannot read content of message {:?}", name),
        ErrorType::EmptyNameSocketMsg =>            warn!("Empty name for socket message"),
    }
}

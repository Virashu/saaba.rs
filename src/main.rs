use colog;
use saaba::{App, HTTPMethod, Response};

fn main() {
    colog::default_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let mut app = App::new();

    #[allow(unused_variables)]
    app.route("get", "/", |req| {
        let req_addr = req.headers.get("Host").unwrap();
        let url = req.path;

        let content = format!(
            "Hello, world!<br>\
            Client address: <code>{req_addr}</code><br>\
            Request URL: <code>{url}</code>"
        );
        Response::html(content)
    });

    #[allow(unused_variables)]
    app.route(HTTPMethod::GET, "/favicon.ico", |req| {
        let mut res = Response::file("__static/favicon.ico");
        res.set_header("Content-Type", "image/x-icon");

        res
    });

    app.run("0.0.0.0", 3333)
}

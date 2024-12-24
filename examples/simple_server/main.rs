use std::collections::HashMap;

use saaba::{App, HTTPMethod, Response};

const STATIC_DIR: &str = "./examples/simple_server/__static";

fn main() {
    colog::default_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let mut app = App::new();

    app.route("get", "/", |req| {
        let req_addr = req.headers.get("Host").unwrap();
        let url = req.url;

        let content = format!(
            "Hello, world!<br>\
            Client address: <code>{req_addr}</code><br>\
            Request URL: <code>{url}</code>"
        );

        Response::html(content)
    });

    app.get("/favicon.ico", |_| {
        let favicon = format!("{STATIC_DIR}/favicon.ico");
        Response::file(&favicon).with_header("Content-Type", "image/x-icon")
    });

    app.route_var(HTTPMethod::GET, "/var/{variable}", |_, variables: HashMap<&str, &str>| {
        let var = variables.get("variable").unwrap_or(&"not set");

        let content = format!(
            "Hello, world!<br>\
            Variable: <code>{var}</code><br>"
        );

        Response::html(content)
    });

    app.static_("/static", STATIC_DIR);

    app.run("0.0.0.0", 80).unwrap();
}

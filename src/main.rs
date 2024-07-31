use saaba::{App, HTTPMethod, Response};

fn main() {
    let mut app = App::new();

    #[allow(unused_variables)]
    app.route("get", "/", |req| {
        let req_addr = req.headers.get("Host").unwrap();
        let url = req.path;
        
        format!("Hello, world!<br>Client address: {req_addr}<br>Request URL: {url}").into()
    });

    #[allow(unused_variables)]
    app.route(HTTPMethod::GET, "/favicon.ico", |req| {
        let mut res = Response::file("__static/favicon.ico");
        res.set_header("Content-Type".into(), "image/x-icon".into());

        res
    });

    app.run("0.0.0.0", 3333)
}

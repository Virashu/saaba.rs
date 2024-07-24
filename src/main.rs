use saaba::{App, HTTPMethod};

fn main() {
    let mut app = App::new();

    app.route(HTTPMethod::GET, "/", |req| {
        return "Hello, world!".into()
    });

    app.run("0.0.0.0", 3333)
}

use std::collections::HashMap;

use regex::Regex;

pub fn parse_headers(headers_vec: Vec<String>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let header_re = Regex::new(r"(?<key>[a-zA-Z-_]+):\s?(?<value>.+)").unwrap();

    for line in headers_vec.into_iter() {
        let capture_opt = header_re.captures(line.as_str());

        if let Some(header) = capture_opt {
            headers.insert(header["key"].into(), header["value"].into());
        } else {
            log::error!("Failed to parse header: {}", line);
        }
    }

    headers
}

pub fn construct_message(message: String) -> String {
    format!(
        "<center>\
        <h1>{}</h1>\
        <hr>\
        <span>saaba</span>\
        </center>",
        message
    )
}

use std::collections::HashMap;

use regex::Regex;


pub fn parse_headers(headers_vec: Vec<String>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let header_re = Regex::new(r"(?<key>[a-zA-Z-_]+):\s?(?<value>.+)").unwrap();

    for line in headers_vec.into_iter() {
        let rm = header_re.captures(line.as_str()).unwrap();
        headers.insert(rm["key"].into(), rm["value"].into());
    }

    headers
}
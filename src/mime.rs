// We can
// 1. Guess by extension
// 2. Read file header

fn ext_to_mime(ext: &str) -> &str {
    match ext {
        // application
        "js" => "application/javascript",

        // text
        "html" => "text/html",
        "css" => "text/css",

        // image
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "jfif" => "image/jpeg",
        "png" => "image/png",
        "svg" => "image/svg",
        "webp" => "image/webp",

        // video

        _ => "",
    }
}

pub fn guess_mime(filename: &str) -> Option<String> {
    let parts = filename.split(".").collect::<Vec<&str>>();
    let ext = parts.last();

    if let Some(ext) = ext {
        let mime = ext_to_mime(ext);

        return match mime {
            "" => None,
            m => Some(m.to_string()),
        };
    }

    None
}

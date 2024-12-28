// We can
// 1. Guess by extension
// 2. Read file header

fn ext_to_mime(ext: &str) -> &str {
    match ext {
        // application
        "epub" => "application/epub+zip",
        "gz" => "application/gzip",
        "jar" => "application/java-archive",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "doc" => "application/msword",
        "bin" => "application/octet-stream",
        "ogx" => "application/ogg",
        "pdf" => "application/pdf",
        "rtf" => "application/rtf",
        "azw" => "application/vnd.amazon.ebook",
        "mpkg" => "application/vnd.apple.installer+xml",
        "xul" => "application/vnd.mozilla.xul+xml",
        "xls" => "application/vnd.ms-excel",
        "eot" => "application/vnd.ms-fontobject",
        "ppt" => "application/vnd.ms-powerpoint",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odt" => "application/vnd.oasis.opendocument.text",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "rar" => "application/vnd.rar",
        "vsd" => "application/vnd.visio",
        "7z" => "application/x-7z-compressed",
        "abw" => "application/x-abiword",
        "bz" => "application/x-bzip",
        "bz2" => "application/x-bzip2",
        "cda" => "application/x-cdf",
        "csh" => "application/x-csh",
        "arc" => "application/x-freearc",
        "php" => "application/x-httpd-php",
        "sh" => "application/x-sh",
        "tar" => "application/x-tar",
        "xhtml" => "application/xhtml+xml",
        "xml" => "application/xml",
        "zip" => "application/zip",

        // audio
        "aac" => "audio/aac",
        "mid" => "audio/midi",
        "midi" => "audio/midi",
        "mp3" => "audio/mpeg",
        "oga" => "audio/ogg",
        "opus" => "audio/ogg",
        "wav" => "audio/wav",
        "weba" => "audio/webm",

        // font
        "otf" => "font/otf",
        "ttf" => "font/ttf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",

        // image
        "apng" => "image/apng",
        "avif" => "image/avif",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "ico" => "image/vnd.microsoft.icon",
        "webp" => "image/webp",

        // text
        "ics" => "text/calendar",
        "css" => "text/css",
        "csv" => "text/csv",
        "htm" => "text/html",
        "html" => "text/html",
        "tif" => "text/html",
        "tiff" => "text/htmlts video/mp2t",
        "js" => "text/javascript",
        "mjs" => "text/javascript",
        "txt" => "text/plain",

        // video
        "3gp" => "video/3gpp",
        "3g2" => "video/3gpp2",
        "mp4" => "video/mp4",
        "mpeg" => "video/mpeg",
        "ogv" => "video/ogg",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",

        // _
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

// We can
// 1. Guess by extension
// 2. Read file header

fn ext_to_mime(ext: &str) -> &str {
    match ext {
        // application
        "7z" => "application/x-7z-compressed",
        "abw" => "application/x-abiword",
        "arc" => "application/x-freearc",
        "azw" => "application/vnd.amazon.ebook",
        "bin" => "application/octet-stream",
        "bz" => "application/x-bzip",
        "bz2" => "application/x-bzip2",
        "cda" => "application/x-cdf",
        "csh" => "application/x-csh",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "eot" => "application/vnd.ms-fontobject",
        "epub" => "application/epub+zip",
        "gz" => "application/gzip",
        "jar" => "application/java-archive",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "mpkg" => "application/vnd.apple.installer+xml",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odt" => "application/vnd.oasis.opendocument.text",
        "ogx" => "application/ogg",
        "pdf" => "application/pdf",
        "php" => "application/x-httpd-php",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "rar" => "application/vnd.rar",
        "rtf" => "application/rtf",
        "sh" => "application/x-sh",
        "tar" => "application/x-tar",
        "vsd" => "application/vnd.visio",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "application/xml",
        "xul" => "application/vnd.mozilla.xul+xml",
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
        "tif" => "image/tiff",
        "tiff" => "image/tiff",

        // text
        "ics" => "text/calendar",
        "css" => "text/css",
        "csv" => "text/csv",
        "htm" => "text/html",
        "html" => "text/html",
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

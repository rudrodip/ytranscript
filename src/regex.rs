/// Regular expression pattern for extracting YouTube video IDs from URLs.
pub const RE_YOUTUBE: &str =
    r#"(?:youtube\.com\/(?:[^\/]+\/.+\/|(?:v|e(?:mbed)?)\/|.*[?&]v=)|youtu\.be\/)([^"&?\/\s]{11})"#;

/// User-Agent string to be used for HTTP requests to YouTube.
pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36,gzip(gfe)";

/// Regular expression pattern for extracting text, start time, and duration from YouTube transcript XML.
pub const RE_XML_TRANSCRIPT: &str = r#"<text start="([^"]*)" dur="([^"]*)">([^<]*)<\/text>"#;

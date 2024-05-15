# ytranscript

`ytranscript` is a Rust crate that provides functionality to fetch YouTube video transcripts. It supports fetching transcripts in different languages and handles various error scenarios that might occur while retrieving the transcripts.

## Features

- Extracts YouTube video IDs from URLs or strings.
- Fetches transcripts for YouTube videos.
- Supports fetching transcripts in specific languages.
- Handles common errors such as video unavailability, transcript unavailability, and too many requests.

## Installation

Add `ytranscript` to your `Cargo.toml`:

## Usage

Here is an example of how to use the `ytranscript` crate in a binary crate:

```rust
use ytranscript::YoutubeTranscript;
use std::env;

#[tokio::main]
async fn main() {
    // Get the video ID from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ytranscript_bin <video_id>");
        return;
    }
    let video_id = &args[1];

    // Fetch the transcript
    match YoutubeTranscript::fetch_transcript(video_id, None).await {
        Ok(transcript) => {
            for entry in transcript {
                println!("{:?}", entry);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

### Functionality

#### `YoutubeTranscript::fetch_transcript`

Fetches the transcript for a given YouTube video ID or URL.

- **Arguments:**
  - `video_id`: A string slice representing the YouTube video URL or ID.
  - `config`: An optional `TranscriptConfig` specifying the desired language for the transcript.

- **Returns:**
  - `Ok(Vec<TranscriptResponse>)`: A vector of `TranscriptResponse` if the transcript is successfully fetched.
  - `Err(YoutubeTranscriptError)`: An error if the transcript cannot be fetched.

### Error Handling

The crate defines a set of errors that might occur while fetching transcripts:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YoutubeTranscriptError {
    #[error("YouTube is receiving too many requests from this IP and now requires solving a captcha to continue")]
    TooManyRequests,
    #[error("The video is no longer available ({0})")]
    VideoUnavailable(String),
    #[error("Transcript is disabled on this video ({0})")]
    TranscriptDisabled(String),
    #[error("No transcripts are available for this video ({0})")]
    TranscriptNotAvailable(String),
    #[error("No transcripts are available in {0} for this video ({2}). Available languages: {1:?}")]
    TranscriptNotAvailableLanguage(String, Vec<String>, String),
    #[error("Impossible to retrieve Youtube video ID.")]
    InvalidVideoId,
}
```

### Regex Patterns

The crate uses regex patterns to extract YouTube video IDs and parse XML transcripts:

```rust
pub const RE_YOUTUBE: &str =
    r#"(?:youtube\.com\/(?:[^\/]+\/.+\/|(?:v|e(?:mbed)?)\/|.*[?&]v=)|youtu\.be\/)([^"&?\/\s]{11})"#;

pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36,gzip(gfe)";

pub const RE_XML_TRANSCRIPT: &str = r#"<text start="([^"]*)" dur="([^"]*)">([^<]*)<\/text>"#;
```

### Types

The crate defines the following types:

```rust
#[derive(Debug)]
pub struct TranscriptConfig {
    pub lang: Option<String>,
}

#[derive(Debug)]
pub struct TranscriptResponse {
    pub text: String,
    pub duration: f64,
    pub offset: f64,
    pub lang: String,
}
```

### Testing

You can test the functionality of the `ytranscript` crate by running the following command:

```sh
cargo test
```

### License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details

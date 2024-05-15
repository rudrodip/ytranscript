/// Configuration options for fetching transcripts.
#[derive(Debug)]
pub struct TranscriptConfig {
    /// The language code for the desired transcript (optional).
    pub lang: Option<String>,
}

/// A struct representing a single entry in a YouTube transcript.
#[derive(Debug)]
pub struct TranscriptResponse {
    /// The text of the transcript entry.
    pub text: String,
    /// The duration (in seconds) for which the text is displayed.
    pub duration: f64,
    /// The offset (in seconds) from the start of the video when the text is displayed.
    pub offset: f64,
    /// The language code of the transcript entry.
    pub lang: String,
}

use thiserror::Error;

/// An enumeration of possible errors that can occur when fetching YouTube transcripts.
#[derive(Error, Debug)]
pub enum YoutubeTranscriptError {
    /// Error indicating that YouTube is receiving too many requests from the IP address and requires solving a captcha.
    #[error("YouTube is receiving too many requests from this IP and now requires solving a captcha to continue")]
    TooManyRequests,

    /// Error indicating that the video is no longer available.
    #[error("The video is no longer available ({0})")]
    VideoUnavailable(String),

    /// Error indicating that transcripts are disabled for the video.
    #[error("Transcript is disabled on this video ({0})")]
    TranscriptDisabled(String),

    /// Error indicating that no transcripts are available for the video.
    #[error("No transcripts are available for this video ({0})")]
    TranscriptNotAvailable(String),

    /// Error indicating that no transcripts are available in the requested language.
    #[error(
        "No transcripts are available in {0} for this video ({2}). Available languages: {1:?}"
    )]
    TranscriptNotAvailableLanguage(String, Vec<String>, String),

    /// Error indicating that it was impossible to retrieve the YouTube video ID.
    #[error("Impossible to retrieve Youtube video ID.")]
    InvalidVideoId,
}

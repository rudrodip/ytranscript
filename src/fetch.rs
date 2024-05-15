use crate::errors::*;
use crate::regex::*;
use crate::types::*;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

const USER_AGENT_STR: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36,gzip(gfe)";

/// A struct providing functionality to fetch YouTube transcripts.
pub struct YoutubeTranscript;

impl YoutubeTranscript {
    /// Fetches the transcript for a given YouTube video ID or URL.
    ///
    /// # Arguments
    ///
    /// * `video_id` - A string slice representing the YouTube video URL or ID.
    /// * `config` - An optional `TranscriptConfig` specifying the desired language for the transcript.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<TranscriptResponse>)` - A vector of `TranscriptResponse` if the transcript is successfully fetched.
    /// * `Err(YoutubeTranscriptError)` - An error if the transcript cannot be fetched.
    pub async fn fetch_transcript(
        video_id: &str,
        config: Option<TranscriptConfig>,
    ) -> Result<Vec<TranscriptResponse>, YoutubeTranscriptError> {
        // Step 1: Retrieve video identifier from URL or ID
        let identifier = Self::retrieve_video_id(video_id)?;

        // Step 2: Create HTTP client
        let client = Client::new();

        // Step 3: Construct video page URL
        let video_page_url = format!("https://www.youtube.com/watch?v={}", identifier);

        // Step 4: Prepare headers for the request
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT_STR));
        if let Some(config) = &config {
            if let Some(lang) = &config.lang {
                headers.insert("Accept-Language", HeaderValue::from_str(&lang).unwrap());
            }
        }

        // Step 5: Fetch the video page content
        let video_page_response = client
            .get(&video_page_url)
            .headers(headers.clone())
            .send()
            .await
            .map_err(|_| YoutubeTranscriptError::TranscriptDisabled(video_id.to_string()))?;

        let video_page_body = video_page_response
            .text()
            .await
            .map_err(|_| YoutubeTranscriptError::TranscriptDisabled(video_id.to_string()))?;

        // Step 6: Split the HTML content to find the captions section
        let splitted_html: Vec<&str> = video_page_body.split("\"captions\":").collect();

        // Step 7: Handle cases where captions are not found
        if splitted_html.len() <= 1 {
            if video_page_body.contains("class=\"g-recaptcha\"") {
                return Err(YoutubeTranscriptError::TooManyRequests);
            }
            if !video_page_body.contains("\"playabilityStatus\":") {
                return Err(YoutubeTranscriptError::VideoUnavailable(video_id.to_string()));
            }
            return Err(YoutubeTranscriptError::TranscriptDisabled(video_id.to_string()));
        }

        // Step 8: Parse the captions JSON
        let captions: Option<serde_json::Value> = serde_json::from_str(
            &splitted_html[1].split(",\"videoDetails").collect::<Vec<&str>>()[0].replace("\n", ""),
        )
        .ok();

        // Step 9: Extract player captions renderer
        let player_captions_renderer = captions
            .as_ref()
            .and_then(|c| c.get("playerCaptionsTracklistRenderer"));

        if player_captions_renderer.is_none() {
            return Err(YoutubeTranscriptError::TranscriptDisabled(video_id.to_string()));
        }

        // Step 10: Extract caption tracks
        let caption_tracks = player_captions_renderer
            .unwrap()
            .get("captionTracks")
            .ok_or(YoutubeTranscriptError::TranscriptNotAvailable(video_id.to_string()))?;

        let caption_tracks =
            caption_tracks.as_array().ok_or(YoutubeTranscriptError::TranscriptNotAvailable(video_id.to_string()))?;

        // Step 11: Check for specific language availability if provided in config
        if let Some(lang) = config.as_ref().and_then(|c| c.lang.clone()) {
            let lang_clone = lang.clone();
            if !caption_tracks.iter().any(|track| {
                track.get("languageCode").and_then(|v| v.as_str()) == Some(&lang_clone)
            }) {
                let available_langs = caption_tracks
                    .iter()
                    .filter_map(|track| {
                        track.get("languageCode").and_then(|lc| lc.as_str().map(String::from))
                    })
                    .collect();
                return Err(YoutubeTranscriptError::TranscriptNotAvailableLanguage(
                    lang,
                    available_langs,
                    video_id.to_string(),
                ));
            }
        }

        // Step 12: Retrieve the transcript URL
        let transcript_url = caption_tracks
            .iter()
            .find(|track| {
                config.as_ref().map_or(true, |c| {
                    track.get("languageCode") == Some(&c.lang.clone().unwrap().into())
                })
            })
            .and_then(|track| track.get("baseUrl"))
            .and_then(|url| url.as_str())
            .ok_or(YoutubeTranscriptError::TranscriptNotAvailable(video_id.to_string()))?;

        // Step 13: Fetch the transcript content
        let transcript_response = client
            .get(transcript_url)
            .headers(headers)
            .send()
            .await
            .map_err(|_| YoutubeTranscriptError::TranscriptNotAvailable(video_id.to_string()))?;

        if !transcript_response.status().is_success() {
            return Err(YoutubeTranscriptError::TranscriptNotAvailable(video_id.to_string()));
        }

        let transcript_body = transcript_response
            .text()
            .await
            .map_err(|_| YoutubeTranscriptError::TranscriptNotAvailable(video_id.to_string()))?;

        // Step 14: Parse the XML transcript
        let re_xml_transcript = Regex::new(RE_XML_TRANSCRIPT).unwrap();
        let results: Vec<TranscriptResponse> = re_xml_transcript
            .captures_iter(&transcript_body)
            .map(|cap| TranscriptResponse {
                text: cap[3].to_string(),
                duration: cap[2].parse().unwrap_or(0.0),
                offset: cap[1].parse().unwrap_or(0.0),
                lang: config
                    .as_ref()
                    .and_then(|c| c.lang.clone())
                    .unwrap_or_else(|| {
                        caption_tracks[0]["languageCode"]
                            .as_str()
                            .unwrap()
                            .to_string()
                    }),
            })
            .collect();

        Ok(results)
    }

    /// Retrieves the video ID from a given YouTube URL or string.
    ///
    /// # Arguments
    ///
    /// * `video_id` - A string slice representing the YouTube video URL or ID.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The video ID if successfully retrieved.
    /// * `Err(YoutubeTranscriptError)` - An error if the video ID cannot be retrieved.
    fn retrieve_video_id(video_id: &str) -> Result<String, YoutubeTranscriptError> {
        if video_id.len() == 11 {
            return Ok(video_id.to_string());
        }
        let re_youtube = Regex::new(RE_YOUTUBE).unwrap();
        if let Some(caps) = re_youtube.captures(video_id) {
            if let Some(matched) = caps.get(1) {
                return Ok(matched.as_str().to_string());
            }
        }
        Err(YoutubeTranscriptError::InvalidVideoId)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieve_video_id_from_url() {
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = YoutubeTranscript::retrieve_video_id(url);
        assert_eq!(result.unwrap(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_retrieve_video_id_from_invalid_url() {
        let url = "https://www.example.com/watch?v=dQw4w9WgXcQ";
        let result = YoutubeTranscript::retrieve_video_id(url);
        assert!(result.is_err());
    }
}

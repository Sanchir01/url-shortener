use regex::Regex;
use std::env;
use teloxide::types::Message;
use url::Url;
pub fn extract_first_valid_url_from_message(msg: &Message) -> Option<String> {
    let text = msg.text()?;

    let re = Regex::new(r#"((https?://)?[a-zA-Z0-9.-]+\.[a-zA-Z0-9]{2,}(/\S*)?)"#).unwrap();

    for caps in re.captures_iter(text) {
        let url_candidate = caps.get(1)?.as_str();

        let fixed_url =
            if url_candidate.starts_with("http://") || url_candidate.starts_with("https://") {
                url_candidate.to_string()
            } else {
                format!("http://{}", url_candidate)
            };

        // Проверяем, можно ли распарсить как URL
        if Url::parse(&fixed_url).is_ok() {
            return Some(fixed_url);
        }
    }

    None
}

pub fn generate_google_oauth_url() -> Result<String, String> {
    let client_id = env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| "GOOGLE_CLIENT_ID environment variable not found".to_string())?;

    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
        .map_err(|e| format!("Failed to parse URL: {}", e))?;

    let google_uri = env::var("GOOGLE_URI_REDIRECT")
        .map_err(|_| "GOOGLE_URI_REDIRECT environment variable not found".to_string())?;
    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &google_uri)
        .append_pair("scope", "openid email profile")
        .append_pair("access_type", "offline");

    Ok(url.to_string())
}

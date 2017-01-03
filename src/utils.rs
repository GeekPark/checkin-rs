
pub fn url_decode(s: &str) -> String {
    use rocket::http::uri::URI;
    URI::percent_decode_lossy(s.as_bytes()).into_owned()
}


pub fn url_decode(s: &str) -> String {
    use rocket::http::uri::URI;
    URI::percent_decode_lossy(s.as_bytes()).into_owned()
}

pub fn sequence<T>(xss: &Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: Sized + Clone
{
    if xss.is_empty() {
        return Vec::new();
    }

    let (xs, xss_) = xss.split_first().unwrap();
    let rest: Vec<Vec<T>> = sequence(&xss_.to_vec());
    let mut result = Vec::new();

    for x in xs.iter() {
        for xs in rest.iter() {
            let mut v = Vec::new();
            v.push(x.clone());
            v.append(&mut xs.clone());
            result.push(v)
        }
    }
    result
}

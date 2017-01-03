use rocket::Route;
use rocket_contrib::JSON;

use model::*;
use utils;

#[derive(Serialize)]
struct UserResult {
    id: String,
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    note: String,
    checked_at: Option<String>,
}

#[derive(Serialize)]
struct SearchResult {
    ok: bool,
    results: Vec<UserResult>,
}

fn convert_user(u: User) -> UserResult {
    use time;
    let checked_at = u.checked_at
        .map(time::at)
        .map(|tm| time::strftime("%Y-%m-%d %H:%M:%S", &tm).unwrap());
    UserResult {
        id: u.id,
        name: u.name,
        phone: u.phone,
        company: u.company,
        position: u.position,
        email: u.email,
        note: u.note,
        checked_at: checked_at,
    }
}

#[get("/search/<keyword>")]
fn search(db: DBI, keyword: &str) -> JSON<SearchResult> {
    let db = &db.0;
    let keyword = utils::url_decode(keyword);
    let results = User::search_all_fields(db, &keyword);
    JSON(SearchResult {
        ok: true,
        results: results.into_iter().map(convert_user).collect(),
    })
}


pub fn routes() -> Vec<Route> {
    routes![search]
}

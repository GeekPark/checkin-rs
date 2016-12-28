use rocket::Route;
use rocket_contrib::JSON;

#[get("/search/<keyword>")]
fn search(keyword: &str) -> JSON<&'static str> {
    JSON("yooo")
}


pub fn routes() -> Vec<Route> {
    routes![search]
}

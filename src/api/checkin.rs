use std::result::Result;

use rocket::Route;
use rocket_contrib::JSON;
use serde::ser::{Serialize, Serializer};

type Message = String;
type TicketInfo = String;
struct CheckinResult(Result<(Message, TicketInfo), String>);

impl Serialize for CheckinResult {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let &CheckinResult(ref res) = self;
        let mut state = try!(serializer.serialize_map(None));
        try!(serializer.serialize_map_key(&mut state, "ok"));
        try!(serializer.serialize_map_value(&mut state, res.is_ok()));

        match res {
            &Ok(ref ok) => {
                let &(ref msg, ref tkt) = ok;
                try!(serializer.serialize_map_key(&mut state, "result"));
                try!(serializer.serialize_map_value(&mut state, &msg));
                try!(serializer.serialize_map_key(&mut state, "ticket"));
                try!(serializer.serialize_map_value(&mut state, &tkt));
            }
            &Err(ref e) => {
                try!(serializer.serialize_map_key(&mut state, "error"));
                try!(serializer.serialize_map_value(&mut state, &e));
            }
        }

        serializer.serialize_map_end(state)
    }
}

#[get("/checkin/code/<code>")]
fn checkin_code(code: &str) -> JSON<CheckinResult> {
    JSON(CheckinResult(Ok(("贏了！".into(), "票！".into()))))
}


pub fn routes() -> Vec<Route> {
    routes![checkin_code]
}

use std::result::Result;

use rocket::Route;
use rocket_contrib::JSON;
use serde::ser::{Serialize, Serializer};

use model::*;

type Message = String;

#[derive(Serialize)]
struct TicketInfo {
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    ticket_cat: String,
    price: f64,
    checked_at: String,
}
struct CheckinResult(Result<TicketInfo, String>);

impl Serialize for CheckinResult {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let &CheckinResult(ref res) = self;
        let mut state = try!(serializer.serialize_map(None));
        try!(serializer.serialize_map_key(&mut state, "ok"));
        try!(serializer.serialize_map_value(&mut state, res.is_ok()));

        match res {
            &Ok(ref tkt) => {
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

impl TicketInfo {
    fn from(ticket: &Ticket, user: &User, tc: &TicketCat) -> Self {
        use time;
        let checked_at = user.checked_at
            .map(time::at)
            .map(|tm| time::strftime("%Y-%m-%d %H:%M:%S", &tm).unwrap())
            .unwrap_or("".into());
        TicketInfo {
            name: user.name.clone(),
            phone: user.phone.clone(),
            company: user.company.clone(),
            position: user.position.clone(),
            email: user.email.clone(),
            ticket_cat: tc.name.clone(),
            price: ticket.price,
            checked_at: checked_at,
        }
    }
}

macro_rules! try_err {
    [ $maybe:expr, $err:expr ] => {
        match $maybe {
            Some(sth) => sth,
            None => {
                return JSON(CheckinResult(Err($err.into())))
            }
        }
    }
}

#[get("/checkin/code/<code>")]
fn checkin_code(code: &str, db: DBI) -> JSON<CheckinResult> {
    let db = &db.0;
    let ticket = try_err!(Ticket::find_by_qrcode(db, code), "找不到指定票号");
    let mut user = try_err!(ticket.user(db), "该票无关联用戶");
    let tc = try_err!(ticket.ticket_cat(db), "找不關聯");
    try_err!(tc.guard_valid_day(), "该票不可用于今天的大会");
    try_err!(user.check_in(db), "不可以重复签到");
    let ticket_info = TicketInfo::from(&ticket, &user, &tc);
    JSON(CheckinResult(Ok(ticket_info)))
}

pub fn routes() -> Vec<Route> {
    routes![checkin_code]
}

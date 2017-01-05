use rocket::Route;
use rocket_contrib::JSON;
use serde::ser::{Serialize, Serializer};

use model::*;
use utils;

struct CheckoutResult<T>(Result<T, String>);

impl<T> Serialize for CheckoutResult<T>
    where T: Serialize
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let &CheckoutResult(ref res) = self;
        let mut state = try!(serializer.serialize_map(None));
        try!(serializer.serialize_map_key(&mut state, "ok"));
        try!(serializer.serialize_map_value(&mut state, res.is_ok()));

        match res {
            &Ok(ref tkt) => {
                try!(serializer.serialize_map_key(&mut state, "result"));
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


macro_rules! try_err {
    [ $maybe:expr, $err:expr ] => {
        match $maybe {
            Some(sth) => sth,
            None => {
                return JSON(CheckoutResult(Err($err.into())))
            }
        }
    }
}


#[get("/gift/for/<uid>/checked")]
fn list_gifts_checked(uid: &str, db: DBI) -> JSON<CheckoutResult<Vec<String>>> {
    let db = &db.0;
    try_err!(User::find_by_id(db, uid), "该用户不存在");
    let gifts = Gift::checked_for_user(db, uid);
    JSON(CheckoutResult(Ok(gifts)))
}

#[get("/gift/for/<uid>/available")]
fn list_gifts_available(uid: &str, db: DBI) -> JSON<CheckoutResult<Vec<String>>> {
    let db = &db.0;
    try_err!(User::find_by_id(db, uid), "该用户不存在");
    let gifts = Gift::available_for_user(db, uid);
    JSON(CheckoutResult(Ok(gifts)))
}
#[get("/gift/checkout/<uid>/<gift>")]
fn checkout_gift(uid: &str, gift: &str, db: DBI) -> JSON<CheckoutResult<i32>> {
    let db = &db.0;
    try_err!(User::find_by_id(db, uid), "该用户不存在");
    try_err!(Gift::checkout_for(db, uid, &utils::url_decode(gift)),
             "用户已领取该礼物");
    JSON(CheckoutResult(Ok(0)))
}

#[get("/gift/uncheck/<uid>/<gift>")]
fn uncheck_gift(uid: &str, gift: &str, db: DBI) -> JSON<CheckoutResult<i32>> {
    let db = &db.0;
    try_err!(User::find_by_id(db, uid), "该用户不存在");
    try_err!(Gift::uncheck_for(db, uid, gift),
             "用户尚未领取该礼物");
    JSON(CheckoutResult(Ok(0)))
}

pub fn routes() -> Vec<Route> {
    routes![list_gifts_checked, list_gifts_available, checkout_gift, uncheck_gift]
}

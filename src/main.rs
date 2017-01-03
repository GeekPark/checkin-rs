#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![feature(custom_attribute)]
#![feature(proc_macro)]
#![plugin(rocket_codegen)]
// For [rustfmt_skip]
#![allow(unused_attributes)]


#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate csv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
extern crate docopt;

pub use rocket_contrib::JSON;

mod api;
mod admin;
mod model;
mod cli;
mod utils;

fn init() {
    use model::*;
    let db = DB::connect();

    User::create_table(&db);
    Ticket::create_table(&db);
    TicketCat::create_table(&db);
    Gift::create_table(&db);

    TicketCat::seed(&db);
}

fn server() {
    rocket::ignite().mount("/api", api::routes()).mount("/admin", admin::routes()).launch();
}

fn main() {
    use cli::{parse_argv, CLIAction};
    match parse_argv() {
        Some(CLIAction::Init) => init(),
        Some(CLIAction::Server) => server(),
        None => panic!("Unable to parse cli argument"),
    }
}

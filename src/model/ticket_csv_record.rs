use std::io::Read;

use csv;

use model::*;

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct TicketCSVRecord {
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    qrcode: String,
    ticket_cat: String,
    price: Option<f64>,
    ticket_cat_id: Option<String>,
    note: Option<String>,
    status: String,
}

impl TicketCSVRecord {
    pub fn import<T>(csv: T, db: &DB) -> Option<usize>
        where T: Read
    {
        let mut rdr = csv::Reader::from_reader(csv);
        let mut count = 0;
        for record in rdr.decode() {
            let r: TicketCSVRecord = record.unwrap();
            r.create_ticket(db);
            count += 1;
        }
        Some(count)
    }

    pub fn extract_user(&self) -> User {
        User::new(&self.name,
                  &self.phone,
                  &self.company,
                  &self.position,
                  &self.email,
                  "")
    }

    pub fn find_user(&self, db: &DB) -> Option<User> {
        User::find_by_phone(db, &self.phone)
    }

    pub fn find_ticket_cat(&self, db: &DB) -> Option<TicketCat> {
        if let Some(ref tc_id) = self.ticket_cat_id {
            TicketCat::find_by_id(db, &tc_id)
        } else {
            Some(TicketCat::media_ticket(db))
        }
    }

    pub fn find_or_create_user(&self, db: &DB) -> Option<User> {
        self.find_user(db).or_else(|| {
            db.insert(&self.extract_user());
            self.find_user(db)
        })
    }

    pub fn create_ticket(&self, db: &DB) {
        let u = self.find_or_create_user(db).unwrap();
        let tc = self.find_ticket_cat(db).unwrap();
        let ticket = Ticket::new(&tc.id, &u.id, &self.qrcode, self.price.unwrap_or(0f64));
        println!("importing ticket: {:?}", ticket);
        db.insert(&ticket);
    }
}

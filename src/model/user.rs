use model::*;

pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub company: String,
    pub position: String,
    pub email: String,
    pub note: String,
    pub checked_at: Option<Timespec>,
}


impl Record for User {
    fn static_table_name() -> &'static str {
        "users"
    }
    fn static_fields() -> &'static [&'static str] {
        static FIELDS: &'static [&'static str] =
            &["id", "name", "phone", "company", "position", "email", "note", "checked_at"];
        FIELDS
    }
    fn values<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![&self.id,
             &self.name,
             &self.phone,
             &self.company,
             &self.position,
             &self.email,
             &self.note,
             &self.checked_at]
    }
    fn from_row(row: &sql::Row) -> Self {
        User {
            id: row.get(0),
            name: row.get(1),
            phone: row.get(2),
            company: row.get(3),
            position: row.get(4),
            email: row.get(5),
            note: row.get(6),
            checked_at: row.get(7),
        }
    }
}

impl User {
    pub fn create_table(db: &DB) {
        db.create_table("users",
                        "id              VARCHAR PRIMARY KEY,
                         name            VARCHAR NOT NULL,
                         phone           VARCHAR NOT NULL,
                         company         VARCHAR,
                         position        VARCHAR,
                         email           VARCHAR NOT NULL,
                         note            VARCHAR,
                         checked_at      INTEGER");
    }

    #[rustfmt_skip]
    pub fn new(name: &str, phone: &str, company: &str,
               position: &str, email: &str, note: &str) -> Self {
        User {
            id: Uuid::new_v4().hyphenated().to_string(),
            name: name.into(),
            phone: phone.into(),
            company: company.into(),
            position: position.into(),
            email: email.into(),
            note: note.into(),
            checked_at: None,
        }
    }

    pub fn find_by_phone(db: &DB, phone: &str) -> Option<Self> {
        db.search_one("SELECT * FROM users WHERE phone = ?", &[&phone])
    }
    pub fn find_by_id(db: &DB, id: &str) -> Option<Self> {
        db.search_one("SELECT * FROM users WHERE id = ?", &[&id])
    }

    fn already_checked_in(&self) -> bool {
        self.checked_at.is_some()
    }
    fn update_column(&self, db: &DB, field: &str, val: &ToSql) {
        db.update("users",
                  "id = ?",
                  &[&self.id],
                  &format!("{} = ?", field),
                  &[val])
    }
    pub fn check_in(&mut self, db: &DB) -> Option<()> {
        if self.already_checked_in() {
            None
        } else {
            use time::get_time;
            self.update_column(db, "checked_at", &get_time());
            self.reload(db)
        }
    }

    fn reload(&mut self, db: &DB) -> Option<()> {
        Self::find_by_id(db, &self.id).map(|user| {
            *self = user;
            ()
        })
    }
}

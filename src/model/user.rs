use model::*;

pub struct User {
    id: String,
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    note: String,
    checked_at: Option<Timespec>,
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
    fn new(name: &str, phone: &str, company: &str,
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
}

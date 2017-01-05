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
    #[rustfmt_skip]
    pub fn create_table(db: &DB) {
        db.create_table("users",
                        "id              VARCHAR PRIMARY KEY,
                         name            VARCHAR NOT NULL,
                         phone           VARCHAR NOT NULL,
                         company         VARCHAR,
                         position        VARCHAR,
                         email           VARCHAR NOT NULL,
                         note            VARCHAR,
                         checked_at      INTEGER")
          .unwrap();
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

    pub fn find_by_phone_and_company(db: &DB, phone: &str, company: &str) -> Option<Self> {
        db.search_one("SELECT * FROM users WHERE phone = ? AND company = ?",
                      &[&phone, &company])
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
        self.reload(db);
        if self.already_checked_in() {
            None
        } else {
            use time::get_time;
            self.update_column(db, "checked_at", &get_time());
            self.reload(db);
            Some(())
        }
    }

    fn reload(&mut self, db: &DB) -> Option<()> {
        Self::find_by_id(db, &self.id).map(|user| {
            *self = user;
            ()
        })
    }
    pub fn tickets(&self, db: &DB) -> Vec<Ticket> {
        db.search("SELECT * FROM tickets WHERE user_id = ?", &[&self.id])
    }
    #[rustfmt_skip]
    pub fn ticket_cats(&self, db: &DB) -> Vec<TicketCat> {
        db.search("SELECT tc.* \
                   FROM ticket_cats AS tc \
                   INNER JOIN tickets AS t ON t.ticket_cat_id = tc.id \
                   INNER JOIN users AS u ON t.user_id = u.id \
                   WHERE u.id = ?",
                  &[&self.id])
    }

    pub fn search_all_fields(db: &DB, keyword: &str) -> Vec<User> {
        let k: &ToSql = &format!("%{}%", keyword);
        let params = &[k, k, k, k, k, k];
        db.search("SELECT u.* \
                   FROM users AS u \
                   WHERE u.name     LIKE ? \
                   OR    u.phone    LIKE ? \
                   OR    u.company  LIKE ? \
                   OR    u.position LIKE ? \
                   OR    u.email    LIKE ? \
                   OR    u.note     LIKE ? \
                   LIMIT 20",
                  params)
    }

    pub fn uncheck(&self, db: &DB) -> Option<i32> {
        if self.already_checked_in() {
            self.update_column(db, "checked_at", &None::<Option<i32>>);
            Some(0)
        } else {
            None
        }
    }
}

use super::schema::*;

#[derive(Queryable)]
#[allow(dead_code)]
pub struct Activity {
    id: i32,
    name: String,
}

#[derive(Insertable, Debug)]
#[table_name="activities"]
pub struct NewActivity<'a> {
    pub name: &'a str,
}

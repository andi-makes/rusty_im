use super::schema::manufacturers;

#[derive(Queryable)]
pub struct Manufacturer {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "manufacturers"]
pub struct NewManufacturer {
    pub name: String,
}

impl std::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

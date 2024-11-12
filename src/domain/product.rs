pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Product {
    pub fn new(id: String, name: String, description: String) -> Self {
        Product { id, name, description }
    }
}

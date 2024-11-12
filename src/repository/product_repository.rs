use crate::domain::product::Product;

pub trait ProductRepository {
    fn create_product(&self, name: String, email: String) -> Result<u64, String>;
    fn get_product_by_id(&self, id: String) -> Result<Product, String>; 
}

// src/service/product_service.rs

use crate::domain::product::Product;

pub trait ProductService {
    fn create_product(&self, name: String, email: String, id: String) -> Result<u64, String>;
    fn get_product_by_id(&self, id: String) -> Result<Product, String>; 
}

use crate::repository::product_repository::ProductRepository;
use crate::domain::product::Product;

pub struct ProductRepositoryImpl;

impl ProductRepository for ProductRepositoryImpl {
    fn create_product(&self, name: String, description: String) -> Result<u64, String> { 
        println!("Creating product: {} with description {}", name, description);
        Ok(1)
    }

    fn get_product_by_id(&self, id: String) -> Result<Product, String> { 
        Ok(Product::new(id, "Biosan Strip".to_string(), "product by Biosan".to_string()))
    } 
}

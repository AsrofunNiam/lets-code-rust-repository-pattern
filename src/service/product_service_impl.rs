use crate::service::product_service::ProductService;
use crate::repository::product_repository::ProductRepository;
use crate::domain::product::Product;

pub struct ProductServiceImpl<R: ProductRepository> {
    product_repository: R,
}

impl<R: ProductRepository> ProductServiceImpl<R> {
    pub fn new(product_repository: R) -> Self {
        ProductServiceImpl { product_repository }
    }
}

impl<R: ProductRepository> ProductService for ProductServiceImpl<R> {
    fn create_product(&self, name: String, email: String, _id: String) -> Result<u64, String> {
        self.product_repository.create_product(name, email)
    }

    fn get_product_by_id(&self, id: String) -> Result<Product, String> {
        self.product_repository.get_product_by_id(id)
    }
}

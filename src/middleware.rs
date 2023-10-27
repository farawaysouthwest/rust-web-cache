use crate::controller::cache::Cache;

pub struct CacheMiddleware {
    pub cache: *Cache,
}

impl CacheMiddleware {
    pub fn new(cache: *Cache) -> Self {
        Self { cache }
    }
}
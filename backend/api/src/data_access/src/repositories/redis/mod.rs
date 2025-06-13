use lazy_static::lazy_static;

mod snap_repo;
pub use snap_repo::RedisSnapRepo;

lazy_static! {
    pub static ref REDIS_URL: String = cfg::var("cache.redis_url");
}

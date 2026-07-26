use crate::cache::format::CacheData;
use crate::error::Result;

pub struct CacheMigrator;

impl CacheMigrator {
    pub fn migrate(data: &mut CacheData) -> Result<()> {
        if data.version == 0 {
            data.version = 1;
        }
        Ok(())
    }
}

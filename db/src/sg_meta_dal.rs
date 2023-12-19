use anyhow::Error;
use dpn_types::sg_meta::SgMeta;
use ethers::types::U256;
use model::storage_sg_meta::StorageSgMeta;

use crate::{model, StorageProcessor};

const SG_META_ID: i32 = 1;

#[derive(Debug)]
pub struct SgMetaDal<'a, 'c> {
    pub(crate) storage: &'a mut StorageProcessor<'c>,
}

impl SgMetaDal<'_, '_> {
    pub async fn get_meta(&mut self) -> Result<SgMeta, Error> {
        match sqlx::query_as!(
            StorageSgMeta,
            r#"
                select * 
                from sg_meta 
                where id = $1
            "#,
            SG_META_ID
        )
        .fetch_one(self.storage.conn())
        .await
        {
            Ok(s) => Ok(SgMeta::from(s)),
            Err(e) => Err(Error::msg(format!("{:?}", e))),
        }
    }

    pub async fn set_head(&mut self, head: U256) -> Result<(), Error> {
        _ = sqlx::query!(
            r#"
                update sg_meta
                set c_head = $1
                where id = $2
            "#,
            head.as_u128() as i64,
            SG_META_ID
        )
        .execute(self.storage.conn())
        .await;
        Ok(())
    }

    pub async fn set_last_block_range(&mut self, from: U256, to: U256) -> Result<(), Error> {
        _ = sqlx::query!(
            r#"
                update sg_meta
                set b_from = $1, b_to = $2
                where id = $3
            "#,
            from.as_u128() as i64,
            to.as_u128() as i64,
            SG_META_ID
        )
        .execute(self.storage.conn())
        .await;
        Ok(())
    }
}

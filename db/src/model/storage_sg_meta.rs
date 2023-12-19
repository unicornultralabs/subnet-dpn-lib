use dpn_types::sg_meta::SgMeta;

#[derive(Debug, sqlx::FromRow)]
pub struct StorageSgMeta {
    pub id: i32,
    pub c_head: i64,
    pub b_from: i64,
    pub b_to: i64,
}

impl From<StorageSgMeta> for SgMeta {
    fn from(model: StorageSgMeta) -> Self {
        SgMeta {
            id: model.id as u32,
            c_head: model.c_head as u64,
            b_from: model.b_from as u64,
            b_to: model.b_to as u64,
        }
    }
}

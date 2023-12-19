use bandwidth_price_dal::BandWidthPriceDal;
use internal_tx_dal::InternalTxDal;
use referral_dal::ReferralDal;
use session_dal::SessionDal;
use sg_meta_dal::SgMetaDal;
use sqlx::{pool::PoolConnection, Connection, PgConnection, Postgres, Transaction};
use tier_dal::TierDal;
use tx_dal::TxDal;
use user_dal::UserDal;

use self::connection::holder::ConnectionHolder;

pub mod connection;
pub mod internal_tx_dal;
pub mod model;
pub mod referral_dal;
pub mod session_dal;
pub mod sg_meta_dal;
pub mod tier_dal;
pub mod tx_dal;
pub mod user_dal;
pub mod utils;
pub mod bandwidth_price_dal;

/// Storage processor is the main storage interaction point.
/// It holds down the connection (either direct or pooled) to the database
/// and provide methods to obtain different storage schemas.
#[derive(Debug)]
pub struct StorageProcessor<'a> {
    conn: ConnectionHolder<'a>,
    in_transaction: bool,
}

impl<'a> StorageProcessor<'a> {
    pub async fn start_transaction<'c: 'b, 'b>(&'c mut self) -> sqlx::Result<StorageProcessor<'b>> {
        let transaction = self.conn().begin().await?;
        let mut processor = StorageProcessor::from_transaction(transaction);
        processor.in_transaction = true;
        Ok(processor)
    }

    /// Checks if the `StorageProcessor` is currently within database transaction.
    pub fn in_transaction(&self) -> bool {
        self.in_transaction
    }

    fn from_transaction(conn: Transaction<'a, Postgres>) -> Self {
        Self {
            conn: ConnectionHolder::Transaction(conn),
            in_transaction: true,
        }
    }

    pub async fn commit(self) -> sqlx::Result<()> {
        if let ConnectionHolder::Transaction(transaction) = self.conn {
            transaction.commit().await
        } else {
            panic!("StorageProcessor::commit can only be invoked after calling StorageProcessor::begin_transaction");
        }
    }

    /// Creates a `StorageProcessor` using a pool of connections.
    /// This method borrows one of the connections from the pool, and releases it
    /// after `drop`.
    pub(crate) fn from_pool(conn: PoolConnection<Postgres>) -> Self {
        Self {
            conn: ConnectionHolder::Pooled(conn),
            in_transaction: false,
        }
    }

    fn conn(&mut self) -> &mut PgConnection {
        match &mut self.conn {
            ConnectionHolder::Pooled(conn) => conn,
            ConnectionHolder::Transaction(conn) => conn,
        }
    }

    pub fn internal_tx_dal(&mut self) -> InternalTxDal<'_, 'a> {
        InternalTxDal { storage: self }
    }

    pub fn session_dal(&mut self) -> SessionDal<'_, 'a> {
        SessionDal { storage: self }
    }

    pub fn tx_dal(&mut self) -> TxDal<'_, 'a> {
        TxDal { storage: self }
    }

    pub fn user_dal(&mut self) -> UserDal<'_, 'a> {
        UserDal { storage: self }
    }

    pub fn sg_meta_dal(&mut self) -> SgMetaDal<'_, 'a> {
        SgMetaDal { storage: self }
    }

    pub fn referral_dal(&mut self) -> ReferralDal<'_, 'a> {
        ReferralDal { storage: self }
    }

    pub fn tier_dal(&mut self) -> TierDal<'_, 'a> {
        TierDal { storage: self }
    }

    pub fn bandwidth_price_dal(&mut self) -> BandWidthPriceDal<'_, 'a> {
        BandWidthPriceDal { storage: self }
    }
}

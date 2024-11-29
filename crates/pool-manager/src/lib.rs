use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy_chains::Chain;
use amms::amm::uniswap_v2::batch_request::get_v2_pool_data_batch_request;
use amms::amm::uniswap_v3::batch_request::get_v3_pool_data_batch_request;
use amms::amm::ve33::factory::Ve33Factory;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
use amms::{
    amm::{
        factory::Factory, uniswap_v2::factory::UniswapV2Factory,
        uniswap_v3::factory::UniswapV3Factory,
    },
    sync::{self},
};
use db::establish_connection;
use db::models::db_pool::DbPool;
use db::models::{NewDbPool, NewDbUniV2Pool, NewDbUniV3Pool};
use db::queries::exchange::get_exchange_by_name;
use db::queries::uni_v2_pool::{
    batch_update_uni_v2_pool_active, batch_upsert_uni_v2_pools, get_uni_v2_pools,
};
use db::queries::uni_v3_pool::{
    batch_update_uni_v3_pool_active, batch_upsert_uni_v3_pools, get_uni_v3_pools,
};
use diesel::PgConnection;
use shared::evm_helpers::get_contract_creation_block;
use shared::pool_helpers::{db_pools_to_amms, extract_v2_pools, extract_v3_pools, filter_amms};
use std::sync::Arc;
use types::exchange::{ExchangeName, ExchangeType};

pub struct PoolStorageManager {
    db_url: String,
}

impl PoolStorageManager {
    pub fn new(db_url: &str) -> Self {
        Self {
            db_url: db_url.to_string(),
        }
    }

    /// Stores Uniswap V3 pools in the database.
    ///
    /// This function fetches Uniswap V3 pools from logs within a specified block range,
    /// populates their data, and stores them in the database.
    pub async fn store_uniswap_v3_pools<P, T, N>(
        &self,
        provider: Arc<P>,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        from_block: Option<u64>,
        to_block: Option<u64>,
        step: u64,
    ) -> Result<(), AMMError>
    where
        P: Provider<T, N>,
        T: Transport + Clone,
        N: Network,
    {
        let mut conn = establish_connection(&self.db_url);

        let start_block = from_block.unwrap_or(0);
        let end_block = to_block.unwrap_or(provider.get_block_number().await.unwrap());

        let contract_creation_block =
            get_contract_creation_block(provider.clone(), factory_address, start_block, end_block)
                .await
                .unwrap();

        let contract_creation_block = if contract_creation_block > start_block {
            contract_creation_block
        } else {
            start_block
        };

        let factory = UniswapV3Factory::new(factory_address, contract_creation_block);

        for block in (contract_creation_block..=end_block).step_by(step as usize) {
            tracing::info!("Fetching pools from block {:?}", block);

            let amms = factory
                .get_pools_from_logs(block, block + step - 1, step, provider.clone())
                .await?;

            let mut pools = extract_v3_pools(&amms);

            for mut chunk in pools.chunks_mut(50) {
                get_v3_pool_data_batch_request(chunk, None, provider.clone()).await?;

                let new_pools = chunk
                    .iter_mut()
                    .map(|pool| {
                        pool.exchange_type = ExchangeType::UniV3;
                        pool.exchange_name = exchange_name;
                        pool.chain = chain.named().unwrap();
                        pool.to_new_db_pool()
                    })
                    .filter_map(|db_pool| {
                        if let NewDbPool::UniV3(v3_pool) = db_pool {
                            Some(v3_pool)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NewDbUniV3Pool>>();

                batch_upsert_uni_v3_pools(&mut conn, &new_pools).unwrap();
                tracing::info!("Inserted {:?} pools", new_pools.len());
            }
        }

        Ok(())
    }

    /// Stores Uniswap V2 pools in the database.
    pub async fn store_uniswap_v2_pools<P, T, N>(
        &self,
        provider: Arc<P>,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
    ) -> Result<(), AMMError>
    where
        P: Provider<T, N> + 'static,
        T: Transport + Clone,
        N: Network,
    {
        let mut conn = establish_connection(&self.db_url);
        let factory = Factory::UniswapV2Factory(UniswapV2Factory::new(factory_address, 0, 3000));

        tracing::info!("Syncing uni-v2 like pools");

        let (amms, _) = sync::sync_amms(vec![factory], provider.clone(), None, 100000)
            .await
            .unwrap();

        let mut pools = extract_v2_pools(&amms);

        for mut chunk in pools.chunks_mut(50) {
            get_v2_pool_data_batch_request(&mut chunk, provider.clone()).await?;

            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::UniV2;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool()
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    pub async fn store_ve33_pools<P, T, N>(
        &self,
        provider: Arc<P>,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
    ) -> Result<(), AMMError>
    where
        P: Provider<T, N> + 'static,
        T: Transport + Clone,
        N: Network,
    {
        let mut conn = establish_connection(&self.db_url);
        let factory = Factory::Ve33Factory(Ve33Factory::new(factory_address, 0, 3000));

        tracing::info!("Syncing ve33 pools");

        let (amms, _) = sync::sync_amms(vec![factory], provider.clone(), None, 100000)
            .await
            .unwrap();

        tracing::info!("Amms synced");

        let mut pools = extract_v2_pools(&amms);

        for mut chunk in pools.chunks_mut(50) {
            // add additional data such as the exchange name
            get_v2_pool_data_batch_request(&mut chunk, provider.clone()).await?;
            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::Ve33;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool()
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            // upsert pools in the database
            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    async fn activate_v3_pools(
        &self,
        chain: Chain,
        mut conn: &mut PgConnection,
        exchange_name: ExchangeName,
        usd_threshold: f64,
    ) -> Result<(), AMMError> {
        let named_chain = chain.named().unwrap();

        let pools = get_uni_v3_pools(
            &mut conn,
            Some(&named_chain.to_string()),
            Some(&exchange_name.to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let v3_pools = pools
            .iter()
            .map(|pool| DbPool::UniV3((*pool).clone()))
            .collect::<Vec<DbPool>>();

        let amms = db_pools_to_amms(&v3_pools)?;

        tracing::info!("Got {:?} uni v3 pools", pools.len());

        for chunk in amms.chunks(1000) {
            let result = filter_amms(chain, usd_threshold, chunk.to_vec()).await;
            if result.is_err() {
                tracing::error!("Error filtering amms: {:?}", result.err());
                continue;
            }
            let active_pools = result.unwrap();
            let active_pools = active_pools
                .iter()
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            let inactive_pools = chunk
                .iter()
                .filter(|amm| !active_pools.contains(&amm.address().to_string()))
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            batch_update_uni_v3_pool_active(&mut conn, &active_pools, true).unwrap();
            batch_update_uni_v3_pool_active(&mut conn, &inactive_pools, false).unwrap();

            tracing::info!(
                "Processed pool chunk. Active pools: {:?}. Inactive pools: {:?}",
                active_pools.len(),
                inactive_pools.len()
            );
        }

        Ok(())
    }

    async fn activate_v2_pools(
        &self,
        chain: Chain,
        mut conn: &mut PgConnection,
        exchange_name: ExchangeName,
        usd_threshold: f64,
    ) -> Result<(), AMMError> {
        let named_chain = chain.named().unwrap();

        let pools = get_uni_v2_pools(
            &mut conn,
            Some(&named_chain.to_string()),
            Some(&exchange_name.to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let v2_pools = pools
            .iter()
            .map(|pool| DbPool::UniV2((*pool).clone()))
            .collect::<Vec<DbPool>>();

        let amms = db_pools_to_amms(&v2_pools)?;

        for chunk in amms.chunks(1000) {
            let result = filter_amms(chain, usd_threshold, chunk.to_vec()).await;
            if result.is_err() {
                tracing::error!("Error filtering amms: {:?}", result.err());
                continue;
            }
            let active_pools = result.unwrap();
            let active_pools = active_pools
                .iter()
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            let inactive_pools = chunk
                .iter()
                .filter(|amm| !active_pools.contains(&amm.address().to_string()))
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            batch_update_uni_v2_pool_active(&mut conn, &active_pools, true).unwrap();
            batch_update_uni_v2_pool_active(&mut conn, &inactive_pools, false).unwrap();

            tracing::info!(
                "Processed pool chunk. Active pools: {:?}. Inactive pools: {:?}",
                active_pools.len(),
                inactive_pools.len()
            );
        }

        Ok(())
    }

    /// Activates pools that meet a certain USD value threshold.
    pub async fn activate_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        usd_threshold: f64,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);

        let exchange = get_exchange_by_name(
            &mut conn,
            &chain.named().unwrap().to_string(),
            &exchange_name.to_string(),
        )
        .unwrap();

        let exchange_type =
            ExchangeType::from_str(&exchange.exchange_type).expect("Invalid exchange type");

        match exchange_type {
            ExchangeType::UniV2 => {
                self.activate_v2_pools(chain, &mut conn, exchange_name, usd_threshold)
                    .await?;
            }
            ExchangeType::UniV3 => {
                self.activate_v3_pools(chain, &mut conn, exchange_name, usd_threshold)
                    .await?;
            }
            _ => {
                return Err(AMMError::UnknownPoolType);
            }
        }

        Ok(())
    }
}

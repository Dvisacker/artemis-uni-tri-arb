// use crate::components::simulator::fork_factory::ForkFactory;
use crate::types::{UniV2Dex, UniV2Pool};
use ethers::{
    abi::{ParamType, Token},
    prelude::*,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
// use log::*;
// use revm::db::{CacheDB, EmptyDB};
use std::{collections::HashMap, sync::Arc};

pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

fn create_progress_bar_with_message(
    message: String,
    multi_progress_bar: &MultiProgress,
) -> ProgressBar {
    let progress_bar = multi_progress_bar.add(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7}")
            .expect("Error when setting progress bar style")
            .progress_chars("##-"),
    );
    progress_bar.set_message(message);
    progress_bar
}

pub async fn get_pools_batch_request<M: Middleware>(
    factory: H160,
    from: U256,
    step: U256,
    middleware: Arc<M>,
) -> Vec<UniV2Pool> {
    let mut pools = vec![];

    let constructor_args = Token::Tuple(vec![
        Token::Uint(from),
        Token::Uint(step),
        Token::Address(factory),
    ]);

    let deployer = GetUniswapV2poolsBatchRequest::deploy(middleware, constructor_args).unwrap();
    let return_data: Bytes = deployer.call_raw().await.unwrap();

    let return_data_tokens = ethers::abi::decode(
        &[ParamType::Array(Box::new(ParamType::Tuple(vec![
            ParamType::Address,   // pool address
            ParamType::Address,   // token a
            ParamType::Address,   // token b
            ParamType::Uint(112), // reserve 0
            ParamType::Uint(112), // reserve 1
        ])))],
        &return_data,
    )
    .unwrap();

    for tokens in return_data_tokens {
        if let Some(tokens_arr) = tokens.into_array() {
            for tup in tokens_arr {
                if let Some(pool_data) = tup.into_tuple() {
                    //If the pool token A is not zero, signaling that the pool data was populated
                    if !pool_data[0].to_owned().into_address().unwrap().is_zero() {
                        //Update the pool data
                        let pool_internal = UniV2Pool {
                            address: pool_data[0].to_owned().into_address().unwrap(),
                            token0: pool_data[1].to_owned().into_address().unwrap(),
                            token1: pool_data[2].to_owned().into_address().unwrap(),
                            reserve0: pool_data[3].to_owned().into_uint().unwrap(),
                            reserve1: pool_data[4].to_owned().into_uint().unwrap(),
                            router_fee: U256::from(9970),

                            fees0: U256::zero(),
                            fees1: U256::zero(),
                        };

                        pools.push(pool_internal);
                    }
                }
            }
        }
    }

    pools
}

pub async fn get_all_pools_via_batched_calls<M: 'static + Middleware>(
    factory_address: Address,
    middleware: Arc<M>,
    progress_bar: ProgressBar,
) -> Vec<UniV2Pool> {
    let factory = UniV2Factory::new(factory_address, middleware.clone());

    let pools_length: U256 = factory.all_pools_length().call().await.unwrap();
    //Initialize the progress bar message
    progress_bar.set_length(pools_length.as_u64());

    let mut pools = vec![];
    let step = 150; //max batch size for this call until codesize is too large
    let mut idx_from = U256::zero();
    let mut idx_to = if step > pools_length.as_usize() {
        pools_length
    } else {
        U256::from(step)
    };

    for _ in (0..pools_length.as_u128()).step_by(step) {
        pools.append(
            &mut get_pools_batch_request(factory_address, idx_from, idx_to, middleware.clone())
                .await,
        );

        idx_from = idx_to;

        if idx_to + step > pools_length {
            idx_to = pools_length - 1
        } else {
            idx_to = idx_to + step;
        }
        progress_bar.inc(step as u64);
    }
    progress_bar.reset();
    pools
}

pub async fn get_all_pools(
    dexes: Vec<UniV2Dex>,
    wss_provider: Arc<Provider<Ws>>,
) -> Option<Vec<UniV2Pool>> {
    let multi_progress_bar = MultiProgress::new();
    let current_block = wss_provider.get_block_number().await.unwrap();
    // let cache_db: CacheDB<EmptyDB> = CacheDB::new(EmptyDB::default());
    // let mut fork_factory = ForkFactory::new_sandbox_factory(wss_provider.clone(), cache_db, None);
    // inject_tax_checker_code(&mut fork_factory);

    let mut pools = Vec::new();

    for dex in dexes {
        let progress_bar = create_progress_bar_with_message(
            format!("Processing pools: {}", dex.factory),
            &multi_progress_bar,
        );

        let pools_internal = get_all_pools_via_batched_calls(
            dex.factory,
            wss_provider.clone(),
            progress_bar.clone(),
        )
        .await;

        progress_bar.reset();
        progress_bar.set_message("Getting tax".to_string());
        progress_bar.set_length(pools_internal.len() as u64);

        let progress_bar_internal =
            create_progress_bar_with_message("Spawning ".to_owned(), &multi_progress_bar);

        progress_bar_internal.set_length(pools_internal.len() as u64);

        let mut tasks_batch = Vec::new();
        let progress_bar_clone = progress_bar.clone();

        // for pool in pools_internal {
        //     insert_fake_approval(pool.token0, pool.address, &mut fork_factory);
        //     let sand_box = fork_factory.new_sandbox_fork();

        //     if pool.reserve0 < U256::from(1000000) || pool.reserve1 < U256::from(1000000) {
        //         progress_bar.inc(1);
        //         progress_bar_internal.inc(1);
        //         continue;
        //     }

        //     let dex_clone = dex.clone();
        //     let current_block_clone = current_block;
        //     let pool_clone = pool.clone();
        //     let progress_bar_clone = progress_bar_clone.clone();

        //     let task = tokio::task::spawn(async move {
        //         let (buy_tax, sell_tax) = match get_tax(
        //             pool_clone.token0,
        //             pool_clone.address,
        //             sand_box,
        //             current_block_clone,
        //             dex_clone.fee,
        //         )
        //         .await
        //         {
        //             Some(d) => d,
        //             None => {
        //                 progress_bar_clone.inc(1);
        //                 return None;
        //             }
        //         };

        //         if buy_tax > U256::from(9970) || sell_tax > U256::from(9970) {
        //             progress_bar_clone.inc(1);
        //             return None;
        //         }

        //         let mut cloned_pool = pool_clone.clone();
        //         cloned_pool.fees0 = buy_tax;
        //         cloned_pool.fees1 = sell_tax;

        //         progress_bar_clone.inc(1);
        //         Some(cloned_pool)
        //     });

        //     tasks_batch.push(task);
        //     progress_bar_internal.inc(1);

        //     if tasks_batch.len() >= 100 {
        //         for task in &mut tasks_batch {
        //             if let Some(pair) = task.await.unwrap() {
        //                 pools.push(pair);
        //             }
        //         }
        //         tokio::time::sleep(Duration::from_secs(1)).await;
        //         tasks_batch.clear();
        //     }
        // }

        // for task in tasks_batch {
        //     if let Some(pair) = task.await.unwrap() {
        //         pools.push(pair);
        //     }
        // }

        progress_bar.reset();
        progress_bar_internal.reset();
    }

    multi_progress_bar.clear().unwrap();

    Some(pools)
}

pub async fn update_reserves(
    pools: &mut Vec<UniV2Pool>,
    dexes: Vec<UniV2Dex>,
    wss_provider: Arc<Provider<Ws>>,
) {
    let multi_progress_bar = MultiProgress::new();
    let mut pools_new: HashMap<H160, UniV2Pool> = HashMap::new();

    for dex in dexes {
        let progress_bar = multi_progress_bar.add(ProgressBar::new(0));
        progress_bar.set_style(
            ProgressStyle::with_template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7}")
                .expect("Error when setting progress bar style")
                .progress_chars("##-"),
        );

        progress_bar.set_message(format!("Getting all pools from: {}", dex.factory));
        let pools_internal = get_all_pools_via_batched_calls(
            dex.factory,
            wss_provider.clone(),
            progress_bar.clone(),
        )
        .await;

        progress_bar.finish_and_clear();
        pools_new.extend(pools_internal.into_iter().map(|pair| (pair.address, pair)));
    }

    let banned_addresses = [
        address("0xd46ba6d942050d489dbd938a2c909a5d5039a161"),
        address("0x83B04AF7a77C727273B7a582D6Fda65472FCB3f2"),
        address("0x9766d2e3f04AE13e8c2EB018eA51dC640d3f9f1F"),
        address("0x7E3d39398C9574e1B4f9510Fd37aa3a47d602cDD"),
    ];

    pools.retain(|pair| {
        !banned_addresses.contains(&pair.token0)
            && !banned_addresses.contains(&pair.token1)
            && !banned_addresses.contains(&pair.address)
    });

    for pair in pools {
        if let Some(new_pair) = pools_new.get(&pair.address) {
            pair.reserve0 = new_pair.reserve0;
            pair.reserve1 = new_pair.reserve1;
        } else {
            panic!("Not supposed to happen");
        }
    }
}

// fn create_progress_bar_with_message(
//     message: String,
//     multi_progress_bar: &MultiProgress,
// ) -> ProgressBar {
//     let progress_bar = multi_progress_bar.add(ProgressBar::new(0));
//     progress_bar.set_style(
//         ProgressStyle::with_template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7}")
//             .expect("Error when setting progress bar style")
//             .progress_chars("##-"),
//     );
//     progress_bar.set_message(message);
//     progress_bar
// }

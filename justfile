set dotenv-load := true

#test contracts using forge
build-contracts:
    forge install --root ./contracts
    forge test --root ./contracts


build-amm-contracts:
    forge compile --root ./crates/amms-rs/contracts

#test contracts using forge
test-contracts: 
    forge test --root ./contracts

download-contracts:
    #!/usr/bin/env bash
    addressbook_path="./addressbook.json"
    multicall_address=$(jq -r ".mainnet.multicall" $addressbook_path)
    uniswap_v2_factory_address=$(jq -r ".mainnet.exchanges.univ2.uniswapv2.factory" $addressbook_path)
    uniswap_v2_router_address=$(jq -r ".mainnet.exchanges.univ2.uniswapv2.router" $addressbook_path)
    # weth_usdc_address=$(jq -r ".mainnet.univ2.uniswapv2.weth-usdc" $addressbook_path)
    uniswap_v3_factory_address=$(jq -r ".mainnet.exchanges.univ3.uniswapv3.factory" $addressbook_path)
    camelot_v3_factory_address=$(jq -r ".arbitrum.exchanges.univ3.camelotv3.factory" $addressbook_path)

    echo "Downloading multicall from $multicall_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $multicall_address

    echo "Downloading uniswap V2 factory from $uniswap_v2_factory_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v2_factory_address

    echo "Downloading uniswap V2 router from $uniswap_v2_router_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v2_router_address

    echo "Downloading uniswap V3 factory from $uniswap_v3_factory_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $uniswap_v3_factory_address

    echo "Downloading camelot V3 factory from $camelot_v3_factory_address"
    cast etherscan-source --etherscan-api-key $ARBISCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $camelot_v3_factory_address -c 42161

    echo "Downloading uniswap V2 pool from $weth_usdc_address"
    cast etherscan-source --etherscan-api-key $ETHERSCAN_API_KEY -d crates/strategies/uni-tri-arb/contracts/src $weth_usdc_address

generate-bindings:
    #!/usr/bin/env bash
    bindings_path="./crates/bindings"
    contract_root_path="./crates/strategies/uni-tri-arb/contracts/"
    rm -rf $bindings_path
    forge bind --bindings-path $bindings_path --root $contract_root_path --crate-name bindings --force --skip-cargo-toml --alloy

build-bindings: download-contracts generate-bindings

fmt: 
    cargo +nightly fmt --all

clippy: 
    cargo clippy --all --all-features

setup-db:
    diesel setup --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-new-migration NAME:
    diesel migration generate {{NAME}} --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-revert:
    diesel migration revert --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-migrate:
    diesel migration run --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

db-print:
    diesel print-schema -- --migration-dir ./crates/db/migrations --config-file ./crates/db/diesel.toml

run-bot-arbitrum:
    RUST_BACKTRACE=1 cargo run --bin bot -- --chain-id 42161

run-bot-mainnet:
    cargo run --bin bot -- --chain-id 1

get-filtered-pools CHAIN_ID:
    cargo run --bin cli -- filter --chain-id {{CHAIN_ID}}

# creation uniswap v3 on mainnet: 12369621
# last indexed: 12465620
get-uniswap-v3-pools CHAIN_ID EXCHANGE_NAME STEP:
    cargo run --bin cli -- get-uniswap-v3-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}} --from-block 18457620 --to-block 20709779 --step {{STEP}}

get-uniswap-v2-pools CHAIN_ID EXCHANGE_NAME:
    cargo run --bin cli -- get-uniswap-v2-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}}

get-amm-value CHAIN_ID POOL_ADDRESS:
    cargo run --bin cli -- get-amm-value --chain-id {{CHAIN_ID}} --pool-address {{POOL_ADDRESS}}

activate-pools CHAIN_ID EXCHANGE_NAME:
    cargo run --bin cli -- activate-pools --chain-id {{CHAIN_ID}} --exchange {{EXCHANGE_NAME}} --min-usd 100000


# 0x7858e59e0c01ea06df3af3d20ac7b0003275d4bf
# 0xf83d5aaab14507a53f97d3c18bdb52c4a62efc40
# 0x886072a44bdd944495eff38ace8ce75c1eacdaf6
# 0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8
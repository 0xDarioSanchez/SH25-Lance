#!/bin/bash
set -e

echo "************************************"
echo -e "\t*****Building*****..."
echo "************************************"
cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/lance_protocol.wasm

echo "**********************************"
echo -e "\t****Deploying & Initializing**** ..."
echo "**********************************"
stellar contract deploy \
  --wasm target/wasm32v1-none/release/lance_protocol.optimized.wasm \
  --source-account lance-admin \
  --network testnet \
  --alias lance-protocol \
  -- \
  --admin lance-admin \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

echo "***********************************************"
echo -e "\tRegistration of Judge 1 ..."
echo "***********************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- new_voter \
    --user judge-1

echo "***********************************************"
echo -e "\tRegistration of Judge 2 ..."
echo "***********************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- new_voter \
    --user judge-2

echo "***********************************************"
echo -e "\tRegistration of Judge 3 ..."
echo "***********************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-3 \
    --network testnet \
    -- new_voter \
    --user judge-3

echo "****************************************"
echo -e "\tGet Judge 1 Info..."
echo "*****************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- get_user \
    --user judge-1

echo "****************************************"
echo -e "\tGet Judge 2 Info..."
echo "*****************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- get_user \
    --user judge-2

echo "****************************************"
echo -e "\tGet Judge 3 Info..."
echo "*****************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-3 \
    --network testnet \
    -- get_user \
    --user judge-3

echo "**************************************************"
echo -e "\tCreating Dispute 1 from Admin ..."
echo "**************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- create_dispute \
    --creator lance-admin \
    --counterpart lance-admin \
    --id 1 \
    --proof "Test dispute for protocol testing"

echo "**********************************************************"
echo -e "\tJudge 1 registering to vote on Dispute 1 ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- register_to_vote \
    --creator judge-1 \
    --dispute_id 1

echo "**********************************************************"
echo -e "\tJudge 2 registering to vote on Dispute 1 ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- register_to_vote \
    --creator judge-2 \
    --dispute_id 1

echo "**********************************************************"
echo -e "\tJudge 3 registering to vote on Dispute 1 ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-3 \
    --network testnet \
    -- register_to_vote \
    --creator judge-3 \
    --dispute_id 1

# TODO: Add commit_vote and reveal_vote functions when implemented
# echo "**********************************************************"
# echo -e "\tJudge 1 committing vote on Dispute 1 ..."
# echo "**********************************************************"
# stellar contract invoke \
#     --id lance-protocol \
#     --source judge-1 \
#     --network testnet \
#     -- commit_vote \
#     --voter judge-1 \
#     --dispute_id 1 \
#     --commit_hash <hash>

# echo "**********************************************************"
# echo -e "\tJudge 1 revealing vote on Dispute 1 ..."
# echo "**********************************************************"
# stellar contract invoke \
#     --id lance-protocol \
#     --source judge-1 \
#     --network testnet \
#     -- reveal_vote \
#     --voter judge-1 \
#     --dispute_id 1 \
#     --vote true \
#     --secret <secret>

echo "**********************************************************"
echo -e "\tGetting balance for Admin ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- get_balance \
    --employee lance-admin

echo "******************************************************"
echo -e "\tOpening contract on Stellar Expert explorer"
echo "******************************************************"

CONTRACT_ID=$(stellar contract alias show lance-protocol)
EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
xdg-open "$EXPLORER_URL"
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

echo "**********************************************************"
echo -e "\tJudge 1 committing vote on Dispute 1 ..."
echo "**********************************************************"
# Judge 1 votes TRUE with secret "secret1" (hex: 73656372657431)
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- commit_vote \
    --voter judge-1 \
    --dispute_id 1 \
    --vote true \
    --secret '"73656372657431"'

echo "**********************************************************"
echo -e "\tJudge 2 committing vote on Dispute 1 ..."
echo "**********************************************************"
# Judge 2 votes FALSE with secret "secret2" (hex: 73656372657432)
stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- commit_vote \
    --voter judge-2 \
    --dispute_id 1 \
    --vote false \
    --secret '"73656372657432"'

echo "**********************************************************"
echo -e "\tJudge 3 committing vote on Dispute 1 ..."
echo "**********************************************************"
# Judge 3 votes TRUE with secret "secret3" (hex: 73656372657433)
stellar contract invoke \
    --id lance-protocol \
    --source judge-3 \
    --network testnet \
    -- commit_vote \
    --voter judge-3 \
    --dispute_id 1 \
    --vote true \
    --secret '"73656372657433"'

echo "**********************************************************"
echo -e "\tDispute creator revealing ALL votes at once ..."
echo "**********************************************************"
# Creator reveals all votes with their secrets
# Votes: [true, false, true] for judges 1, 2, 3
# Secrets: ["secret1", "secret2", "secret3"] in hex format
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- reveal_votes \
    --creator lance-admin \
    --dispute_id 1 \
    --votes '[true, false, true]' \
    --secrets '["73656372657431", "73656372657432", "73656372657433"]'

echo "**********************************************************"
echo -e "\tGetting balance for Admin ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- get_balance \
    --employee lance-admin

# echo "******************************************************"
# echo -e "\tOpening contract on Stellar Expert explorer"
# echo "******************************************************"

# CONTRACT_ID=$(stellar contract alias show lance-protocol)
# EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
# xdg-open "$EXPLORER_URL"
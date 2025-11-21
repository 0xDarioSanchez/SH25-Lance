#!/bin/bash
set -e

# Lance Protocol - Dispute Resolution Demo Script
# This script demonstrates the complete workflow of the Lance Protocol:
# 1. Build and deploy the contract
# 2. Register judges (voters)
# 3. Create a dispute
# 4. Judges register to vote on the dispute
# 5. Judges commit their votes (encrypted with secrets)
# 6. Creator reveals all votes at once to determine the winner
#
# Voting is a commit-reveal process:
# - Commit phase: Judges submit hashed votes (vote + secret)
# - Reveal phase: Creator reveals all votes with their secrets
# - The contract verifies the secrets match and counts the votes

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
echo -e "\tJudge 1 committing vote on Dispute 1 (votes FOR creator) ..."
echo "**********************************************************"
# Judge 1 votes TRUE (for creator) with secret "judge1_secret"
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- commit_vote \
    --voter judge-1 \
    --dispute_id 1 \
    --vote true \
    --secret '{"bytes":"6a75646765315f736563726574"}'

echo "**********************************************************"
echo -e "\tJudge 2 committing vote on Dispute 1 (votes AGAINST creator) ..."
echo "**********************************************************"
# Judge 2 votes FALSE (against creator) with secret "judge2_secret"
stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- commit_vote \
    --voter judge-2 \
    --dispute_id 1 \
    --vote false \
    --secret '{"bytes":"6a75646765325f736563726574"}'

echo "**********************************************************"
echo -e "\tJudge 3 committing vote on Dispute 1 (votes FOR creator) ..."
echo "**********************************************************"
# Judge 3 votes TRUE (for creator) with secret "judge3_secret"
stellar contract invoke \
    --id lance-protocol \
    --source judge-3 \
    --network testnet \
    -- commit_vote \
    --voter judge-3 \
    --dispute_id 1 \
    --vote true \
    --secret '{"bytes":"6a75646765335f736563726574"}'

echo "**********************************************************"
echo -e "\tDispute creator revealing ALL votes at once ..."
echo "**********************************************************"
# Creator reveals all votes with their secrets
# Result: 2 votes FOR (true), 1 vote AGAINST (false) => Creator wins!
# Votes: [true, false, true] for judges 1, 2, 3
# Secrets in hex: ["judge1_secret", "judge2_secret", "judge3_secret"]
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- reveal_votes \
    --creator lance-admin \
    --dispute_id 1 \
    --votes '[true, false, true]' \
    --secrets '[{"bytes":"6a75646765315f736563726574"}, {"bytes":"6a75646765325f736563726574"}, {"bytes":"6a75646765335f736563726574"}]'

echo "**********************************************************"
echo -e "\tGetting balance for Admin ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- get_balance \
    --employee lance-admin

echo "**********************************************************"
echo -e "\tGetting balances for judges ..."
echo "**********************************************************"
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- get_balance \
    --employee judge-1

stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- get_balance \
    --employee judge-2

stellar contract invoke \
    --id lance-protocol \
    --source judge-3 \
    --network testnet \
    -- get_balance \
    --employee judge-3

echo ""
echo "============================================================"
echo -e "\t✅ DISPUTE RESOLUTION COMPLETE!"
echo "============================================================"
echo "Result: 2 votes FOR creator (true), 1 vote AGAINST (false)"
echo "Winner: lance-admin (the creator)"
echo "Judges who voted for the winner receive rewards"
echo "============================================================"
echo ""

# echo "******************************************************"
# echo -e "\tOpening contract on Stellar Expert explorer"
# echo "******************************************************"

# CONTRACT_ID=$(stellar contract alias show lance-protocol)
# EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
# xdg-open "$EXPLORER_URL"
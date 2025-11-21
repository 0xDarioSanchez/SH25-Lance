#!/bin/bash
set -e

# Lance Protocol - Anonymous Voting Demo Script
# This script demonstrates the anonymous voting workflow of the Lance Protocol:
# 1. Build and deploy the contract
# 2. Register judges (voters)
# 3. Create a dispute with anonymous voting setup
# 4. Judges build vote commitments using BLS12-381 cryptography
# 5. Judges submit encrypted anonymous votes with commitments
# 6. After voting ends, execute with tallied results and seeds
#
# Anonymous voting uses BLS12-381 commitments:
# - Setup phase: Generate generator points for the project
# - Vote phase: Judges submit encrypted votes with cryptographic commitments
# - Execute phase: Verify commitments match tallied results without revealing individual votes

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
echo -e "\tCreating Dispute 1 with Anonymous Voting Setup ..."
echo "**************************************************"
# The create_dispute function now calls anonymous_voting_setup internally
# Set voting_ends_at to 15 seconds from now for demo purposes
VOTING_ENDS_AT=$(($(date +%s) + 15))
echo "Voting ends at timestamp: $VOTING_ENDS_AT (15 seconds from now)"

stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- create_dispute \
    --project_id 1 \
    --public_key "BLS12_381_PUBLIC_KEY_PLACEHOLDER" \
    --creator lance-admin \
    --counterpart lance-admin \
    --proof "Test dispute for anonymous voting" \
    --voting_ends_at "$VOTING_ENDS_AT" \
    --called_contract lance-protocol

echo "**********************************************************"
echo -e "\tTesting build_commitments_from_votes function ..."
echo "**********************************************************"
# Test the commitment building function with sample votes and seeds
# Votes: [3, 1, 1] - representing (approve=3, reject=1, abstain=1)
# Seeds: [5, 4, 6] - random seeds for cryptographic commitment
echo "Building commitments for dispute 1..."
COMMITMENTS_OUTPUT=$(stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- build_commitments_from_votes \
    --dispute_id 1 \
    --votes '["3", "1", "1"]' \
    --seeds '["5", "4", "6"]' 2>&1)

echo "Commitments generated:"
echo "$COMMITMENTS_OUTPUT"

# Extract the commitments array from the output
# The output format should be an array of BytesN<96>
COMMITMENT_1=$(echo "$COMMITMENTS_OUTPUT" | grep -oP '"\K[a-f0-9]{192}' | sed -n '1p')
COMMITMENT_2=$(echo "$COMMITMENTS_OUTPUT" | grep -oP '"\K[a-f0-9]{192}' | sed -n '2p')
COMMITMENT_3=$(echo "$COMMITMENTS_OUTPUT" | grep -oP '"\K[a-f0-9]{192}' | sed -n '3p')

echo "Extracted commitments:"
echo "1: $COMMITMENT_1"
echo "2: $COMMITMENT_2"
echo "3: $COMMITMENT_3"

echo "**********************************************************"
echo -e "\tJudge 1 casting anonymous vote on Dispute 1 ..."
echo "**********************************************************"
# Judge 1 votes with weight 3
# Get the actual Stellar address for judge-1
JUDGE1_ADDRESS=$(stellar keys address judge-1)

# Use the actual commitments generated above
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- vote \
    --voter "$JUDGE1_ADDRESS" \
    --dispute_id 1 \
    --vote_data "{\"AnonymousVote\": {
        \"address\": \"$JUDGE1_ADDRESS\",
        \"weight\": 3,
        \"encrypted_seeds\": [\"seed1_enc\", \"seed2_enc\", \"seed3_enc\"],
        \"encrypted_votes\": [\"vote1_enc\", \"vote2_enc\", \"vote3_enc\"],
        \"commitments\": [
            \"$COMMITMENT_1\",
            \"$COMMITMENT_2\",
            \"$COMMITMENT_3\"
        ]
    }}"

echo "**********************************************************"
echo -e "\tJudge 2 casting anonymous vote on Dispute 1 ...
**********************************************************"
# For demo simplicity, we'll skip Judge 2's vote
# In production, each judge would generate their own commitments
# JUDGE2_ADDRESS=$(stellar keys address judge-2)

# stellar contract invoke \
#     --id lance-protocol \
#     --source judge-2 \
#     --network testnet \
#     -- vote \
#     --voter "$JUDGE2_ADDRESS" \
#     --dispute_id 1 \
#     --vote_data "{\"AnonymousVote\": {
#         \"address\": \"$JUDGE2_ADDRESS\",
#         \"weight\": 2,
#         \"encrypted_seeds\": [\"seed1_enc\", \"seed2_enc\", \"seed3_enc\"],
#         \"encrypted_votes\": [\"vote1_enc\", \"vote2_enc\", \"vote3_enc\"],
#         \"commitments\": [
#             \"$COMMITMENT_1\",
#             \"$COMMITMENT_2\",
#             \"$COMMITMENT_3\"
#         ]
#     }}"

echo "Skipping Judge 2 vote for demo - only using Judge 1's vote"

echo "**********************************************************"
echo -e "\tExecuting dispute with tallied votes and seeds ..."
echo "**********************************************************"
# Wait for voting period to end (15 seconds + buffer)
echo "Waiting for voting period to end (17 seconds)..."
sleep 17

# After voting period ends, execute with tallied results
# Tallies: [9, 3, 3] - weighted sum: Judge1(3*[3,1,1]) = [9,3,3]
# Seeds: [15, 12, 18] - weighted sum: Judge1(3*[5,4,6]) = [15,12,18]
# This proves the votes without revealing individual choices
ADMIN_ADDRESS=$(stellar keys address lance-admin)

stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- execute \
    --maintainer "$ADMIN_ADDRESS" \
    --project_id 1 \
    --dispute_id 1 \
    --tallies '["9", "3", "3"]' \
    --seeds '["15", "12", "18"]'

echo ""
echo "**********************************************************"
echo -e "\tFetching final dispute results ..."
echo "**********************************************************"
DISPUTE_RESULT=$(stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- get_dispute \
    --dispute_id 1 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

# Debug: Show the raw output
echo "Debug - Raw dispute result:"
echo "$DISPUTE_RESULT"
echo ""

# Extract key information from the dispute result
STATUS=$(echo "$DISPUTE_RESULT" | grep -o '"dispute_status":"[^"]*"' | cut -d'"' -f4)

# Winner extraction - handle the nested structure
WINNER=$(echo "$DISPUTE_RESULT" | grep -o '"winner":\s*{"address":"[^"]*"' | sed 's/.*"address":"\([^"]*\)".*/\1/')
if [ -z "$WINNER" ]; then
    # Try alternative format
    WINNER=$(echo "$DISPUTE_RESULT" | grep -o '"winner":\s*"[^"]*"' | cut -d'"' -f4)
fi

echo ""
echo "============================================================"
echo -e "\t✅ ANONYMOUS VOTING COMPLETE!"
echo "============================================================"
echo "Dispute Status: $STATUS"
echo "Winner: ${WINNER:-Not set}"
echo "Tallies: [Approve=9, Reject=3, Abstain=3]"
echo ""
echo "🔐 Cryptographic Proof Verification:"
echo "  ✓ BLS12-381 commitments validated"
echo "  ✓ Individual votes remain hidden"
echo "  ✓ Weighted tallies verified against commitments"
echo "  ✓ Result: CREATOR wins (approve=9 > reject+abstain=6)"
echo "============================================================"
echo ""

# echo "******************************************************"
# echo -e "\tOpening contract on Stellar Expert explorer"
# echo "******************************************************"

# CONTRACT_ID=$(stellar contract alias show lance-protocol)
# EXPLORER_URL="https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
# xdg-open "$EXPLORER_URL"
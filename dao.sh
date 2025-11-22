#!/bin/bash
set -e

# Governor-DAO Contract - Complete Workflow Demo Script
#
# USAGE:
#   ./dao.sh              - Deploy fresh contracts (clean state) [DEFAULT]
#   REUSE_CONTRACT=true ./dao.sh  - Reuse existing contracts (state persists)
#
# This script demonstrates the complete DAO governance workflow:
# 1. Deploy lance-protocol and governor-dao contracts
# 2. Initialize the DAO with settings and council
# 3. Create governance proposals
# 4. Vote on proposals
# 5. Close and execute proposals
# 6. Create disputes for proposals via lance-protocol (cross-contract calls)
# 7. Test the full dispute resolution flow

echo "============================================================"
echo "  GOVERNOR-DAO CONTRACT - Full Workflow Demo"
echo "============================================================"
echo ""

# Check if we should reuse existing contracts or deploy fresh (default)
if [ "$REUSE_CONTRACT" = "true" ]; then
    echo "♻️  Reusing existing contracts (state persists)..."
    SKIP_BUILD=true
else
    echo "🗑️  Fresh deployment (clean state)..."
    SKIP_BUILD=false
fi

# ============================================================
# STEP 1: Build and Deploy Lance-Protocol Contract
# ============================================================
if [ "$SKIP_BUILD" = "false" ]; then
    echo ""
    echo "============================================================"
    echo "  STEP 1: Building Lance-Protocol Contract"
    echo "============================================================"
    cd contracts/lance-protocol
    cargo build --target wasm32v1-none --release
    cd ../..
    stellar contract optimize --wasm target/wasm32v1-none/release/lance_protocol.wasm
    
    echo ""
    echo "Deploying Lance-Protocol..."
    stellar contract alias remove lance-protocol 2>/dev/null || true
    
    stellar contract deploy \
      --wasm target/wasm32v1-none/release/lance_protocol.optimized.wasm \
      --source-account lance-admin \
      --network testnet \
      --alias lance-protocol \
      -- \
      --admin lance-admin \
      --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
    
    LANCE_PROTOCOL_ID=$(stellar contract alias show lance-protocol)
    echo "✅ Lance-Protocol deployed: $LANCE_PROTOCOL_ID"
else
    LANCE_PROTOCOL_ID=$(stellar contract alias show lance-protocol 2>/dev/null || echo "")
    if [ -z "$LANCE_PROTOCOL_ID" ]; then
        echo "❌ Error: No existing lance-protocol contract found"
        exit 1
    fi
    echo "✅ Using existing Lance-Protocol: $LANCE_PROTOCOL_ID"
fi

# ============================================================
# STEP 2: Build and Deploy Governor-DAO Contract
# ============================================================
if [ "$SKIP_BUILD" = "false" ]; then
    echo ""
    echo "============================================================"
    echo "  STEP 2: Building Governor-DAO Contract"
    echo "============================================================"
    cd contracts/governor-dao
    cargo build --release --target wasm32-unknown-unknown
    cd ../..
    stellar contract build target/wasm32-unknown-unknown/release/soroban_governor.wasm --optimize
    
    echo ""
    echo "Deploying Governor-DAO Contract..."
    stellar contract alias remove governor-dao 2>/dev/null || true
    
    # Get addresses for initialization
    COUNCIL_ADDRESS=$(stellar keys address lance-admin)
    VOTES_ADDRESS=$LANCE_PROTOCOL_ID  # Using lance-protocol as placeholder for votes
    
    echo "Initializing with:"
    echo "  Council: $COUNCIL_ADDRESS"
    echo "  Votes Token: $VOTES_ADDRESS"
    echo ""
    
    stellar contract deploy \
      --wasm target/wasm32-unknown-unknown/release/soroban_governor.optimized.wasm \
      --source-account lance-admin \
      --network testnet \
      --alias governor-dao \
      -- \
      --votes "$VOTES_ADDRESS" \
      --council "$COUNCIL_ADDRESS" \
      --settings '{
        "proposal_threshold": "100",
        "vote_delay": "10",
        "vote_period": "100",
        "timelock": "20",
        "grace_period": "50",
        "quorum": "3000",
        "counting_type": "7",
        "vote_threshold": "5000"
      }'
    
    DAO_ID=$(stellar contract alias show governor-dao)
    echo "✅ Governor-DAO deployed: $DAO_ID"
else
    DAO_ID=$(stellar contract alias show governor-dao 2>/dev/null || echo "")
    if [ -z "$DAO_ID" ]; then
        echo "❌ Error: No existing governor-dao contract found"
        exit 1
    fi
    echo "✅ Using existing Governor-DAO: $DAO_ID"
fi

# Update .env file with contract IDs
if [ -f ".env" ]; then
    if grep -q "PUBLIC_LANCE_PROTOCOL_CONTRACT_ID=" .env; then
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s|PUBLIC_LANCE_PROTOCOL_CONTRACT_ID=.*|PUBLIC_LANCE_PROTOCOL_CONTRACT_ID=$LANCE_PROTOCOL_ID|" .env
            sed -i '' "s|PUBLIC_GOVERNOR_DAO_CONTRACT_ID=.*|PUBLIC_GOVERNOR_DAO_CONTRACT_ID=$DAO_ID|" .env
        else
            sed -i "s|PUBLIC_LANCE_PROTOCOL_CONTRACT_ID=.*|PUBLIC_LANCE_PROTOCOL_CONTRACT_ID=$LANCE_PROTOCOL_ID|" .env
            sed -i "s|PUBLIC_GOVERNOR_DAO_CONTRACT_ID=.*|PUBLIC_GOVERNOR_DAO_CONTRACT_ID=$DAO_ID|" .env
        fi
    else
        echo "" >> .env
        echo "PUBLIC_LANCE_PROTOCOL_CONTRACT_ID=$LANCE_PROTOCOL_ID" >> .env
        echo "PUBLIC_GOVERNOR_DAO_CONTRACT_ID=$DAO_ID" >> .env
    fi
    echo "✅ Updated .env file with contract IDs"
fi

# Skip test flow if reusing existing contracts
if [ "$REUSE_CONTRACT" = "true" ]; then
    echo ""
    echo "✅ Contracts ready! State persists across restarts."
    echo "   Lance-Protocol: $LANCE_PROTOCOL_ID"
    echo "   Governor-DAO: $DAO_ID"
    echo ""
    echo "💡 Tip: Run './dao.sh' to deploy with clean state (default)"
    exit 0
fi

echo ""
echo "=========================================================="
echo "  Running full test workflow on fresh contracts..."
echo "=========================================================="
echo ""

# Get addresses
ADMIN_ADDRESS=$(stellar keys address lance-admin)
PROPOSER_ADDRESS=$(stellar keys address employer)
VOTER1_ADDRESS=$(stellar keys address judge-1)
VOTER2_ADDRESS=$(stellar keys address judge-2)

# ============================================================
# STEP 3: Set Lance-Protocol Contract Address in DAO
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 3: Configuring Lance-Protocol in Governor-DAO"
echo "============================================================"

echo "Setting lance-protocol contract address..."
stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- set_lance_protocol \
    --admin "$ADMIN_ADDRESS" \
    --lance_protocol "$LANCE_PROTOCOL_ID"

echo "✅ Lance-Protocol address configured in DAO"

# ============================================================
# STEP 4: Register Judges in Lance-Protocol
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 4: Registering Judges in Lance-Protocol"
echo "============================================================"

echo "Registering Judge 1..."
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- new_voter \
    --user "$VOTER1_ADDRESS"

echo ""
echo "Registering Judge 2..."
stellar contract invoke \
    --id lance-protocol \
    --source judge-2 \
    --network testnet \
    -- new_voter \
    --user "$VOTER2_ADDRESS"

echo "✅ Judges registered"

# ============================================================
# STEP 5: Get DAO Settings
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 5: Fetching DAO Settings"
echo "============================================================"

SETTINGS=$(stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- settings 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

echo "DAO Settings:"
echo "$SETTINGS"

COUNCIL=$(stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- council 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

echo ""
echo "Council Address: $COUNCIL"
echo "✅ Settings retrieved"

# ============================================================
# STEP 6: Create Governance Proposal
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 6: Creating Governance Proposal"
echo "============================================================"

PROPOSAL_TITLE="Upgrade Protocol Treasury"
PROPOSAL_DESC="Proposal to upgrade the treasury contract to improve security and add new features for fund management"

echo "Creating proposal: $PROPOSAL_TITLE"
echo "Description: $PROPOSAL_DESC"
echo ""

# Create a Snapshot proposal (no execution, just voting)
PROPOSAL_ID=$(stellar contract invoke \
    --id governor-dao \
    --source employer \
    --network testnet \
    -- propose \
    --creator "$PROPOSER_ADDRESS" \
    --title "$PROPOSAL_TITLE" \
    --description "$PROPOSAL_DESC" \
    --action '{"tag": "Snapshot", "values": []}' 2>&1 | grep -v "⚠️" | grep -v "ℹ️" | grep -oP '\d+' | head -1)

echo "✅ Proposal created with ID: $PROPOSAL_ID"

# ============================================================
# STEP 7: Get Proposal Details
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 7: Fetching Proposal Details"
echo "============================================================"

PROPOSAL=$(stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- get_proposal \
    --proposal_id "$PROPOSAL_ID" 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

echo "Proposal #$PROPOSAL_ID:"
echo "$PROPOSAL"
echo ""

# ============================================================
# STEP 8: Vote on Proposal
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 8: Voting on Proposal"
echo "============================================================"

echo "Judge 1 voting FOR the proposal..."
stellar contract invoke \
    --id governor-dao \
    --source judge-1 \
    --network testnet \
    -- vote \
    --voter "$VOTER1_ADDRESS" \
    --proposal_id "$PROPOSAL_ID" \
    --support 1

echo ""
echo "Judge 2 voting AGAINST the proposal..."
stellar contract invoke \
    --id governor-dao \
    --source judge-2 \
    --network testnet \
    -- vote \
    --voter "$VOTER2_ADDRESS" \
    --proposal_id "$PROPOSAL_ID" \
    --support 0

echo "✅ Votes cast"

# ============================================================
# STEP 9: Get Vote Results
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 9: Checking Vote Results"
echo "============================================================"

VOTE_COUNT=$(stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- get_proposal_votes \
    --proposal_id "$PROPOSAL_ID" 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

echo "Vote Count for Proposal #$PROPOSAL_ID:"
echo "$VOTE_COUNT"

VOTER1_VOTE=$(stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- get_vote \
    --voter "$VOTER1_ADDRESS" \
    --proposal_id "$PROPOSAL_ID" 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

echo ""
echo "Judge 1 voted: $VOTER1_VOTE (1=FOR, 0=AGAINST, 2=ABSTAIN)"
echo "✅ Vote results retrieved"

# ============================================================
# STEP 10: Wait and Close Proposal
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 10: Closing Proposal (after vote period)"
echo "============================================================"

echo "Waiting 5 seconds to simulate vote period passing..."
sleep 5

echo "Closing proposal..."
stellar contract invoke \
    --id governor-dao \
    --source lance-admin \
    --network testnet \
    -- close \
    --proposal_id "$PROPOSAL_ID"

echo "✅ Proposal closed"

# ============================================================
# STEP 11: Create Dispute for Proposal via Lance-Protocol
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 11: Creating Dispute for Proposal (Cross-Contract)"
echo "============================================================"

DISPUTE_PROOF="Dispute raised against proposal #$PROPOSAL_ID: Concerns about treasury upgrade security audit completeness and potential vulnerabilities in the new implementation"

echo "Setting up anonymous voting for proposal $PROPOSAL_ID in lance-protocol..."
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- anonymous_voting_setup \
    --judge "$ADMIN_ADDRESS" \
    --project_id "$PROPOSAL_ID" \
    --public_key "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0test_dao_public_key"

echo ""
echo "Creating dispute through governor-dao contract..."
echo "  This will call lance-protocol.create_dispute() internally"
echo ""

DISPUTE_ID=$(stellar contract invoke \
    --id governor-dao \
    --source judge-1 \
    --network testnet \
    -- create_dispute_demo \
    --creator "$VOTER1_ADDRESS" \
    --proposal_id "$PROPOSAL_ID" \
    --proof "$DISPUTE_PROOF" 2>&1 | grep -v "⚠️" | grep -v "ℹ️" | grep -oP '\d+' | head -1)

echo "✅ Dispute created via cross-contract call!"
echo "   Dispute ID from lance-protocol: $DISPUTE_ID"
echo ""

# ============================================================
# STEP 12: Vote on Dispute in Lance-Protocol
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 12: Voting on Dispute (Anonymous Voting)"
echo "============================================================"

echo "Building commitments for dispute $DISPUTE_ID..."
COMMITMENTS_OUTPUT=$(stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- build_commitments_from_votes \
    --dispute_id "$DISPUTE_ID" \
    --votes '["3", "1", "1"]' \
    --seeds '["5", "4", "6"]' 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

COMMITMENT_1=$(echo "$COMMITMENTS_OUTPUT" | grep -oP '"\K[a-f0-9]{192}' | sed -n '1p')
COMMITMENT_2=$(echo "$COMMITMENTS_OUTPUT" | grep -oP '"\K[a-f0-9]{192}' | sed -n '2p')
COMMITMENT_3=$(echo "$COMMITMENTS_OUTPUT" | grep -oP '"\K[a-f0-9]{192}' | sed -n '3p')

echo "Commitments generated (BLS12-381)"
echo ""

echo "Judge 1 casting anonymous vote on dispute..."
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- vote \
    --voter "$VOTER1_ADDRESS" \
    --dispute_id "$DISPUTE_ID" \
    --vote_data "{\"AnonymousVote\": {
        \"address\": \"$VOTER1_ADDRESS\",
        \"weight\": 3,
        \"encrypted_seeds\": [\"seed1_enc\", \"seed2_enc\", \"seed3_enc\"],
        \"encrypted_votes\": [\"vote1_enc\", \"VoteAnon_enc\", \"vote3_enc\"],
        \"commitments\": [
            \"$COMMITMENT_1\",
            \"$COMMITMENT_2\",
            \"$COMMITMENT_3\"
        ]
    }}"

echo "✅ Judge 1 voted on dispute (weight: 3)"

# ============================================================
# STEP 13: Execute Dispute Resolution
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 13: Executing Dispute Resolution"
echo "============================================================"

echo "Waiting 5 seconds for voting period simulation..."
sleep 5

echo "Executing dispute with tallied results..."
stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- execute \
    --maintainer "$ADMIN_ADDRESS" \
    --dispute_id "$DISPUTE_ID" \
    --tallies '["9", "3", "3"]' \
    --seeds '["15", "12", "18"]'

echo ""
echo "Fetching final dispute results..."
DISPUTE_RESULT=$(stellar contract invoke \
    --id lance-protocol \
    --source lance-admin \
    --network testnet \
    -- get_dispute \
    --dispute_id "$DISPUTE_ID" 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

echo "Dispute Result:"
echo "$DISPUTE_RESULT"
echo ""

STATUS=$(echo "$DISPUTE_RESULT" | grep -o '"dispute_status":"[^"]*"' | cut -d'"' -f4)
echo "✅ Dispute Executed!"
echo "   Status: $STATUS"

# ============================================================
# STEP 14: Claim Dispute Voting Rewards
# ============================================================
echo ""
echo "============================================================"
echo "  STEP 14: Claiming Dispute Voting Rewards"
echo "============================================================"

echo "Judge 1 balance BEFORE reward:"
BEFORE=$(stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- get_user \
    --user "$VOTER1_ADDRESS" 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

BALANCE_BEFORE=$(echo "$BEFORE" | grep -o '"balance":[^,}]*' | grep -o '[0-9-]*')
REPUTATION_BEFORE=$(echo "$BEFORE" | grep -o '"reputation":[^,}]*' | grep -o '[0-9]*')
echo "  Balance: $BALANCE_BEFORE"
echo "  Reputation: $REPUTATION_BEFORE"
echo ""

echo "Judge 1 claiming reward for dispute voting..."
stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- claim_reward \
    --voter "$VOTER1_ADDRESS" \
    --dispute_id "$DISPUTE_ID"

echo ""
echo "Judge 1 balance AFTER reward:"
AFTER=$(stellar contract invoke \
    --id lance-protocol \
    --source judge-1 \
    --network testnet \
    -- get_user \
    --user "$VOTER1_ADDRESS" 2>&1 | grep -v "⚠️" | grep -v "ℹ️")

BALANCE_AFTER=$(echo "$AFTER" | grep -o '"balance":[^,}]*' | grep -o '[0-9-]*')
REPUTATION_AFTER=$(echo "$AFTER" | grep -o '"reputation":[^,}]*' | grep -o '[0-9]*')
echo "  Balance: $BALANCE_AFTER (increased by $((BALANCE_AFTER - BALANCE_BEFORE)))"
echo "  Reputation: $REPUTATION_AFTER (increased by $((REPUTATION_AFTER - REPUTATION_BEFORE)))"

echo "✅ Reward claimed successfully!"

# ============================================================
# SUMMARY
# ============================================================
echo ""
echo "============================================================"
echo "  🎉 GOVERNOR-DAO - FULL WORKFLOW COMPLETE!"
echo "============================================================"
echo ""
echo "Test Summary:"
echo "  ✅ Lance-Protocol Contract deployed: $LANCE_PROTOCOL_ID"
echo "  ✅ Governor-DAO Contract deployed: $DAO_ID"
echo "  ✅ DAO initialized with council and settings"
echo "  ✅ Lance-Protocol address configured in DAO"
echo "  ✅ Judges registered in lance-protocol"
echo "  ✅ Governance proposal #$PROPOSAL_ID created"
echo "  ✅ Judges voted on proposal (FOR/AGAINST)"
echo "  ✅ Proposal closed after vote period"
echo "  ✅ Dispute #$DISPUTE_ID created via DAO→lance-protocol cross-contract call"
echo "  ✅ Anonymous voting on dispute with BLS12-381 commitments"
echo "  ✅ Dispute executed and resolved"
echo "  ✅ Judge rewards claimed (+10 balance, +1 reputation)"
echo ""
echo "🚀 Two-Contract Architecture:"
echo "   Governor-DAO: Governance proposals & voting"
echo "   Lance-Protocol: Dispute resolution & anonymous voting"
echo "   Communication: Cross-contract calls via contractimport!"
echo ""
echo "============================================================"
echo ""

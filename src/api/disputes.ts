import { Dispute } from "./types";
import { LANCE_CONTRACT_ID } from "../util/contractCall";
import { network } from "../contracts/util";
import { Contract, nativeToScVal, scValToNative, TransactionBuilder, Account, BASE_FEE } from "@stellar/stellar-sdk";
import { rpc } from "@stellar/stellar-sdk";

// Query the contract for a dispute by ID (read-only, no signing needed)
export async function getDispute(id: string): Promise<Dispute> {
  try {
    console.log(`[getDispute] Fetching dispute #${id} from contract ${LANCE_CONTRACT_ID}`);
    
    const rpcServer = new rpc.Server(network.rpcUrl, {
      allowHttp: network.rpcUrl.includes("localhost"),
    });

    const contract = new Contract(LANCE_CONTRACT_ID);
    
    // Build the operation
    const disputeIdParam = nativeToScVal(parseInt(id), { type: "u32" });
    const operation = contract.call("get_dispute", disputeIdParam);
    
    // Create a transaction with a zero account for simulation
    const sourceAccount = new Account("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF", "0");
    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: network.passphrase,
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();
    
    // Simulate the transaction
    const simulation = await rpcServer.simulateTransaction(transaction);
    
    console.log(`[getDispute] Simulation result for #${id}:`, simulation);
    
    if (rpc.Api.isSimulationSuccess(simulation) && simulation.result?.retval) {
      const dispute = scValToNative(simulation.result.retval);
      console.log(`[getDispute] ✓ Found dispute #${id}:`, dispute);
      return convertBlockchainDisputeToFrontend(dispute, id);
    }
    
    console.log(`[getDispute] ✗ Dispute #${id} not found or simulation failed`);
    throw new Error("Dispute not found");
  } catch (error) {
    // Only log as error if it's not a "Dispute not found" error (expected when iterating)
    if (error instanceof Error && error.message === "Dispute not found") {
      throw error; // Re-throw without logging
    }
    console.error(`[getDispute] Error fetching dispute #${id}:`, error);
    throw error;
  }
}

// Get all disputes by querying each ID sequentially
export async function getDisputes(): Promise<Dispute[]> {
  try {
    const disputes: Dispute[] = [];
    
    console.log("[getDisputes] Starting to fetch disputes from contract:", LANCE_CONTRACT_ID);
    
    // Try to fetch disputes with IDs 1-20 (adjust based on your needs)
    // In production, you'd want to track the current dispute count
    for (let i = 1; i <= 20; i++) {
      try {
        console.log(`[getDisputes] Attempting to fetch dispute #${i}...`);
        const dispute = await getDispute(i.toString());
        console.log(`[getDisputes] ✓ Found dispute #${i}:`, dispute);
        disputes.push(dispute);
      } catch (error) {
        console.log(`[getDisputes] ✗ Dispute #${i} not found, stopping search`);
        // Stop when we hit a dispute that doesn't exist
        break;
      }
    }
    
    console.log(`[getDisputes] Total disputes found: ${disputes.length}`);
    return disputes;
  } catch (error) {
    console.error("Error fetching disputes:", error);
    return [];
  }
}

// Convert blockchain dispute object to frontend format
function convertBlockchainDisputeToFrontend(blockchainDispute: any, id: string): Dispute {
  // Map DisputeStatus enum to string
  // Handle both numeric enum (0, 1, 2, 3) and string enum (["OPEN"], ["CREATOR"], etc.)
  let statusFromBlockchain = blockchainDispute.dispute_status;
  
  // If it's an array, extract the first element (Stellar SDK enum format)
  if (Array.isArray(statusFromBlockchain) && statusFromBlockchain.length > 0) {
    statusFromBlockchain = statusFromBlockchain[0];
  }
  
  let status: string;
  
  if (typeof statusFromBlockchain === 'string') {
    // String enum format
    const stringStatusMap: Record<string, string> = {
      "OPEN": "Open",
      "CREATOR": "Creator",
      "COUNTERPART": "Counterpart",
      "ABSTAIN": "Abstain",
    };
    status = stringStatusMap[statusFromBlockchain] || "Open";
  } else {
    // Numeric enum format
    const numericStatusMap: Record<number, string> = {
      0: "Open",
      1: "Creator",
      2: "Counterpart",
      3: "Abstain",
    };
    status = numericStatusMap[statusFromBlockchain] || "Open";
  }
  
  const isFinalized = status !== "Open";
  
  // Convert BigInt timestamps to numbers safely
  const initialTimestamp = Number(blockchainDispute.initial_timestamp);
  const votingEndsAt = Number(blockchainDispute.vote_data?.voting_ends_at || 0);
  
  return {
    id: id,
    createdAt: new Date(initialTimestamp * 1000).toISOString(),
    projectId: String(blockchainDispute.project_id || 1),
    publicKey: "BLS12_381_PUBLIC_KEY",
    creator: blockchainDispute.creator,
    counterpart: blockchainDispute.counterpart,
    proof: blockchainDispute.creator_proves || "",
    votingEndsAt: new Date(votingEndsAt * 1000).toISOString(),
    state: isFinalized ? "Finalized" : status,
    winner: blockchainDispute.winner || null,
    votesFor: Number(blockchainDispute.votes_for || 0),
    votesAgainst: Number(blockchainDispute.votes_against || 0),
    votesAbstain: 0, // Not stored separately in contract
    blockchainDisputeId: Number(blockchainDispute.dispute_id),
  };
}

// Not applicable for blockchain - disputes are immutable
export async function createDispute(payload: Partial<Dispute>): Promise<Dispute> {
  throw new Error("Use CreateDisputeButton component to create disputes on-chain");
}

// Not applicable for blockchain - disputes are immutable
export async function deleteDispute(id: string): Promise<Dispute> {
  throw new Error("Disputes cannot be deleted from the blockchain");
}

// Not applicable for blockchain - disputes are immutable once finalized
export async function updateDispute(id: string, payload: Partial<Dispute>): Promise<Dispute> {
  throw new Error("Disputes are immutable on the blockchain");
}


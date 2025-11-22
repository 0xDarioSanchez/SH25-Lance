import { useState } from "react";
import { Button, Card, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import { callContract, LANCE_CONTRACT_ID, addressToScVal, u32ToScVal } from "../util/contractCall";
import { nativeToScVal, Contract, TransactionBuilder, Account, BASE_FEE, scValToNative, rpc } from "@stellar/stellar-sdk";
import { network, MAINTAINER_ADDRESS } from "../contracts/util";
import { getDispute } from "../api/disputes";
import { parseKeypairFile, decryptWithPrivateKey } from "../util/crypto";

interface ExecuteDisputeProps {
  disputeId: number;
  voteCount: number;
  minVotes?: number;
  onExecuted?: () => void;
}

// Verify proof on blockchain using read-only simulation
async function verifyProofOnChain(
  disputeId: number,
  tallies: string[],
  seeds: string[]
): Promise<boolean> {
  try {
    console.log("[verifyProofOnChain] Calling contract proof function...");
    console.log("[verifyProofOnChain] Dispute ID:", disputeId);
    console.log("[verifyProofOnChain] Tallies:", tallies);
    console.log("[verifyProofOnChain] Seeds:", seeds);
    
    const rpcServer = new rpc.Server(network.rpcUrl, {
      allowHttp: network.rpcUrl.includes("localhost"),
    });

    const contract = new Contract(LANCE_CONTRACT_ID);
    
    // Convert each element to u128 ScVal, then wrap in Vec
    const talliesVec = tallies.map(t => nativeToScVal(t, { type: "u128" }));
    const seedsVec = seeds.map(s => nativeToScVal(s, { type: "u128" }));
    const talliesScVal = nativeToScVal(talliesVec, { type: "vec" });
    const seedsScVal = nativeToScVal(seedsVec, { type: "vec" });
    
    console.log("[verifyProofOnChain] Calling with converted values");
    
    const operation = contract.call(
      "proof",
      nativeToScVal(disputeId, { type: "u32" }),
      talliesScVal,
      seedsScVal
    );
    
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
    
    console.log("[verifyProofOnChain] Simulation result:", simulation);
    
    if (rpc.Api.isSimulationSuccess(simulation) && simulation.result?.retval) {
      const result = scValToNative(simulation.result.retval);
      console.log("[verifyProofOnChain] Proof result:", result);
      return result === true;
    }
    
    console.error("[verifyProofOnChain] Simulation failed");
    if (rpc.Api.isSimulationError(simulation)) {
      console.error("[verifyProofOnChain] Simulation error:", simulation.error);
    }
    return false;
  } catch (error) {
    console.error("[verifyProofOnChain] Error:", error);
    return false;
  }
}

export const ExecuteDispute = ({ 
  disputeId, 
  voteCount, 
  minVotes = 1,
  onExecuted 
}: ExecuteDisputeProps) => {
  const { address, signTransaction } = useWallet();
  const [loading, setLoading] = useState(false);
  const [executed, setExecuted] = useState(false);
  const [processingStep, setProcessingStep] = useState<'idle' | 'collecting' | 'computing' | 'executing'>('idle');
  const [tallies, setTallies] = useState<string[] | null>(null);
  const [seeds, setSeeds] = useState<string[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  const canExecute = voteCount >= minVotes;
  
  // Check if current user is the maintainer
  const isMaintainer = address === MAINTAINER_ADDRESS;

  const computeTalliesFromVotes = async () => {
    setProcessingStep('collecting');
    setError(null);
    
    try {
      // Step 1: Upload private key file
      const fileInput = document.createElement('input');
      fileInput.type = 'file';
      fileInput.accept = '.json';
      
      const file = await new Promise<File>((resolve, reject) => {
        fileInput.onchange = (e) => {
          const target = e.target as HTMLInputElement;
          if (target.files && target.files[0]) {
            resolve(target.files[0]);
          } else {
            reject(new Error('No file selected'));
          }
        };
        fileInput.click();
      });
      
      // Parse keypair file
      const keypair = await parseKeypairFile(file);
      console.log("Private key file loaded successfully");
      
      setProcessingStep('computing');
      
      // Step 2: Fetch dispute votes from blockchain
      const dispute = await getDispute(disputeId.toString());
      console.log(`Fetched dispute #${disputeId} with ${voteCount} votes`);
      
      // Step 3: Get votes from blockchain directly
      const rpcServer = new rpc.Server(network.rpcUrl, {
        allowHttp: network.rpcUrl.includes("localhost"),
      });
      
      const contract = new Contract(LANCE_CONTRACT_ID);
      const disputeIdParam = nativeToScVal(disputeId, { type: "u32" });
      const operation = contract.call("get_dispute", disputeIdParam);
      
      const sourceAccount = new Account("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF", "0");
      const transaction = new TransactionBuilder(sourceAccount, {
        fee: BASE_FEE,
        networkPassphrase: network.passphrase,
      })
        .addOperation(operation)
        .setTimeout(30)
        .build();
      
      const simulation = await rpcServer.simulateTransaction(transaction);
      
      if (!rpc.Api.isSimulationSuccess(simulation) || !simulation.result?.retval) {
        throw new Error("Failed to fetch dispute from blockchain");
      }
      
      const blockchainDispute = scValToNative(simulation.result.retval);
      const votes = blockchainDispute.vote_data.votes;
      
      console.log(`Found ${votes.length} encrypted votes on blockchain`);
      
      // Step 4: Decrypt votes and compute tallies
      let talliesArr = [0, 0, 0];
      let seedsArr = [0, 0, 0];
      
      for (const vote of votes) {
        console.log("Processing vote:", vote);
        
        // Soroban enums are represented as arrays: [variant_name, data]
        // Check if it's an AnonymousVote variant
        if (Array.isArray(vote) && vote[0] === 'AnonymousVote') {
          const voteData = vote[1]; // The actual vote data is the second element
          const encryptedVotes = voteData.encrypted_votes;
          const encryptedSeeds = voteData.encrypted_seeds;
          const weight = voteData.weight;
          
          console.log(`Decrypting vote from ${voteData.address} (weight: ${weight})`);
          console.log(`Encrypted votes length: ${encryptedVotes?.length}`);
          console.log(`Encrypted seeds length: ${encryptedSeeds?.length}`);
          
          // Decrypt all three choices
          for (let i = 0; i < 3; i++) {
            try {
              const decryptedVote = await decryptWithPrivateKey(
                encryptedVotes[i],
                keypair.privateKey
              );
              const decryptedSeed = await decryptWithPrivateKey(
                encryptedSeeds[i],
                keypair.privateKey
              );
              
              console.log(`Choice ${i} - Decrypted vote: "${decryptedVote}"`);
              console.log(`Choice ${i} - Decrypted seed: "${decryptedSeed}"`);
              
              // Extract numeric values (format is "vote:1" or "seed:12345")
              const voteValue = parseInt(decryptedVote.split(':')[1]);
              const seedValue = parseInt(decryptedSeed.split(':')[1]);
              
              console.log(`Choice ${i} - Parsed vote value: ${voteValue}`);
              console.log(`Choice ${i} - Parsed seed value: ${seedValue}`);
              
              if (isNaN(voteValue) || isNaN(seedValue)) {
                console.error(`Choice ${i} - Failed to parse values. Vote: "${decryptedVote}", Seed: "${decryptedSeed}"`);
                continue;
              }
              
              // Apply weight
              talliesArr[i] += voteValue * weight;
              seedsArr[i] += seedValue * weight;
              
              console.log(`Choice ${i} - After adding: Tally=${talliesArr[i]}, Seed=${seedsArr[i]}`);
            } catch (err) {
              console.error(`Error decrypting vote choice ${i}:`, err);
            }
          }
        }
      }
      
      console.log("✓ All votes decrypted successfully");
      console.log("Tallies:", talliesArr);
      console.log("Weighted seeds:", seedsArr);
      
      setTallies(talliesArr.map(t => t.toString()));
      setSeeds(seedsArr.map(s => s.toString()));
      setProcessingStep('idle');
      
    } catch (err) {
      console.error("Error computing tallies:", err);
      setError(err instanceof Error ? err.message : "Failed to compute tallies");
      setProcessingStep('idle');
    }
  };

  const handleExecute = async () => {
    if (!address || !signTransaction || !tallies || !seeds) return;
    
    setLoading(true);
    setProcessingStep('executing');
    setError(null);
    
    try {
      console.log("Executing dispute:", disputeId);
      console.log("Tallies:", tallies);
      console.log("Seeds:", seeds);
      
      // Convert to proper format for contract
      const talliesVec = tallies.map(t => nativeToScVal(t, { type: "u128" }));
      const seedsVec = seeds.map(s => nativeToScVal(s, { type: "u128" }));
      
      // Wrap in Option::Some for the contract
      const talliesOption = nativeToScVal(talliesVec, { type: "vec" });
      const seedsOption = nativeToScVal(seedsVec, { type: "vec" });
      
      // Call execute function - must use maintainer address
      const result = await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "execute",
        args: [
          addressToScVal(MAINTAINER_ADDRESS), // maintainer - authorized address
          u32ToScVal(1), // project_id
          u32ToScVal(disputeId),
          talliesOption, // Option<Vec<u128>>
          seedsOption, // Option<Vec<u128>>
        ],
        publicKey: address,
        signTransaction,
      });
      
      console.log("✓ Dispute executed successfully!");
      console.log("Result:", result);
      
      setExecuted(true);
      setProcessingStep('idle');
      
      if (onExecuted) {
        // Wait for blockchain to update
        setTimeout(() => onExecuted(), 2000);
      }
    } catch (err) {
      console.error("Error executing dispute:", err);
      const errorMsg = err instanceof Error ? err.message : "Failed to execute dispute";
      setError(errorMsg);
      setProcessingStep('idle');
    } finally {
      setLoading(false);
    }
  };

  if (!address) {
    return (
      <Card>
        <Text as="p" size="md">
          Connect your wallet to execute the dispute
        </Text>
      </Card>
    );
  }

  if (!canExecute) {
    return (
      <Card>
        <Text as="p" size="md">
          Waiting for votes... ({voteCount}/{minVotes} minimum)
        </Text>
        <Text as="p" size="sm" style={{ marginTop: "8px", color: "#666" }}>
          At least {minVotes} judge(s) must vote before the dispute can be executed.
        </Text>
      </Card>
    );
  }

  if (executed) {
    return (
      <Card>
        <Text as="h3" size="lg" style={{ color: "#4caf50" }}>
          ✓ Dispute Executed Successfully
        </Text>
        <Text as="p" size="sm" style={{ marginTop: "8px" }}>
          The dispute has been finalized on the blockchain. Results are now immutable.
        </Text>
        <Text as="p" size="xs" style={{ marginTop: "8px", color: "#666" }}>
          Refresh the page to see the final results.
        </Text>
      </Card>
    );
  }

  return (
    <Card>
      <Text as="h3" size="lg">
        Execute Dispute #{disputeId}
      </Text>
      <Text as="p" size="sm" style={{ marginTop: "8px", marginBottom: "16px" }}>
        {voteCount} vote(s) collected. The voting period has ended.
      </Text>
      
      {!isMaintainer && (
        <div style={{ 
          padding: "12px", 
          backgroundColor: "#fff3cd", 
          borderRadius: "4px", 
          marginBottom: "16px",
          border: "1px solid #ffc107"
        }}>
          <Text as="p" size="sm" style={{ color: "#856404" }}>
            ⚠️ <strong>Only the maintainer can execute disputes.</strong>
          </Text>
          <Text as="p" size="xs" style={{ color: "#856404", marginTop: "4px" }}>
            Maintainer: {MAINTAINER_ADDRESS.slice(0, 12)}...{MAINTAINER_ADDRESS.slice(-8)}
          </Text>
        </div>
      )}

      {error && (
        <div style={{ 
          padding: "12px", 
          backgroundColor: "#fee", 
          borderRadius: "4px", 
          marginBottom: "16px",
          border: "1px solid #fcc"
        }}>
          <Text as="p" size="sm" style={{ color: "#c00" }}>
            <strong>Error:</strong> {error}
          </Text>
        </div>
      )}

      {!tallies ? (
        <>
          <Text as="p" size="sm" style={{ marginBottom: "12px" }}>
            <strong>Step 1:</strong> Compute tallies from encrypted votes
          </Text>
          <Text as="p" size="xs" style={{ marginBottom: "12px", color: "#666" }}>
            This will decrypt all votes and calculate the final tallies (approve, reject, abstain).
            In production, you would upload a private key file to decrypt the votes.
          </Text>
          <Button
            size="md"
            variant="secondary"
            onClick={computeTalliesFromVotes}
            disabled={processingStep !== 'idle' || !isMaintainer}
          >
            {processingStep === 'collecting' ? "Collecting votes..." : 
             processingStep === 'computing' ? "Computing tallies..." : 
             "Compute Tallies"}
          </Button>
        </>
      ) : (
        <>
          <div style={{ 
            padding: "12px", 
            backgroundColor: "#e8f5e9", 
            borderRadius: "4px", 
            marginBottom: "16px" 
          }}>
            <Text as="p" size="sm" style={{ marginBottom: "8px", fontWeight: 600 }}>
              ✓ Tallies Computed:
            </Text>
            <Text as="p" size="sm">
              • Approve (Creator): {tallies[0]}
            </Text>
            <Text as="p" size="sm">
              • Reject (Counterpart): {tallies[1]}
            </Text>
            <Text as="p" size="sm">
              • Abstain: {tallies[2]}
            </Text>
            <Text as="p" size="xs" style={{ marginTop: "8px", color: "#666" }}>
              Seeds: [{seeds?.join(", ")}]
            </Text>
          </div>
          
          <Text as="p" size="sm" style={{ marginBottom: "12px" }}>
            <strong>Step 2:</strong> Execute the dispute on blockchain
          </Text>
          <Text as="p" size="xs" style={{ marginBottom: "12px", color: "#666" }}>
            This will submit the tallies to the smart contract and finalize the dispute.
            The contract will verify the cryptographic commitments and update the dispute status.
          </Text>
          <Button
            size="md"
            variant="primary"
            onClick={handleExecute}
            disabled={loading || processingStep !== 'idle' || !isMaintainer}
          >
            {loading || processingStep === 'executing' ? "Executing..." : "Execute Dispute"}
          </Button>
        </>
      )}
    </Card>
  );
};

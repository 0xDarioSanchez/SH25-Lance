import { useState, useEffect } from "react";
import { Button, Card, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import { callContract, LANCE_CONTRACT_ID, addressToScVal, u32ToScVal, vecU128ToScVal } from "../util/contractCall";
import { nativeToScVal, Contract, TransactionBuilder, Account, BASE_FEE, scValToNative, xdr } from "@stellar/stellar-sdk";
import { rpc } from "@stellar/stellar-sdk";
import { network } from "../contracts/util";
import { encryptWithPublicKey } from "../util/crypto";

interface VotingPanelProps {
  disputeId: number;
  onVoteCast?: () => void;
}

export const VotingPanel = ({ disputeId, onVoteCast }: VotingPanelProps) => {
  const { address, signTransaction } = useWallet();
  const [registered, setRegistered] = useState(false);
  const [voted, setVoted] = useState(false);
  const [loading, setLoading] = useState(false);
  const [checkingStatus, setCheckingStatus] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [projectId, setProjectId] = useState<number>(1); // Store project_id from dispute
  const [publicKey, setPublicKey] = useState<string | null>(null); // Store public key for encryption
  
  // Vote choice: 'approve', 'reject', or 'abstain'
  const [voteChoice, setVoteChoice] = useState<'approve' | 'reject' | 'abstain' | null>(null);
  const [weight] = useState("3"); // Fixed weight, not editable by user

  // Check registration status on mount
  useEffect(() => {
    const checkStatus = async () => {
      if (!address) {
        setCheckingStatus(false);
        return;
      }

      try {
        // Query blockchain directly for dispute details
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
        
        if (rpc.Api.isSimulationSuccess(simulation) && simulation.result?.retval) {
          const dispute = scValToNative(simulation.result.retval);
          
          // Store project_id for later use in voting
          setProjectId(dispute.project_id);
          
          // Get public key from anonymous voting config
          try {
            const configOperation = contract.call(
              "get_anonymous_voting_config",
              u32ToScVal(dispute.project_id)
            );
            const configTx = new TransactionBuilder(sourceAccount, {
              fee: BASE_FEE,
              networkPassphrase: network.passphrase,
            })
              .addOperation(configOperation)
              .setTimeout(30)
              .build();
            
            const configSim = await rpcServer.simulateTransaction(configTx);
            if (rpc.Api.isSimulationSuccess(configSim) && configSim.result?.retval) {
              const config = scValToNative(configSim.result.retval);
              setPublicKey(config.public_key);
            }
          } catch (err) {
            console.error("Error fetching public key:", err);
          }
          
          // Check if already registered to vote
          const isRegistered = dispute.able_to_vote.some(
            (voter: string) => voter === address
          );
          setRegistered(isRegistered);

          // Check if already voted (check vote_data.votes)
          const hasVoted = dispute.vote_data.votes.some(
            (vote: any) => vote.AnonymousVote?.address === address
          );
          setVoted(hasVoted);
        }
      } catch (err) {
        console.error("Error checking registration status:", err);
      } finally {
        setCheckingStatus(false);
      }
    };

    checkStatus();
  }, [address, disputeId]);

  const handleRegister = async () => {
    if (!address || !signTransaction) return;
    
    setLoading(true);
    setError(null);
    try {
      await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "register_to_vote",
        args: [
          addressToScVal(address),
          u32ToScVal(disputeId),
        ],
        publicKey: address,
        signTransaction,
      });
      
      setRegistered(true);
    } catch (error) {
      console.error("Error registering to vote:", error);
      const errorMsg = error instanceof Error ? error.message : "Failed to register";
      
      // Check if the error is about account not found (not registered as judge)
      if (errorMsg.includes("Account not found") || errorMsg.includes("not found")) {
        setError("You must register as a judge first. Go back to the home page and click 'Register as Judge'.");
      } else {
        setError(errorMsg);
      }
    } finally {
      setLoading(false);
    }
  };

  const generateRandomU128 = (): string => {
    // Generate random u128 value using crypto API
    const array = new Uint32Array(4);
    crypto.getRandomValues(array);
    // Combine into a big number (simplified - for production use BigInt properly)
    return array[0].toString();
  };

  const handleVote = async () => {
    if (!address || !registered || !voteChoice || !signTransaction) {
      console.log("Vote blocked - missing requirements:", {
        address: !!address,
        registered,
        voteChoice,
        signTransaction: !!signTransaction,
        publicKey: !!publicKey
      });
      return;
    }
    
    if (!publicKey) {
      setError("Public key not loaded. Please refresh the page or ensure anonymous voting is set up for this project.");
      return;
    }
    
    setLoading(true);
    setError(null);
    try {
      console.log("Starting vote submission...");
      
      // Generate random seeds
      const seed1 = generateRandomU128();
      const seed2 = generateRandomU128();
      const seed3 = generateRandomU128();
      
      // Calculate votes based on choice
      const weightNum = parseInt(weight);
      let votes: [string, string, string];
      
      if (voteChoice === 'approve') {
        votes = ["1", "0", "0"];  // Unweighted - weight applied during proof
      } else if (voteChoice === 'reject') {
        votes = ["0", "1", "0"];
      } else { // abstain
        votes = ["0", "0", "1"];
      }
      
      console.log("Encrypting votes and seeds with public key...");
      
      // Encrypt votes and seeds with RSA public key
      const encryptedVotes = await Promise.all(
        votes.map(v => encryptWithPublicKey(`vote:${v}`, publicKey))
      );
      const encryptedSeeds = await Promise.all(
        [seed1, seed2, seed3].map(s => encryptWithPublicKey(`seed:${s}`, publicKey))
      );
      
      // Calculate tallies for logging (user will need private key to decrypt later)
      const tallies = votes.map(v => (parseInt(v) * weightNum).toString());
      const weightedSeeds = [seed1, seed2, seed3].map(s => parseInt(s) * weightNum);
      
      console.log("========================================");
      console.log("⚠️  VOTES AND SEEDS ARE NOW ENCRYPTED");
      console.log("You will need the private key file to decrypt and execute this dispute.");
      console.log(`Unweighted votes: [${votes.join(", ")}]`);
      console.log(`Weight: ${weightNum}`);
      console.log(`Expected tallies after decryption: [${tallies.join(", ")}]`);
      console.log(`Expected weighted seeds after decryption: [${weightedSeeds.join(", ")}]`);
      console.log("========================================");
      
      // Step 1: Build commitments from votes and seeds (uses project_id, not dispute_id)
      const commitments = await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "build_commitments_from_votes",
        args: [
          u32ToScVal(projectId),
          vecU128ToScVal([votes[0], votes[1], votes[2]]),
          vecU128ToScVal([seed1, seed2, seed3]),
        ],
        publicKey: address,
        signTransaction,
      });
      
      console.log("Commitments built:", commitments);
      
      // Step 2: Call vote with the commitments and encrypted values
      // Build AnonymousVote variant manually
      const anonymousVoteStruct = xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("address"),
          val: addressToScVal(address)
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("commitments"),
          val: nativeToScVal(commitments)
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("encrypted_seeds"),
          val: nativeToScVal(encryptedSeeds)
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("encrypted_votes"),
          val: nativeToScVal(encryptedVotes)
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("weight"),
          val: nativeToScVal(weightNum, { type: "u32" })
        }),
      ]);

      // Wrap in Vote2::AnonymousVote enum
      const vote2Enum = xdr.ScVal.scvVec([
        xdr.ScVal.scvSymbol("AnonymousVote"),
        anonymousVoteStruct
      ]);
      
      await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "vote",
        args: [
          addressToScVal(address),
          u32ToScVal(disputeId),
          vote2Enum,
        ],
        publicKey: address,
        signTransaction,
      });
      
      setVoted(true);
      if (onVoteCast) onVoteCast();
    } catch (error) {
      console.error("Error casting vote:", error);
      setError(error instanceof Error ? error.message : "Failed to cast vote");
    } finally {
      setLoading(false);
    }
  };

  if (!address) {
    return (
      <Card>
        <Text as="p" size="md">
          Connect your wallet to participate in voting
        </Text>
      </Card>
    );
  }

  if (checkingStatus) {
    return (
      <Card>
        <Text as="p" size="md">
          Checking your registration status...
        </Text>
      </Card>
    );
  }

  if (voted) {
    return (
      <Card>
        <Text as="p" size="md" style={{ color: "green" }}>
          ✓ Your vote has been cast successfully
        </Text>
      </Card>
    );
  }

  if (!registered) {
    return (
      <Card>
        <Text as="h3" size="lg">
          Register to Vote
        </Text>
        <Text as="p" size="sm" style={{ marginBottom: "16px" }}>
          Register as a judge for dispute #{disputeId}
        </Text>
        {error && (
          <Text as="p" size="sm" style={{ color: "red", marginBottom: "12px" }}>
            Error: {error}
          </Text>
        )}
        <Button
          size="md"
          variant="primary"
          onClick={handleRegister}
          disabled={loading}
        >
          {loading ? "Registering..." : "Register to Vote"}
        </Button>
      </Card>
    );
  }

  return (
    <Card>
      <Text as="h3" size="lg">
        Cast Your Vote
      </Text>
      <Text as="p" size="sm" style={{ marginBottom: "16px" }}>
        Enter your anonymous vote data for dispute #{disputeId}
      </Text>

      <div style={{ marginBottom: "12px" }}>
        <Text as="span" size="sm" style={{ fontWeight: 600 }}>
          Weight: {weight}
        </Text>
        <Text as="p" size="xs" style={{ color: "#666", marginTop: "4px" }}>
          Your vote will be multiplied by this weight
        </Text>
      </div>

      <Text as="span" size="sm" style={{ fontWeight: 600, marginTop: "12px" }}>
        Choose your vote:
      </Text>
      <div style={{ display: "flex", gap: "8px", marginTop: "8px", marginBottom: "16px" }}>
        <Button
          size="md"
          variant={voteChoice === 'approve' ? 'primary' : 'secondary'}
          onClick={() => setVoteChoice('approve')}
        >
          Approve
        </Button>
        <Button
          size="md"
          variant={voteChoice === 'reject' ? 'primary' : 'secondary'}
          onClick={() => setVoteChoice('reject')}
        >
          Reject
        </Button>
        <Button
          size="md"
          variant={voteChoice === 'abstain' ? 'primary' : 'secondary'}
          onClick={() => setVoteChoice('abstain')}
        >
          Abstain
        </Button>
      </div>

      <Text as="p" size="xs" style={{ color: "#666", marginBottom: "16px" }}>
        Random seeds will be generated automatically for privacy
      </Text>
      
      {!publicKey && (
        <div style={{ 
          padding: "12px", 
          backgroundColor: "#fff3cd", 
          borderRadius: "4px", 
          marginBottom: "16px",
          border: "1px solid #ffc107"
        }}>
          <Text as="p" size="sm" style={{ color: "#856404" }}>
            ⚠️ Public key not loaded. Anonymous voting may not be set up for this project yet.
            The maintainer needs to run "Anonymous Voting Setup" first.
          </Text>
        </div>
      )}

      {error && (
        <Text as="p" size="sm" style={{ color: "red", marginBottom: "12px" }}>
          Error: {error}
        </Text>
      )}

      <Button
        size="md"
        variant="primary"
        onClick={handleVote}
        disabled={loading || !voteChoice || !publicKey}
      >
        {loading ? "Submitting Vote..." : "Submit Vote"}
      </Button>
    </Card>
  );
};

import { useState } from "react";
import { Button, Card, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import { callContract, LANCE_CONTRACT_ID, addressToScVal, u32ToScVal } from "../util/contractCall";
import { nativeToScVal } from "@stellar/stellar-sdk";
import { generateRSAKeyPair, downloadKeypairFile } from "../util/crypto";
import { MAINTAINER_ADDRESS } from "../contracts/util";

interface AnonymousVotingSetupProps {
  projectId: number;
  onSetupComplete?: () => void;
}

export const AnonymousVotingSetup = ({ projectId, onSetupComplete }: AnonymousVotingSetupProps) => {
  const { address, signTransaction } = useWallet();
  const isMaintainer = address === MAINTAINER_ADDRESS;
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [setupComplete, setSetupComplete] = useState(false);
  const [generatedKeys, setGeneratedKeys] = useState<{ publicKey: string; privateKey: string } | null>(null);

  const handleSetup = async () => {
    if (!address || !signTransaction) return;

    setLoading(true);
    setError(null);

    try {
      // Step 1: Generate RSA keypair
      console.log("Generating RSA keypair for anonymous voting...");
      const keys = await generateRSAKeyPair();
      setGeneratedKeys(keys);

      console.log("Public key generated (will be stored on blockchain)");
      console.log("Private key generated (will be downloaded - KEEP IT SAFE!)");

      // Step 2: Store public key on blockchain
      await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "anonymous_voting_setup",
        args: [
          addressToScVal(address),
          u32ToScVal(projectId),
          nativeToScVal(keys.publicKey, { type: "string" }),
        ],
        publicKey: address,
        signTransaction,
      });

      // Step 3: Download the keypair file
      downloadKeypairFile(keys.publicKey, keys.privateKey, projectId);

      setSetupComplete(true);
      if (onSetupComplete) {
        onSetupComplete();
      }

      console.log("✓ Anonymous voting setup complete!");
      console.log("✓ Private key file downloaded");
    } catch (err) {
      console.error("Error setting up anonymous voting:", err);
      setError(err instanceof Error ? err.message : "Failed to setup anonymous voting");
    } finally {
      setLoading(false);
    }
  };

  const handleDownloadAgain = () => {
    if (generatedKeys) {
      downloadKeypairFile(generatedKeys.publicKey, generatedKeys.privateKey, projectId);
    }
  };

  // Don't show to non-maintainers
  if (!isMaintainer) {
    return null;
  }

  if (!address) {
    return null;
  }

  if (setupComplete) {
    return (
      <Card>
        <div style={{ 
          padding: "12px", 
          backgroundColor: "#e3f2fd", 
          borderRadius: "4px", 
          marginBottom: "16px",
          border: "1px solid #2196f3"
        }}>
          <Text as="p" size="sm" style={{ color: "#1565c0", fontWeight: 600 }}>
            🔑 Maintainer Admin Panel
          </Text>
        </div>
        
        <Text as="h3" size="lg" style={{ color: "#4caf50", marginBottom: "1rem" }}>
          ✓ Anonymous Voting Setup Complete
        </Text>
        <Text as="p" size="sm" style={{ marginBottom: "1rem" }}>
          The private key file has been downloaded. You will need this file to decrypt votes and execute disputes.
        </Text>
        <Text as="p" size="sm" style={{ marginBottom: "1rem", color: "#f44336" }}>
          <strong>⚠️ IMPORTANT:</strong> Keep the private key file safe! Without it, votes cannot be decrypted and disputes cannot be executed.
        </Text>
        <Button
          size="sm"
          variant="secondary"
          onClick={handleDownloadAgain}
        >
          Download Key File Again
        </Button>
      </Card>
    );
  }

  return (
    <Card>
      <div style={{ 
        padding: "12px", 
        backgroundColor: "#e3f2fd", 
        borderRadius: "4px", 
        marginBottom: "16px",
        border: "1px solid #2196f3"
      }}>
        <Text as="p" size="sm" style={{ color: "#1565c0", fontWeight: 600 }}>
          🔑 Maintainer Admin Panel
        </Text>
        <Text as="p" size="xs" style={{ color: "#1976d2", marginTop: "4px" }}>
          You are the authorized maintainer. Only you can set up anonymous voting and execute disputes.
        </Text>
      </div>
      
      <Text as="h3" size="lg">
        🔐 Anonymous Voting Setup
      </Text>
      <Text as="p" size="sm" style={{ marginBottom: "1rem", color: "#666" }}>
        Generate encryption keys for anonymous voting on Project #{projectId}. 
        You'll download a private key file that you'll need to decrypt votes later.
      </Text>

      {error && (
        <Text as="p" size="sm" style={{ color: "red", marginBottom: "1rem" }}>
          {error}
        </Text>
      )}

      <Button
        size="md"
        variant="primary"
        onClick={handleSetup}
        disabled={loading}
      >
        {loading ? "Setting up..." : "Generate Keys & Setup"}
      </Button>
    </Card>
  );
};

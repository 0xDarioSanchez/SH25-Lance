import { useState } from "react";
import { Button, Input, Modal, Text, Textarea } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import { StrKey } from "@stellar/stellar-sdk";
import { callContract, LANCE_CONTRACT_ID, addressToScVal, u32ToScVal } from "../util/contractCall";
import { nativeToScVal } from "@stellar/stellar-sdk";

interface Props {
  onCreated?: () => void;
}

export const CreateDisputeButton = ({ onCreated }: Props) => {
  const [showModal, setShowModal] = useState(false);
  const [counterpart, setCounterpart] = useState("");
  const [proof, setProof] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const { address, isPending, signTransaction } = useWallet();
  const isConnected = !!address;

  const isValidStellarAddress = (addr: string): boolean => {
    return StrKey.isValidEd25519PublicKey(addr.trim());
  };

  const counterpartIsValid = isValidStellarAddress(counterpart);

  const handleSubmit = async () => {
    if (!address || !counterpartIsValid || !signTransaction) return;

    setSubmitting(true);
    setError(null);

    try {
      // Calculate voting end time (80 seconds from now)
      const votingEndsAt = Math.floor(Date.now() / 1000) + 80;

      // Call the smart contract's create_dispute function
      // Note: public_key is now set separately via anonymous_voting_setup
      const result = await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "create_dispute",
        args: [
          u32ToScVal(1), // project_id
          addressToScVal(address), // creator
          addressToScVal(counterpart.trim()), // counterpart
          nativeToScVal(proof.trim(), { type: "string" }), // proof
          nativeToScVal(votingEndsAt, { type: "u64" }), // voting_ends_at
          addressToScVal(LANCE_CONTRACT_ID), // called_contract
        ],
        publicKey: address,
        signTransaction,
      });

      // Extract the blockchain dispute_id from the result
      const blockchainDisputeId = result?.dispute_id || 1;
      console.log("✓ Created dispute on blockchain with ID:", blockchainDisputeId);

      if (onCreated) onCreated();

      setShowModal(false);
      setCounterpart("");
      setProof("");
      setError(null);
    } catch (error) {
      console.error("Error creating dispute:", error);
      setError(error instanceof Error ? error.message : "Failed to create dispute");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div id="createDisputeContainer">
      <Modal
        visible={showModal}
        onClose={() => setShowModal(false)}
        parentId="createDisputeContainer"
      >
        <Modal.Heading>Create Dispute</Modal.Heading>

        <Text as="p" size="md" style={{ marginBottom: "10px" }}>
          Fill the parameters required by the smart contract to open a dispute.
        </Text>

        {error && (
          <Text as="p" size="sm" style={{ color: "red", marginBottom: "10px" }}>
            Error: {error}
          </Text>
        )}

        <Text as="span" size="sm" style={{ fontWeight: 600 }}>
          Creator (your wallet)
        </Text>
        <Input
          disabled
          value={address ?? ""}
          style={{ marginTop: "4px", marginBottom: "14px" }}
          id=""
          fieldSize="md"
        />

        <Text as="span" size="sm" style={{ fontWeight: 600 }}>
          Counterpart Address
        </Text>

        <Input
          placeholder="Enter the other party's Stellar address"
          value={counterpart}
          onChange={(e) => setCounterpart(e.target.value)}
          style={{ marginTop: "4px", marginBottom: "6px" }}
          id=""
          fieldSize="md"
        />

        {!counterpartIsValid && counterpart.trim() !== "" && (
          <Text as="div" size="sm" style={{ color: "#d9534f", marginBottom: "14px" }}>
            Invalid Stellar address
          </Text>
        )}

        <Text as="span" size="sm" style={{ fontWeight: 600 }}>
          Proof / Reason for Dispute
        </Text>
        <Textarea
          placeholder="Explain the reason for this dispute"
          value={proof}
          onChange={(e) => setProof(e.target.value)}
          style={{ marginTop: "4px", marginBottom: "20px", height: "120px" }}
          id=""
          fieldSize="md"
        />

        <Modal.Footer itemAlignment="stack">
          <Button
            size="md"
            variant="primary"
            onClick={handleSubmit}
            disabled={
              submitting ||
              !counterpart.trim() ||
              !proof.trim() ||
              !counterpartIsValid
            }
          >
            {submitting ? "Submitting..." : "Submit Dispute"}
          </Button>

          <Button
            size="md"
            variant="tertiary"
            onClick={() => setShowModal(false)}
          >
            Cancel
          </Button>
        </Modal.Footer>
      </Modal>

      <Button
        size="md"
        variant={isConnected ? "primary" : "secondary"}
        disabled={!isConnected || isPending}
        onClick={() => setShowModal(true)}
        style={{ marginTop: "20px" }}
      >
        {isConnected ? "Create Dispute" : "Connect Wallet to Create Dispute"}
      </Button>
    </div>
  );
};

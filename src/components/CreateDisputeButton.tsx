import { useState } from "react";
import { Button, Input, Modal, Text, Textarea } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import { StrKey } from "@stellar/stellar-sdk";
import { createDispute } from "../api/disputes";

interface Props {
  onCreated?: () => void;
}

export const CreateDisputeButton = ({ onCreated }: Props) => {
  const [showModal, setShowModal] = useState(false);
  const [counterpart, setCounterpart] = useState("");
  const [proof, setProof] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const { address, isPending } = useWallet();
  const isConnected = !!address;

  const isValidStellarAddress = (addr: string): boolean => {
    return StrKey.isValidEd25519PublicKey(addr.trim());
  };

  const counterpartIsValid = isValidStellarAddress(counterpart);

  const handleSubmit = async () => {
    if (!address || !counterpartIsValid) return;

    setSubmitting(true);

    const payload = {
      creator: address,
      counterpart: counterpart.trim(),
      proof: proof.trim(),
      state: "pending",
      createdAt: new Date().toISOString(),
    };

    try {
      await createDispute(payload);

      if (onCreated) onCreated();

      setShowModal(false);
      setCounterpart("");
      setProof("");
    } catch (error) {
      console.error("Error creating dispute:", error);
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

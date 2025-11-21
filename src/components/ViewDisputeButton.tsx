import { useState, useEffect } from "react";
import { Button, Modal, Text } from "@stellar/design-system";
import { getDispute } from "../api/disputes";
import type { Dispute } from "../api/types";

interface Props {
  id: string;
  disabled?: boolean;
  variant?: "primary" | "secondary";
}

export const ViewDisputeButton = ({ id, disabled, variant = "primary" }: Props) => {
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(false);
  const [dispute, setDispute] = useState<Dispute | null>(null);

  const loadDispute = async () => {
    setLoading(true);
    try {
      const data = await getDispute(id);
      setDispute(data);
    } catch (err) {
      console.error("Error loading dispute", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (open) {
      loadDispute();
    }
  }, [open]);

  const formatDate = (iso: string | undefined) => {
    if (!iso) return "";
    const date = new Date(iso);
    return date.toISOString().replace("T", " ").replace(".000Z", " UTC");
  };

  return (
    <div id={`viewDispute-${id}`}>
      <Button
        size="sm"
        variant={variant}
        disabled={disabled}
        onClick={() => setOpen(true)}
      >
        View
      </Button>

      <Modal
        visible={open}
        onClose={() => setOpen(false)}
        parentId={`viewDispute-${id}`}
      >
        <Modal.Heading>Dispute Details</Modal.Heading>

        {loading && (
          <Text as="p" size="md">Loading...</Text>
        )}

        {!loading && dispute && (
          <>
            <Text as="p" size="md">
              <strong>ID:</strong> {dispute.id}
            </Text>

            {/* Creator */}
            <Text as="p" size="sm" style={{ wordBreak: "break-all" }}>
              <strong>Creator:</strong><br />
              {dispute.creator}
            </Text>

            {/* Counterpart */}
            <Text as="p" size="sm" style={{ wordBreak: "break-all" }}>
              <strong>Counterpart:</strong><br />
              {dispute.counterpart}
            </Text>

            <Text as="p" size="md">
              <strong>Proof:</strong><br />
              {dispute.proof}
            </Text>

            <Text as="p" size="md">
              <strong>State:</strong><br />
              {dispute.state}
            </Text>

            <Text as="p" size="md">
              <strong>Created At:</strong><br />
              {formatDate(dispute.createdAt)}
            </Text>
          </>
        )}

        <Modal.Footer>
          <Button size="md" variant="primary" onClick={() => setOpen(false)}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
};

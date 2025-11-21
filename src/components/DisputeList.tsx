import { Button, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import type { Dispute } from "../api/types";
import { deleteDispute } from "../api/disputes";
import { ViewDisputeButton } from "../components/ViewDisputeButton";

interface Props {
  disputes: Dispute[];
  loading: boolean;
  onDelete: () => void;
}

export const DisputeList = ({ disputes, loading, onDelete }: Props) => {
  const { address, isPending } = useWallet();
  const isConnected = !!address;

  const handleDelete = async (id: string) => {
    if (!isConnected || isPending) return;

    await deleteDispute(id);
    onDelete();
  };

  if (loading) {
    return <Text as="p" size="md">Loading disputes...</Text>;
  }

  return (
    <div style={{ marginTop: "2rem" }}>
      <Text as="h2" size="lg">Disputes</Text>

      <div style={{ marginTop: "1rem" }}>
        {disputes.map((d) => (
          <div
            key={d.id}
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
              padding: "0.75rem 1rem",
              marginBottom: "0.5rem",
              background: "#f7f8fa",
              borderRadius: "8px",
            }}
          >
            <div>
              <Text as="p" size="md">ID: {d.id}</Text>
              <Text as="p" size="md">State: {d.state}</Text>
            </div>

            <div style={{ display: "flex", gap: "0.5rem" }}>
              <ViewDisputeButton
                id={d.id}
                disabled={!isConnected || isPending}
                variant={isConnected ? "primary" : "secondary"}
              />

              {/* NEW: Close button (conceptual) */}
              <Button
                size="sm"
                variant="tertiary"
                disabled={!isConnected || isPending}
                onClick={() => {}}
              >
                Close
              </Button>

              <Button
                size="sm"
                variant="destructive"
                disabled={!isConnected || isPending}
                onClick={() => handleDelete(d.id)}
              >
                Delete
              </Button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

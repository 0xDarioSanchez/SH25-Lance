import { Button, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import type { Dispute } from "../api/types";
import { ViewDisputeButton } from "../components/ViewDisputeButton";

interface Props {
  disputes: Dispute[];
  loading: boolean;
  onDelete?: () => void;
}

export const DisputeList = ({ disputes, loading }: Props) => {
  const { address, isPending } = useWallet();
  const isConnected = !!address;

  if (loading) {
    return <Text as="p" size="md">Loading disputes from blockchain...</Text>;
  }

  if (disputes.length === 0) {
    return (
      <div style={{ marginTop: "2rem" }}>
        <Text as="h2" size="lg">Disputes</Text>
        <Text as="p" size="md" style={{ marginTop: "1rem", color: "#666" }}>
          No disputes found on the blockchain yet. Create the first one!
        </Text>
      </div>
    );
  }

  return (
    <div style={{ marginTop: "2rem" }}>
      <Text as="h2" size="lg">Disputes on Blockchain</Text>

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
              background: d.state === "Finalized" ? "#e8f5e9" : "#f7f8fa",
              borderRadius: "8px",
              borderLeft: d.state === "Finalized" ? "4px solid #4caf50" : "4px solid #2196f3",
            }}
          >
            <div>
              <Text as="p" size="md">
                <strong>Dispute #{d.blockchainDisputeId || d.id}</strong>
              </Text>
              <Text as="p" size="sm" style={{ color: "#666" }}>
                Creator: {d.creator.substring(0, 8)}...{d.creator.substring(d.creator.length - 4)}
              </Text>
              <Text as="p" size="sm" style={{ color: "#666" }}>
                Counterpart: {d.counterpart.substring(0, 8)}...{d.counterpart.substring(d.counterpart.length - 4)}
              </Text>
              <Text as="p" size="md" style={{ marginTop: "4px" }}>
                Status: <strong>{d.state}</strong>
                {d.state === "Finalized" && d.winner && (
                  <span style={{ marginLeft: "8px", color: "#4caf50", fontWeight: 600 }}>
                    (Winner: {d.winner.substring(0, 8)}...)
                  </span>
                )}
              </Text>
              {d.state === "Finalized" && (
                <Text as="p" size="xs" style={{ color: "#666", marginTop: "4px" }}>
                  Votes: {d.votesFor || 0} for, {d.votesAgainst || 0} against
                </Text>
              )}
            </div>

            <div style={{ display: "flex", gap: "0.5rem" }}>
              <ViewDisputeButton
                id={d.blockchainDisputeId?.toString() || d.id}
                disabled={!isConnected || isPending}
                variant={isConnected ? "primary" : "secondary"}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

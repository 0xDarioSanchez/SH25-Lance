import React, { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import { Layout, Text, Card } from "@stellar/design-system";
import { VotingPanel } from "../components/VotingPanel";
import { ExecuteDispute } from "../components/ExecuteDispute";
import { getDispute } from "../api/disputes";
import type { Dispute } from "../api/types";
import { Contract, nativeToScVal, scValToNative, TransactionBuilder, Account, BASE_FEE, rpc } from "@stellar/stellar-sdk";
import { LANCE_CONTRACT_ID } from "../util/contractCall";
import { network } from "../contracts/util";

const DisputeDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [dispute, setDispute] = useState<Dispute | null>(null);
  const [loading, setLoading] = useState(true);
  const [voteCount, setVoteCount] = useState(0);

  const loadDispute = async () => {
    if (!id) return;
    
    try {
      const data = await getDispute(id);
      setDispute(data);

      // Fetch actual vote count from blockchain
      const rpcServer = new rpc.Server(network.rpcUrl, {
        allowHttp: network.rpcUrl.includes("localhost"),
      });

      const contract = new Contract(LANCE_CONTRACT_ID);
      const disputeIdParam = nativeToScVal(parseInt(id), { type: "u32" });
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
        const blockchainDispute = scValToNative(simulation.result.retval);
        const actualVoteCount = blockchainDispute.vote_data.votes.length;
        setVoteCount(actualVoteCount);
        console.log(`✓ Loaded ${actualVoteCount} votes from blockchain`);
      }
    } catch (err) {
      console.error("Error loading dispute:", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDispute();
  }, [id]);

  if (loading) {
    return (
      <Layout.Content>
        <Layout.Inset>
          <Text as="p" size="md">Loading dispute...</Text>
        </Layout.Inset>
      </Layout.Content>
    );
  }

  if (!dispute) {
    return (
      <Layout.Content>
        <Layout.Inset>
          <Text as="p" size="md">Dispute not found</Text>
        </Layout.Inset>
      </Layout.Content>
    );
  }

  // Check if voting period has ended
  const votingEnded = new Date(dispute.votingEndsAt) <= new Date();
  const isOpen = dispute.state === "Open";

  return (
    <Layout.Content>
      <Layout.Inset>
        <Text as="h1" size="xl">
          Dispute #{dispute.blockchainDisputeId || dispute.id}
        </Text>

        <div style={{ marginTop: "20px", marginBottom: "20px" }}>
          <Card>
            <Text as="h3" size="md">
              Dispute Details
            </Text>
            <div style={{ marginTop: "12px" }}>
              <Text as="p" size="sm">
                <strong>Blockchain ID:</strong> {dispute.blockchainDisputeId || dispute.id}
              </Text>
              <Text as="p" size="sm">
                <strong>Creator:</strong> {dispute.creator}
              </Text>
              <Text as="p" size="sm">
                <strong>Counterpart:</strong> {dispute.counterpart}
              </Text>
              <Text as="p" size="sm">
                <strong>Proof:</strong> {dispute.proof}
              </Text>
              <Text as="p" size="sm">
                <strong>Status:</strong> {dispute.state}
              </Text>
              <Text as="p" size="sm">
                <strong>Created:</strong> {new Date(dispute.createdAt).toLocaleString()}
              </Text>
              <Text as="p" size="sm">
                <strong>Voting Ends:</strong> {new Date(dispute.votingEndsAt).toLocaleString()}
              </Text>
              {dispute.state === "Finalized" && (
                <>
                  <div style={{ marginTop: "12px", padding: "12px", backgroundColor: "#e8f5e9", borderRadius: "4px" }}>
                    <Text as="p" size="sm" style={{ fontWeight: 600, marginBottom: "8px" }}>
                      Final Results (from blockchain):
                    </Text>
                    <Text as="p" size="sm">
                      <strong>Winner:</strong> {dispute.winner || "No winner (tie or majority abstain)"}
                    </Text>
                    <Text as="p" size="sm">
                      <strong>Votes For (Approve):</strong> {dispute.votesFor || 0}
                    </Text>
                    <Text as="p" size="sm">
                      <strong>Votes Against (Reject):</strong> {dispute.votesAgainst || 0}
                    </Text>
                  </div>
                </>
              )}
            </div>
          </Card>
        </div>

        {dispute.state !== "Finalized" && (
          <>
            {/* Show voting panel only when voting is still open */}
            {!votingEnded && isOpen && (
              <div style={{ marginBottom: "20px" }}>
                <VotingPanel
                  disputeId={dispute.blockchainDisputeId || parseInt(id!)}
                  onVoteCast={() => {
                    // Reload dispute to get updated vote count
                    loadDispute();
                  }}
                />
              </div>
            )}

            {/* Show execute only when voting period has ended */}
            {votingEnded && isOpen && (
              <div>
                <ExecuteDispute
                  disputeId={dispute.blockchainDisputeId || parseInt(id!)}
                  voteCount={voteCount}
                  minVotes={1}
                  onExecuted={() => {
                    loadDispute();
                  }}
                />
              </div>
            )}
          </>
        )}
      </Layout.Inset>
    </Layout.Content>
  );
};

export default DisputeDetail;

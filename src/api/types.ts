export interface Dispute {
  id: string;
  createdAt: string;
  projectId: string;
  publicKey: string;
  creator: string;
  counterpart: string;
  proof: string;
  votingEndsAt: string;
  state: string;
  winner?: string | null;
  votesFor?: number;
  votesAgainst?: number;
  votesAbstain?: number;
  blockchainDisputeId?: number; // The actual dispute_id from the smart contract
}

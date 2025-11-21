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
}

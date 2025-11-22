# Lance Protocol

## Day 1 Documentation:

https://docs.google.com/document/d/1-FBoplOi9BM8TADwnrAGp4s9P0spiEyjou3cgHLVBZ4/edit?tab=t.0


# Lance Protocol & Lance Market: Decentralized Voting + Freelance Marketplace

## Problem

Centralized freelance platforms charge high fees (5–20%), impose delays on cross-border payments, and handle disputes with opaque, centralized processes.  
On top of that, developers who want to build trustless reputation or arbitration features must build these systems from scratch, which slows down innovation.

## Solution

We redesigned our system into two independent but interoperable components:

### 1. Lance Protocol
A decentralized voting and dispute-resolution protocol that any Stellar smart contract can plug into.  
It allows any project to outsource trustless voting, commit–reveal arbitration, and ZK-based privacy to a single, universal layer.

Key features:
- Commit–reveal with Zero-Knowledge Proofs  
  Uses BLS12-381 verification (similar to Tansu) for anonymous encrypted votes.
- One-transaction flow enabling private voting and tally verification.
- Integrations for other apps: Smart contracts can request a vote, listen for results, and enforce outcomes.
- Governance logic with incentives based on game theory and Neural Quorum concepts.

### 2. Lance Market
The first real use case built on Lance Protocol:  
a decentralized freelance marketplace with escrow, disputes, and yield-generating locked funds.

Clients post jobs, freelancers deliver work, and any dispute is resolved through the universal voting system.

Benefits:
- Low Fees: Revenue model based on yield through Blend, not charging users.
- Instant Global Payments: USDC/XLM transfers on Stellar are near-free and settle instantly.
- Trustless Escrow: Soroban contracts guarantee unbiased release of funds.
- Transparent & Auditable: All interactions and results are visible on Stellar Explorer.

## Focus of This Hackathon Submission

This submission focuses primarily on Lance Protocol, the reusable private voting system.  
Lance Market is included only as the first integrated example, demonstrating real use of the protocol in disputes.

### What’s new for this hackathon
- The protocol has been fully separated into its own smart contract.
- Any Stellar project can:
  1. Register a dispute or vote event  
  2. Submit encrypted votes  
  3. Trigger ZKP validation and final tally  
  4. Consume the result in their contract logic
- ZKP Voting Implemented using the commit–reveal pattern + BLS12-381 verification (Tansu approach).  
  Voters submit an encrypted commitment; reveal stage uses a zero-knowledge proof to validate the vote without exposing the underlying identity.

## Architecture

### High-Level Components

- Lance Protocol (Soroban)  
  Smart contract handling vote creation, encrypted vote submissions, commit–reveal logic, ZK verification, tallying, and result emission.

- Lance Market (Soroban)  
  Uses the protocol to manage user disputes and enforce final outcomes in freelancer–client agreements.

- Frontend  
  Next.js UI for job posting, dispute triggers, and vote interactions.

- Backend (minimal)  
  Node.js for wallet integration, job metadata, and optional IPFS storage.

- IPFS  
  Stores job descriptions and proofs of work.

- User Auth  
  Freighter wallet for signing transactions and ZK vote submissions.

## Voting Flow (Lance Protocol)

1. A contract (e.g., Lance Market) opens a dispute and registers it in Lance Protocol.  
2. Protocol assigns eligible voters (either governance members or NQG-based randomly selected jurors).  
3. Voters submit an encrypted commitment.  
4. Reveal stage:  
   - Voters submit a BLS12-381 zero-knowledge proof validating the vote.  
   - The vote is tallied without exposing private data.  
5. Lance Protocol stores the final result.  
6. The calling contract enforces the decision (e.g., release escrow or refund employer).

## Stellar Components

- Soroban for Lance Protocol (ZK verification + commit–reveal logic)  
- Soroban for Lance Market (escrow, disputes, job lifecycle)  
- Stellar SDK for wallet integration, viewing results, and contract calls  
- Freighter for transaction signing  
- Stellar Assets such as USDC and XLM  
- Blend for yield on locked escrow funds  
- Stellar Explorer for transparent vote auditability

## Installation

### Prerequisites
- Node.js (v16+)
- Rust + Soroban CLI
- Freighter Wallet
- Git + npm  
- wasm32 build target

### Steps

chmod +x ./accounts.sh  
chmod +x ./run.sh

Run once:
./accounts.sh

Build and execute:
./run.sh

## Front-End

Demo:  
https://lancestellar.vercel.app/

Source:  
https://github.com/artugrande/lance-front/

## Tests

cargo test

Covers:
- ZKP verification logic  
- Commit and reveal lifecycle  
- Dispute registration  
- Tally correctness  
- Unauthorized access prevention  
- Escrow deposit/withdraw with Blend  

## Security Features

- ZKP-based private voting using BLS12-381  
- Commit–reveal prevents early leakage  
- Immutable vote records stored on-chain  
- Access control for dispute initiators  
- Replay attack prevention with unique vote IDs  
- Open-source tally logic for auditability  

## Roadmap

### Completed  
- Split architecture into Lance Protocol + Lance Market  
- Implement commit–reveal private voting  
- Integrate ZKP verification using BLS12-381  
- On-chain reputation and NQG-inspired incentive system  
- Blend integration for yield on locked funds  
- Full front-end demo  

### Upcoming  
- Add proof-of-personhood optional module  
- Improve vote registry for external dApps  
- Cross-contract callbacks for automatic enforcement  
- Expanded reputation scoring  
- Add more use cases beyond freelance disputes  
- Mobile app interface  
- Multi-wallet support  
- Full documentation and tutorial videos  

## Screenshots

### Front-End
![frontend](images/frontend.png "Front-End")
### Running script 1
![script](images/script1.png "Running script 1")
### Running script 2
![script](images/script2.png "Running script 2")
### Public Functions
![public_functions](images/public_functions.png "Public Functions")
### Transactions
![transactions](images/transactions.png "Transactions")
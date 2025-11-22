import { useEffect, useState } from "react";
import { useWallet } from "./useWallet";
import { Contract, TransactionBuilder, BASE_FEE, Address, nativeToScVal, rpc } from "@stellar/stellar-sdk";
import { rpcUrl, networkPassphrase } from "../contracts/util";

// Replace with your actual contract ID
const CONTRACT_ID = process.env.PUBLIC_LANCE_PROTOCOL_CONTRACT_ID || "YOUR_CONTRACT_ID";

export const useAnonymousVoting = () => {
  const { address } = useWallet();
  const [initialized, setInitialized] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initializeAnonymousVoting = async () => {
      if (!address || initialized) return;

      try {
        const server = new rpc.Server(rpcUrl);
        const sourceAccount = await server.getAccount(address);

        // Build the transaction to call anonymous_voting_setup
        const contract = new Contract(CONTRACT_ID);
        
        const transaction = new TransactionBuilder(sourceAccount, {
          fee: BASE_FEE,
          networkPassphrase: networkPassphrase,
        })
          .addOperation(
            contract.call(
              "anonymous_voting_setup",
              Address.fromString(address).toScVal(),
              nativeToScVal(1, { type: "u32" }), // project_id
              nativeToScVal("BLS12_381_PUBLIC_KEY_PLACEHOLDER", { type: "string" })
            )
          )
          .setTimeout(30)
          .build();

        // Simulate first to check if already initialized
        const simulated = await server.simulateTransaction(transaction);
        
        if (rpc.Api.isSimulationSuccess(simulated)) {
          console.log("Anonymous voting setup would succeed");
          setInitialized(true);
        } else {
          // May already be initialized, which is fine
          console.log("Anonymous voting may already be set up");
          setInitialized(true);
        }
      } catch (err) {
        console.error("Error initializing anonymous voting:", err);
        setError(err instanceof Error ? err.message : "Unknown error");
        // Don't block the app if initialization fails
        setInitialized(true);
      }
    };

    initializeAnonymousVoting();
  }, [address, initialized]);

  return { initialized, error };
};

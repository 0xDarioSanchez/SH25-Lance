import { useState, useEffect } from "react";
import { Button, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import { callContract, LANCE_CONTRACT_ID, addressToScVal } from "../util/contractCall";
import { fundAccount } from "../util/friendbot";
import { Contract, nativeToScVal, scValToNative, TransactionBuilder, Account, BASE_FEE } from "@stellar/stellar-sdk";
import { rpc } from "@stellar/stellar-sdk";
import { network } from "../contracts/util";

export const RegisterJudgeButton = () => {
  const { address, signTransaction } = useWallet();
  const [loading, setLoading] = useState(false);
  const [checkingRegistration, setCheckingRegistration] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [registered, setRegistered] = useState(false);
  const [needsFunding, setNeedsFunding] = useState(false);
  const [fundingAccount, setFundingAccount] = useState(false);

  // Check if user is already registered on the blockchain (read-only, no signing)
  useEffect(() => {
    const checkRegistration = async () => {
      if (!address) {
        setRegistered(false);
        return;
      }

      setCheckingRegistration(true);
      try {
        // Use simulation for read-only check (no wallet signing required)
        const rpcServer = new rpc.Server(network.rpcUrl, {
          allowHttp: network.rpcUrl.includes("localhost"),
        });

        const contract = new Contract(LANCE_CONTRACT_ID);
        const operation = contract.call("get_user", addressToScVal(address));
        
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
          const result = scValToNative(simulation.result.retval);
          // If we get a result without error, user is registered
          if (result && result.address) {
            setRegistered(true);
          } else {
            setRegistered(false);
          }
        } else {
          setRegistered(false);
        }
      } catch (err) {
        // UserNotFound or any error means not registered
        setRegistered(false);
      } finally {
        setCheckingRegistration(false);
      }
    };

    checkRegistration();
  }, [address]);

  // Check if account needs funding when address changes
  useEffect(() => {
    setNeedsFunding(false);
    setError(null);
  }, [address]);

  const handleFundAccount = async () => {
    if (!address) return;

    setFundingAccount(true);
    setError(null);

    try {
      await fundAccount(address);
      setNeedsFunding(false);
      setRegistered(true);
    } catch (err) {
      console.error("Error funding account:", err);
      setError("Failed to fund account. Please try again or use Stellar Laboratory.");
    } finally {
      setFundingAccount(false);
    }
  };

  const handleRegister = async () => {
    if (!address || !signTransaction) return;

    setLoading(true);
    setError(null);

    try {
      await callContract({
        contractId: LANCE_CONTRACT_ID,
        method: "new_voter",
        args: [addressToScVal(address)],
        publicKey: address,
        signTransaction,
      });

      setRegistered(true);
    } catch (err) {
      console.error("Error registering as judge:", err);
      const errorMsg = err instanceof Error ? err.message : "Failed to register as judge";
      
      // Check if account needs to be funded
      if (errorMsg.includes("Account not found") || errorMsg.includes("not found")) {
        setNeedsFunding(true);
        setError("Your account needs to be funded on testnet first. Click 'Fund Account' below.");
      } else if (errorMsg.includes("MissingValue") || errorMsg.includes("Storage")) {
        setError("The contract is not properly initialized. Please ensure the contract has been deployed with --admin and --token parameters, or contact the contract administrator.");
      } else {
        setError(errorMsg);
      }
    } finally {
      setLoading(false);
    }
  };

  if (!address) {
    return null; // Don't show button if wallet not connected
  }

  // Show loading state while checking registration
  if (checkingRegistration) {
    return (
      <div style={{ marginTop: "2rem" }}>
        <Text as="p" size="sm">Checking registration status...</Text>
      </div>
    );
  }

  // If already registered, show success message
  if (registered) {
    return (
      <div style={{ marginTop: "2rem" }}>
        <Text as="h3" size="md" style={{ marginBottom: "0.5rem" }}>
          Judge Registration
        </Text>
        <Text as="p" size="sm" style={{ color: "green", marginBottom: "1rem" }}>
          ✓ You are registered as a judge! You can now vote on disputes.
        </Text>
      </div>
    );
  }

  return (
    <div style={{ marginTop: "2rem" }}>
      <Text as="h3" size="md" style={{ marginBottom: "0.5rem" }}>
        Register as Judge
      </Text>
      <Text as="p" size="sm" style={{ marginBottom: "1rem", color: "#666" }}>
        Before you can vote on disputes, you need to register as a judge in the system.
      </Text>

      {error && (
        <Text as="p" size="sm" style={{ color: "red", marginBottom: "1rem" }}>
          {error}
        </Text>
      )}

      {needsFunding ? (
        <div>
          <Button
            size="md"
            variant="primary"
            onClick={handleFundAccount}
            disabled={fundingAccount}
            style={{ marginRight: "1rem" }}
          >
            {fundingAccount ? "Funding Account..." : "Fund Account (Testnet)"}
          </Button>
          <a
            href={`https://laboratory.stellar.org/#account-creator?network=test`}
            target="_blank"
            rel="noopener noreferrer"
            style={{ fontSize: "14px", color: "#0066cc" }}
          >
            Or use Stellar Laboratory
          </a>
        </div>
      ) : (
        <Button
          size="md"
          variant="secondary"
          onClick={handleRegister}
          disabled={loading}
        >
          {loading ? "Registering..." : "Register as Judge"}
        </Button>
      )}
    </div>
  );
};

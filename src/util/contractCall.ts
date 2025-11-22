import {
  BASE_FEE,
  Contract,
  TransactionBuilder,
  xdr,
  scValToNative,
  nativeToScVal,
} from "@stellar/stellar-sdk";
import { rpc as StellarRpc } from "@stellar/stellar-sdk";
import { network } from "../contracts/util";

// Get contract ID from environment or use default
export const LANCE_CONTRACT_ID =
  import.meta.env.PUBLIC_LANCE_PROTOCOL_CONTRACT_ID ||
  "CCPMCRODNLFR24FFGYIDVCNKLKE7JZH6XVFIOJPCJZVFR474XS7VTMB5"; // Deployed & initialized testnet contract

console.log("[contractCall] LANCE_CONTRACT_ID loaded:", LANCE_CONTRACT_ID);
console.log("[contractCall] Raw env value:", import.meta.env.PUBLIC_LANCE_PROTOCOL_CONTRACT_ID);

interface ContractCallParams {
  contractId: string;
  method: string;
  args: xdr.ScVal[];
  publicKey: string;
  signTransaction: (xdr: string, opts: any) => Promise<{ signedTxXdr: string }>;
}

export async function callContract({
  contractId,
  method,
  args,
  publicKey,
  signTransaction,
}: ContractCallParams): Promise<any> {
  const rpcServer = new StellarRpc.Server(network.rpcUrl, {
    allowHttp: network.rpcUrl.includes("localhost"),
  });
  const contract = new Contract(contractId);

  // Get account
  const sourceAccount = await rpcServer.getAccount(publicKey);

  // Build operation
  const operation = contract.call(method, ...args);

  // Build transaction
  const transaction = new TransactionBuilder(sourceAccount, {
    fee: BASE_FEE,
    networkPassphrase: network.passphrase,
  })
    .addOperation(operation)
    .setTimeout(30)
    .build();

  // Prepare transaction (simulates and assembles)
  const preparedTransaction = await rpcServer.prepareTransaction(transaction);

  // Sign transaction with wallet
  const { signedTxXdr } = await signTransaction(preparedTransaction.toXDR(), {
    address: publicKey,
    networkPassphrase: network.passphrase,
  });

  const signedTransaction = TransactionBuilder.fromXDR(
    signedTxXdr,
    network.passphrase
  );

  // Submit transaction
  const sendResponse = await rpcServer.sendTransaction(signedTransaction);

  if (sendResponse.status !== "PENDING") {
    throw new Error(`Transaction error: ${sendResponse.status}`);
  }

  // Wait for transaction to be confirmed
  const MAX_ATTEMPTS = 10;
  let attempts = 0;
  let getResponse;

  while (attempts++ < MAX_ATTEMPTS) {
    getResponse = await rpcServer.getTransaction(sendResponse.hash);

    if (getResponse.status === "SUCCESS") {
      // Parse and return result if available
      if (getResponse.returnValue) {
        return scValToNative(getResponse.returnValue);
      }
      return null;
    }

    if (getResponse.status === "FAILED") {
      throw new Error(`Transaction failed`);
    }

    // NOT_FOUND, wait and retry
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }

  throw new Error("Transaction timeout");
}

// Helper to convert JavaScript values to ScVal
export function toScVal(value: any): xdr.ScVal {
  return nativeToScVal(value);
}

// Helper to create Address ScVal
export function addressToScVal(address: string): xdr.ScVal {
  return nativeToScVal(address, { type: "address" });
}

// Helper to create u32 ScVal
export function u32ToScVal(value: number): xdr.ScVal {
  return nativeToScVal(value, { type: "u32" });
}

// Helper to create u128 ScVal (as string)
export function u128ToScVal(value: string): xdr.ScVal {
  return nativeToScVal(value, { type: "u128" });
}

// Helper to create Vec<u128> ScVal
export function vecU128ToScVal(values: string[]): xdr.ScVal {
  const u128Values = values.map(v => nativeToScVal(v, { type: "u128" }));
  return nativeToScVal(u128Values, { type: "vec" });
}

// Helper to create Bytes ScVal
export function bytesToScVal(hexString: string): xdr.ScVal {
  return nativeToScVal(Buffer.from(hexString, "hex"), { type: "bytes" });
}

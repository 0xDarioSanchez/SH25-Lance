import { stellarNetwork } from "../contracts/util";

// Utility to get the correct Friendbot URL based on environment
export function getFriendbotUrl(address: string) {
  switch (stellarNetwork) {
    case "LOCAL":
      // Use proxy in development for local
      return `/friendbot?addr=${address}`;
    case "FUTURENET":
      return `https://friendbot-futurenet.stellar.org/?addr=${address}`;
    case "TESTNET":
      return `https://friendbot.stellar.org/?addr=${address}`;
    default:
      throw new Error(
        `Unknown or unsupported PUBLIC_STELLAR_NETWORK for friendbot: ${stellarNetwork}`,
      );
  }
}

// Fund an account using Friendbot (testnet/futurenet only)
export async function fundAccount(address: string): Promise<void> {
  if (stellarNetwork === "PUBLIC") {
    throw new Error("Friendbot is not available on mainnet");
  }

  const friendbotUrl = getFriendbotUrl(address);
  
  const response = await fetch(friendbotUrl);
  
  if (!response.ok) {
    const text = await response.text();
    throw new Error(`Friendbot failed: ${text}`);
  }
  
  // Wait a bit for the account to be created
  await new Promise(resolve => setTimeout(resolve, 2000));
}

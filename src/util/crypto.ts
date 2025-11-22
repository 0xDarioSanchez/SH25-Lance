// Crypto utility for anonymous voting with RSA-OAEP encryption
// Provides RSA key-pair generation and encryption/decryption for votes and seeds

// Generate a 2048-bit RSA-OAEP key pair and return base-64 strings
export async function generateRSAKeyPair(): Promise<{
  publicKey: string;
  privateKey: string;
}> {
  const keyPair = await crypto.subtle.generateKey(
    {
      name: "RSA-OAEP",
      modulusLength: 2048,
      publicExponent: new Uint8Array([0x01, 0x00, 0x01]),
      hash: "SHA-256",
    },
    true,
    ["encrypt", "decrypt"],
  );

  const [pubBuf, privBuf] = await Promise.all([
    crypto.subtle.exportKey("spki", keyPair.publicKey),
    crypto.subtle.exportKey("pkcs8", keyPair.privateKey),
  ]);

  return { publicKey: bufToB64(pubBuf), privateKey: bufToB64(privBuf) };
}

// Encrypt plaintext with RSA public key
export async function encryptWithPublicKey(
  plaintext: string,
  publicKeyB64: string,
): Promise<string> {
  const key = await importPublicKey(publicKeyB64);
  const enc = new TextEncoder().encode(plaintext);
  const cipher = await crypto.subtle.encrypt(
    { name: "RSA-OAEP" },
    key,
    enc as BufferSource,
  );
  return bufToB64(cipher);
}

// Decrypt ciphertext with RSA private key
export async function decryptWithPrivateKey(
  cipherB64: string,
  privateKeyB64: string,
): Promise<string> {
  const key = await importPrivateKey(privateKeyB64);
  const cipherBuf = b64ToBuf(cipherB64);
  const plainBuf = await crypto.subtle.decrypt(
    { name: "RSA-OAEP" },
    key,
    cipherBuf,
  );
  return new TextDecoder().decode(plainBuf);
}

// Helper: ArrayBuffer to Base64
function bufToB64(buf: ArrayBuffer): string {
  return btoa(String.fromCharCode(...new Uint8Array(buf)));
}

// Helper: Base64 to ArrayBuffer
function b64ToBuf(b64: string): ArrayBuffer {
  // Remove spaces, tabs, newlines, and non-base64-safe characters
  b64 = b64.replace(/[^A-Za-z0-9+/=]/g, "");
  const bin = atob(b64);
  return Uint8Array.from(bin, (c) => c.charCodeAt(0)).buffer;
}

// Helper: Import public key from base64
async function importPublicKey(b64: string): Promise<CryptoKey> {
  const clean = b64.replace(/[^A-Za-z0-9+/=]/g, "");
  return crypto.subtle.importKey(
    "spki",
    b64ToBuf(clean),
    { name: "RSA-OAEP", hash: "SHA-256" },
    true,
    ["encrypt"],
  );
}

// Helper: Import private key from base64
async function importPrivateKey(b64: string): Promise<CryptoKey> {
  const clean = b64.replace(/[^A-Za-z0-9+/=]/g, "");
  return crypto.subtle.importKey(
    "pkcs8",
    b64ToBuf(clean),
    { name: "RSA-OAEP", hash: "SHA-256" },
    true,
    ["decrypt"],
  );
}

// Download keypair as JSON file for user to save
export function downloadKeypairFile(
  publicKey: string,
  privateKey: string,
  projectId: number,
  disputeId?: number
): void {
  const keypairData = {
    publicKey,
    privateKey,
    projectId,
    disputeId,
    createdAt: new Date().toISOString(),
    description: disputeId 
      ? `Lance Protocol - Dispute #${disputeId} Anonymous Voting Keys`
      : `Lance Protocol - Project #${projectId} Anonymous Voting Keys`,
  };

  const blob = new Blob([JSON.stringify(keypairData, null, 2)], {
    type: "application/json",
  });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = disputeId 
    ? `lance-dispute-${disputeId}-keys.json`
    : `lance-project-${projectId}-keys.json`;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

// Parse and validate uploaded keypair file
export async function parseKeypairFile(file: File): Promise<{
  publicKey: string;
  privateKey: string;
  projectId: number;
  disputeId?: number;
}> {
  const text = await file.text();
  const data = JSON.parse(text);
  
  if (!data.privateKey || !data.publicKey) {
    throw new Error("Invalid key file - missing privateKey or publicKey");
  }
  
  return {
    publicKey: data.publicKey,
    privateKey: data.privateKey,
    projectId: data.projectId,
    disputeId: data.disputeId,
  };
}

import { Dispute } from "./types";

const BASE_URL = "https://6920566231e684d7bfcca593.mockapi.io/disputes";

export async function getDisputes(): Promise<Dispute[]> {
  const res = await fetch(BASE_URL);
  if (!res.ok) throw new Error("Error fetching disputes");
  return res.json();
}

export async function getDispute(id: string): Promise<Dispute> {
  const res = await fetch(`${BASE_URL}/${id}`);
  if (!res.ok) throw new Error("Error fetching dispute");
  return res.json();
}

export async function createDispute(payload: Partial<Dispute>): Promise<Dispute> {
  const res = await fetch(BASE_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!res.ok) throw new Error("Error creating dispute");
  return res.json();
}

export async function deleteDispute(id: string): Promise<Dispute> {
  const res = await fetch(`${BASE_URL}/${id}`, {
    method: "DELETE",
  });
  if (!res.ok) throw new Error("Error deleting dispute");
  return res.json();
}

export async function updateDispute(id: string, payload: Partial<Dispute>): Promise<Dispute> {
  const res = await fetch(`${BASE_URL}/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!res.ok) throw new Error("Error updating dispute");
  return res.json();
}

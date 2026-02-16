export async function fetchMessages() {
  const res = await fetch('/api/messages');
  if (!res.ok) throw new Error(`Failed to fetch messages: ${res.status}`);
  return res.json();
}

export async function fetchConfig() {
  const res = await fetch('/api/config');
  if (!res.ok) throw new Error(`Failed to fetch config: ${res.status}`);
  return res.json();
}

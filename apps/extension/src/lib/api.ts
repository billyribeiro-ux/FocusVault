/**
 * Lightweight API client for the FocusVault extension.
 * Talks to the local FocusVault server.
 */

import { browser } from '#imports';

const DEFAULT_API_URL = 'http://localhost:3000/api/v1';

export async function getApiUrl(): Promise<string> {
  const result = await browser.storage.local.get('apiUrl');
  return (result.apiUrl as string) || DEFAULT_API_URL;
}

export async function setApiUrl(url: string): Promise<void> {
  await browser.storage.local.set({ apiUrl: url });
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const base = await getApiUrl();
  const url = `${base}${path}`;

  const response = await fetch(url, {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    ...options,
  });

  if (!response.ok) {
    const err = await response.json().catch(() => ({ error: `HTTP ${response.status}` }));
    throw new Error(err.error || `HTTP ${response.status}`);
  }

  if (response.status === 204) return undefined as T;
  return response.json();
}

export interface VaultItem {
  id: string;
  item_type: string;
  status: string;
  title: string;
  url: string | null;
  tags: string[];
  capture_source: string;
  created_at: string;
}

export interface CreateVaultItem {
  item_type: string;
  title: string;
  url?: string;
  body?: string;
  tags?: string[];
  capture_source?: string;
}

export const api = {
  async captureToVault(input: CreateVaultItem): Promise<VaultItem> {
    return request<VaultItem>('/vault', {
      method: 'POST',
      body: JSON.stringify({
        ...input,
        capture_source: input.capture_source || 'extension',
      }),
    });
  },

  async getTabLimit(): Promise<number> {
    const today = await request<{ tab_limit: number }>('/logs/today');
    return today.tab_limit;
  },

  async healthCheck(): Promise<boolean> {
    try {
      await request<unknown>('/health');
      return true;
    } catch {
      return false;
    }
  },
};

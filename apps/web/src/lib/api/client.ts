/**
 * API client abstraction.
 * - On web: uses fetch() to the Rust backend (proxied via Vite in dev)
 * - On Tauri desktop: will use invoke() (Phase 4)
 */

const API_BASE = '/api/v1';

interface ApiError {
	error: string;
	code: string;
}

class FocusVaultClient {
	private async request<T>(
		path: string,
		options: RequestInit = {}
	): Promise<T> {
		const url = `${API_BASE}${path}`;
		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			},
			...options
		});

		if (!response.ok) {
			const error: ApiError = await response.json().catch(() => ({
				error: `HTTP ${response.status}`,
				code: 'UNKNOWN'
			}));
			throw new Error(error.error);
		}

		if (response.status === 204) {
			return undefined as T;
		}

		return response.json();
	}

	// ── Vault ──

	async listVaultItems(params?: Record<string, string>) {
		const query = params ? `?${new URLSearchParams(params)}` : '';
		return this.request<unknown[]>(`/vault${query}`);
	}

	async createVaultItem(input: unknown) {
		return this.request<unknown>('/vault', {
			method: 'POST',
			body: JSON.stringify(input)
		});
	}

	async updateVaultItem(id: string, update: unknown) {
		return this.request<unknown>(`/vault/${id}`, {
			method: 'PATCH',
			body: JSON.stringify(update)
		});
	}

	async deleteVaultItem(id: string) {
		return this.request<void>(`/vault/${id}`, { method: 'DELETE' });
	}

	// ── Missions ──

	async listMissions() {
		return this.request<unknown[]>('/missions');
	}

	async createMission(input: unknown) {
		return this.request<unknown>('/missions', {
			method: 'POST',
			body: JSON.stringify(input)
		});
	}

	async activateMission(id: string) {
		return this.request<unknown>(`/missions/${id}/activate`, { method: 'POST' });
	}

	// ── Daily Logs ──

	async listDailyLogs(params?: Record<string, string>) {
		const query = params ? `?${new URLSearchParams(params)}` : '';
		return this.request<unknown[]>(`/logs${query}`);
	}

	async upsertDailyLog(input: unknown) {
		return this.request<unknown>('/logs', {
			method: 'POST',
			body: JSON.stringify(input)
		});
	}

	// ── Health ──

	async healthCheck() {
		return this.request<{ status: string; version: string }>('/health');
	}
}

export const api = new FocusVaultClient();

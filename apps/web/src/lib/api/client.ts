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

	async updateMission(id: string, update: unknown) {
		return this.request<unknown>(`/missions/${id}`, {
			method: 'PATCH',
			body: JSON.stringify(update)
		});
	}

	async activateMission(id: string) {
		return this.request<unknown>(`/missions/${id}/activate`, { method: 'POST' });
	}

	// ── Daily Logs ──

	async getToday() {
		return this.request<unknown>('/logs/today');
	}

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

	async updateDailyLog(date: string, update: unknown) {
		return this.request<unknown>(`/logs/${date}`, {
			method: 'PATCH',
			body: JSON.stringify(update)
		});
	}

	// ── Projects ──

	async listProjects() {
		return this.request<unknown[]>('/projects');
	}

	async createProject(input: unknown) {
		return this.request<unknown>('/projects', {
			method: 'POST',
			body: JSON.stringify(input)
		});
	}

	async updateProject(id: string, update: unknown) {
		return this.request<unknown>(`/projects/${id}`, {
			method: 'PATCH',
			body: JSON.stringify(update)
		});
	}

	// ── Courses ──

	async listCourses() {
		return this.request<unknown[]>('/courses');
	}

	async createCourse(input: unknown) {
		return this.request<unknown>('/courses', {
			method: 'POST',
			body: JSON.stringify(input)
		});
	}

	async activateCourse(id: string) {
		return this.request<unknown>(`/courses/${id}/activate`, { method: 'POST' });
	}

	async completeCourse(id: string) {
		return this.request<unknown>(`/courses/${id}/complete`, { method: 'POST' });
	}

	// ── Languages ──

	async listLanguageTracks() {
		return this.request<unknown[]>('/languages');
	}

	async createLanguageTrack(input: unknown) {
		return this.request<unknown>('/languages', {
			method: 'POST',
			body: JSON.stringify(input)
		});
	}

	async activateLanguageTrack(id: string) {
		return this.request<unknown>(`/languages/${id}/activate`, { method: 'POST' });
	}

	async completeLanguageTrack(id: string) {
		return this.request<unknown>(`/languages/${id}/complete`, { method: 'POST' });
	}

	// ── Health ──

	async healthCheck() {
		return this.request<{ status: string; version: string }>('/health');
	}
}

export const api = new FocusVaultClient();

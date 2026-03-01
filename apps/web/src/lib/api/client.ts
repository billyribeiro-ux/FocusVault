/**
 * API client abstraction.
 * - On web: uses fetch() to the Rust backend (proxied via Vite in dev)
 * - On Tauri desktop: uses fetch() to the embedded server on a dynamic port
 *
 * In Tauri mode the embedded axum server emits an "api-ready" event with the
 * base URL. The frontend can also call the `get_api_url` Tauri command.
 */

let apiBase = '/api/v1';

/**
 * Detect Tauri environment and resolve the embedded API server URL.
 * Called once at startup from the root layout.
 *
 * In the Tauri desktop app, __TAURI_INTERNALS__ is injected by the runtime.
 * We use it directly to avoid a build-time dependency on @tauri-apps/api.
 */
export async function initApiBase(): Promise<void> {
	if (typeof window === 'undefined') return;

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const tauri = (window as any).__TAURI_INTERNALS__;
	if (!tauri?.invoke) return;

	// The embedded server starts asynchronously — retry briefly if not ready
	for (let attempt = 0; attempt < 15; attempt++) {
		try {
			const url: string = await tauri.invoke('get_api_url');
			if (url) {
				apiBase = `${url}/api/v1`;
				return;
			}
		} catch {
			// Server not ready yet
		}
		await new Promise((r) => setTimeout(r, 200));
	}
}

interface ApiError {
	error: string;
	code: string;
}

class FocusVaultClient {
	private async request<T>(
		path: string,
		options: RequestInit = {}
	): Promise<T> {
		const url = `${apiBase}${path}`;
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

import { api } from '$lib/api/client';

// Types matching the Rust domain model
export interface LanguageTrack {
	id: string;
	name: string;
	status: 'active' | 'paused' | 'completed';
	started_at: string | null;
	completed_at: string | null;
	weekly_goal_minutes: number;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateLanguageTrackInput {
	name: string;
	weekly_goal_minutes?: number;
}

// Class-based store using $state rune
class LanguageStore {
	items = $state<LanguageTrack[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	// Derived: the currently active language track
	get activeTrack(): LanguageTrack | undefined {
		return this.items.find((t) => t.status === 'active');
	}

	async load() {
		this.loading = true;
		this.error = null;
		try {
			this.items = (await api.listLanguageTracks()) as LanguageTrack[];
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load language tracks';
		} finally {
			this.loading = false;
		}
	}

	async create(input: CreateLanguageTrackInput) {
		this.error = null;
		try {
			const track = (await api.createLanguageTrack(input)) as LanguageTrack;
			this.items = [track, ...this.items];
			return track;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to create language track';
			throw e;
		}
	}

	async activate(id: string) {
		this.error = null;
		try {
			await api.activateLanguageTrack(id);
			// Reload all tracks to reflect status changes
			// (activating one track may deactivate others)
			await this.load();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to activate language track';
			throw e;
		}
	}

	async complete(id: string) {
		this.error = null;
		try {
			await api.completeLanguageTrack(id);
			// Reload all tracks to reflect status changes
			await this.load();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to complete language track';
			throw e;
		}
	}
}

export const languageStore = new LanguageStore();

import { api } from '$lib/api/client';

// Types matching the Rust domain model
export interface Mission {
	id: string;
	name: string;
	description: string;
	status: 'active' | 'paused' | 'done';
	started_at: string | null;
	ended_at: string | null;
	tab_limit: number;
	weekly_targets: {
		deploy: number;
		outreach: number;
		iterate: number;
	};
	kpis: {
		calls_booked: number;
		revenue: number;
		leads: number;
	};
	created_at: string;
	updated_at: string;
}

export interface CreateMissionInput {
	name: string;
	description?: string;
	tab_limit?: number;
	weekly_targets?: Mission['weekly_targets'];
	kpis?: Mission['kpis'];
}

// Class-based store using $state rune
class MissionStore {
	items = $state<Mission[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	// Derived: the currently active mission
	get activeMission(): Mission | undefined {
		return this.items.find((m) => m.status === 'active');
	}

	async load() {
		this.loading = true;
		this.error = null;
		try {
			this.items = (await api.listMissions()) as Mission[];
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load missions';
		} finally {
			this.loading = false;
		}
	}

	async create(input: CreateMissionInput) {
		this.error = null;
		try {
			const mission = (await api.createMission(input)) as Mission;
			this.items = [mission, ...this.items];
			return mission;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to create mission';
			throw e;
		}
	}

	async update(id: string, update: Partial<Pick<Mission, 'name' | 'description' | 'status' | 'tab_limit' | 'weekly_targets' | 'kpis'>>) {
		this.error = null;
		try {
			const updated = (await api.updateMission(id, update)) as Mission;
			this.items = this.items.map((m) => (m.id === id ? updated : m));
			return updated;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to update mission';
			throw e;
		}
	}

	async activate(id: string) {
		this.error = null;
		try {
			await api.activateMission(id);
			// Reload all missions to reflect status changes
			// (activating one mission may deactivate others)
			await this.load();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to activate mission';
			throw e;
		}
	}
}

export const missionStore = new MissionStore();

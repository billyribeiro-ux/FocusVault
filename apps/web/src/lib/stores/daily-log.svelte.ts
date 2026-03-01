import { api } from '$lib/api/client';

// Types matching the Rust domain model
export interface DailyLog {
	id: string;
	date: string;
	plan_day: string | null;
	cycles_completed: number;
	watch_done: boolean;
	build_done: boolean;
	prove_done: boolean;
	tab_limit: number;
	tab_limit_met: boolean;
	active_mission_id: string | null;
	focus_notes: string | null;
	sleep_hours_est: number | null;
	mood: 'low' | 'ok' | 'high' | null;
	created_at: string;
	updated_at: string;
}

// Helper: get today's date as YYYY-MM-DD in local timezone
function todayDateString(): string {
	const now = new Date();
	const year = now.getFullYear();
	const month = String(now.getMonth() + 1).padStart(2, '0');
	const day = String(now.getDate()).padStart(2, '0');
	return `${year}-${month}-${day}`;
}

// Class-based store using $state rune
class DailyLogStore {
	today = $state<DailyLog | null>(null);
	logs = $state<DailyLog[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	async loadToday() {
		this.loading = true;
		this.error = null;
		try {
			// GET /logs/today auto-creates if missing
			const log = (await api.getToday()) as DailyLog;
			this.today = log;
			return log;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load today\'s log';
		} finally {
			this.loading = false;
		}
	}

	async loadRange(params?: Record<string, string>) {
		this.loading = true;
		this.error = null;
		try {
			this.logs = (await api.listDailyLogs(params)) as DailyLog[];
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load daily logs';
		} finally {
			this.loading = false;
		}
	}

	async updateToday(fields: Partial<DailyLog>) {
		this.error = null;
		try {
			const log = (await api.updateDailyLog(todayDateString(), fields)) as DailyLog;
			this.today = log;
			return log;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to update today\'s log';
			throw e;
		}
	}

	async incrementCycle() {
		const current = this.today?.cycles_completed ?? 0;
		return this.updateToday({ cycles_completed: current + 1 } as Partial<DailyLog>);
	}

	async toggleWatch() {
		const current = this.today?.watch_done ?? false;
		return this.updateToday({ watch_done: !current } as Partial<DailyLog>);
	}

	async toggleBuild() {
		const current = this.today?.build_done ?? false;
		return this.updateToday({ build_done: !current } as Partial<DailyLog>);
	}

	async toggleProve() {
		const current = this.today?.prove_done ?? false;
		return this.updateToday({ prove_done: !current } as Partial<DailyLog>);
	}
}

export const dailyLogStore = new DailyLogStore();

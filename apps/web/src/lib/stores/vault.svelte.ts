import { api } from '$lib/api/client';

// Types matching the Rust domain model
export interface VaultItem {
	id: string;
	type: 'link' | 'note' | 'snippet' | 'file_ref';
	url: string | null;
	title: string | null;
	favicon_url: string | null;
	hostname: string | null;
	why: string;
	notes: string | null;
	status: 'inbox' | 'saved' | 'actioned' | 'archived';
	priority: 'low' | 'med' | 'high';
	pinned: boolean;
	tags: string[];
	project_id: string | null;
	source: 'manual' | 'extension' | 'import' | 'api';
	due_at: string | null;
	last_opened_at: string | null;
	open_count: number;
	metadata: unknown;
	created_at: string;
	updated_at: string;
}

export interface CreateVaultItemInput {
	type: VaultItem['type'];
	url?: string;
	title?: string;
	favicon_url?: string;
	why: string;
	notes?: string;
	status?: VaultItem['status'];
	priority?: VaultItem['priority'];
	tags?: string[];
	project_id?: string;
	source?: VaultItem['source'];
	due_at?: string;
	metadata?: unknown;
}

// Class-based store using $state rune
class VaultStore {
	items = $state<VaultItem[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);
	activeStatus = $state<VaultItem['status']>('inbox');

	// Derived: filtered items by active status
	get filtered(): VaultItem[] {
		return this.items.filter((item) => item.status === this.activeStatus);
	}

	get inboxCount(): number {
		return this.items.filter((i) => i.status === 'inbox').length;
	}

	async load(params?: Record<string, string>) {
		this.loading = true;
		this.error = null;
		try {
			this.items = (await api.listVaultItems(params)) as VaultItem[];
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load vault items';
		} finally {
			this.loading = false;
		}
	}

	async create(input: CreateVaultItemInput) {
		this.error = null;
		try {
			const item = (await api.createVaultItem(input)) as VaultItem;
			this.items = [item, ...this.items];
			return item;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to create item';
			throw e;
		}
	}

	async update(id: string, update: Partial<VaultItem>) {
		this.error = null;
		try {
			const updated = (await api.updateVaultItem(id, update)) as VaultItem;
			this.items = this.items.map((i) => (i.id === id ? updated : i));
			return updated;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to update item';
			throw e;
		}
	}

	async remove(id: string) {
		this.error = null;
		try {
			await api.deleteVaultItem(id);
			this.items = this.items.filter((i) => i.id !== id);
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to delete item';
			throw e;
		}
	}

	setActiveStatus(status: VaultItem['status']) {
		this.activeStatus = status;
	}
}

export const vaultStore = new VaultStore();

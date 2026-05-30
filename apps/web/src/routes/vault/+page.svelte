<script lang="ts">
	import { onMount } from 'svelte';
	import { vaultStore } from '$lib/stores/vault.svelte';
	import type { VaultItem, CreateVaultItemInput } from '$lib/stores/vault.svelte';
	import VaultCreateModal from '$lib/components/VaultCreateModal.svelte';

	let showCreateModal = $state(false);
	let searchQuery = $state('');
	let searchTimeout = $state<ReturnType<typeof setTimeout> | null>(null);

	const tabs: { id: VaultItem['status']; label: string }[] = [
		{ id: 'inbox', label: 'Inbox' },
		{ id: 'saved', label: 'Saved' },
		{ id: 'actioned', label: 'Actioned' },
		{ id: 'archived', label: 'Archived' }
	];

	const statusFlow: Record<string, VaultItem['status']> = {
		inbox: 'saved',
		saved: 'actioned',
		actioned: 'archived'
	};

	const typeIcons: Record<VaultItem['type'], string> = {
		link: '🔗',
		note: '📝',
		snippet: '< >',
		file_ref: '📁'
	};

	const priorityColors: Record<VaultItem['priority'], string> = {
		low: 'var(--color-success)',
		med: 'var(--color-warning)',
		high: 'var(--color-danger)'
	};

	const priorityLabels: Record<VaultItem['priority'], string> = {
		low: 'Low',
		med: 'Med',
		high: 'High'
	};

	onMount(() => {
		vaultStore.load();
	});

	function handleSearch(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		searchQuery = value;

		if (searchTimeout) clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			const params: Record<string, string> = {};
			if (value.trim()) params.query = value.trim();
			vaultStore.load(params);
		}, 300);
	}

	function switchTab(status: VaultItem['status']) {
		vaultStore.setActiveStatus(status);
	}

	async function handleCreate(input: CreateVaultItemInput) {
		try {
			await vaultStore.create(input);
			showCreateModal = false;
		} catch {
			// error displayed via vaultStore.error
		}
	}

	function getDisplayTitle(item: VaultItem): string {
		if (item.title) return item.title;
		if (item.hostname) return item.hostname;
		return 'Untitled';
	}

	function getItemIcon(item: VaultItem): string {
		return typeIcons[item.type] || '?';
	}

	async function moveToNextStatus(item: VaultItem) {
		const next = statusFlow[item.status];
		if (next) {
			await vaultStore.update(item.id, { status: next });
		}
	}

	async function togglePin(item: VaultItem) {
		await vaultStore.update(item.id, { pinned: !item.pinned });
	}

	async function archiveItem(item: VaultItem) {
		await vaultStore.update(item.id, { status: 'archived' });
	}

	async function deleteItem(item: VaultItem) {
		await vaultStore.remove(item.id);
	}

	function getNextStatusLabel(status: VaultItem['status']): string | null {
		const map: Record<string, string> = {
			inbox: 'Save',
			saved: 'Action',
			actioned: 'Archive'
		};
		return map[status] || null;
	}

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		const now = new Date();
		const diff = now.getTime() - d.getTime();
		const days = Math.floor(diff / (1000 * 60 * 60 * 24));

		if (days === 0) return 'Today';
		if (days === 1) return 'Yesterday';
		if (days < 7) return `${days}d ago`;
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	function tabCount(status: VaultItem['status']): number {
		return vaultStore.items.filter((i) => i.status === status).length;
	}
</script>

<div class="vault-page">
	<!-- Header -->
	<div class="vault-header">
		<div>
			<h2 class="vault-title">Vault</h2>
			<p class="vault-subtitle">Captured items and references</p>
		</div>
		<button class="add-btn" onclick={() => (showCreateModal = true)}>
			+ Add Item
		</button>
	</div>

	<!-- Error Banner -->
	{#if vaultStore.error}
		<div class="error-banner">
			<span class="error-icon">!</span>
			<span class="error-text">{vaultStore.error}</span>
			<button class="error-dismiss" onclick={() => (vaultStore.error = null)}>&times;</button>
		</div>
	{/if}

	<!-- Search Bar -->
	<div class="search-container">
		<span class="search-icon">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<circle cx="11" cy="11" r="8"></circle>
				<line x1="21" y1="21" x2="16.65" y2="16.65"></line>
			</svg>
		</span>
		<input
			type="text"
			class="search-input"
			placeholder="Search vault items..."
			value={searchQuery}
			oninput={handleSearch}
		/>
		{#if searchQuery}
			<button class="search-clear" onclick={() => { searchQuery = ''; vaultStore.load(); }}>
				&times;
			</button>
		{/if}
	</div>

	<!-- Pipeline Tabs -->
	<div class="tabs-container">
		{#each tabs as tab}
			<button
				class="tab-btn"
				class:tab-active={vaultStore.activeStatus === tab.id}
				onclick={() => switchTab(tab.id)}
			>
				{tab.label}
				{#if tabCount(tab.id) > 0}
					<span class="tab-count">{tabCount(tab.id)}</span>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Loading State -->
	{#if vaultStore.loading}
		<div class="loading-container">
			<div class="spinner"></div>
			<p class="loading-text">Loading vault items...</p>
		</div>
	{:else if vaultStore.filtered.length === 0}
		<!-- Empty State -->
		<div class="empty-state">
			<div class="empty-icon">
				{#if vaultStore.activeStatus === 'inbox'}
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="color: var(--color-text-muted)">
						<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
						<path d="M5.45 5.11L2 12v6a2 2 0 002 2h16a2 2 0 002-2v-6l-3.45-6.89A2 2 0 0016.76 4H7.24a2 2 0 00-1.79 1.11z"></path>
					</svg>
				{:else}
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="color: var(--color-text-muted)">
						<rect x="2" y="6" width="20" height="14" rx="2"></rect>
						<path d="M2 10h20"></path>
					</svg>
				{/if}
			</div>
			<p class="empty-title">
				{#if searchQuery}
					No results for "{searchQuery}"
				{:else}
					No {vaultStore.activeStatus} items
				{/if}
			</p>
			<p class="empty-subtitle">
				{#if vaultStore.activeStatus === 'inbox' && !searchQuery}
					Capture your first item to get started
				{:else if searchQuery}
					Try a different search term
				{:else}
					Items will appear here as you move them through the pipeline
				{/if}
			</p>
			{#if vaultStore.activeStatus === 'inbox' && !searchQuery}
				<button class="empty-cta" onclick={() => (showCreateModal = true)}>
					+ Add your first item
				</button>
			{/if}
		</div>
	{:else}
		<!-- Item List -->
		<div class="item-list">
			{#each vaultStore.filtered as item (item.id)}
				<div class="item-row" class:item-pinned={item.pinned}>
					<!-- Left: Icon + Content -->
					<div class="item-main">
						<!-- Type icon / Favicon -->
						<div class="item-icon-container">
							{#if item.type === 'link' && item.favicon_url}
								<img
									class="item-favicon"
									src={item.favicon_url}
									alt=""
									onerror={(e) => { (e.target as HTMLImageElement).style.display = 'none'; (e.target as HTMLImageElement).nextElementSibling?.classList.remove('hidden'); }}
								/>
								<span class="item-type-icon hidden">{getItemIcon(item)}</span>
							{:else}
								<span class="item-type-icon">{getItemIcon(item)}</span>
							{/if}
						</div>

						<!-- Content -->
						<div class="item-content">
							<div class="item-title-row">
								{#if item.pinned}
									<span class="pin-indicator" title="Pinned">&#x1F4CC;</span>
								{/if}
								<span class="item-title">{getDisplayTitle(item)}</span>
								<span class="priority-badge" style="background-color: {priorityColors[item.priority]}20; color: {priorityColors[item.priority]}">
									{priorityLabels[item.priority]}
								</span>
							</div>

							{#if item.why}
								<p class="item-why">{item.why}</p>
							{/if}

							<div class="item-meta">
								{#if item.tags.length > 0}
									<div class="item-tags">
										{#each item.tags.slice(0, 3) as tag}
											<span class="item-tag">{tag}</span>
										{/each}
										{#if item.tags.length > 3}
											<span class="item-tag item-tag-more">+{item.tags.length - 3}</span>
										{/if}
									</div>
								{/if}
								<span class="item-date">{formatDate(item.created_at)}</span>
							</div>
						</div>
					</div>

					<!-- Right: Actions -->
					<div class="item-actions">
						{#if getNextStatusLabel(item.status)}
							<button
								class="action-btn action-move"
								title="Move to {getNextStatusLabel(item.status)}"
								onclick={() => moveToNextStatus(item)}
							>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<line x1="5" y1="12" x2="19" y2="12"></line>
									<polyline points="12 5 19 12 12 19"></polyline>
								</svg>
							</button>
						{/if}
						<button
							class="action-btn"
							class:action-pinned={item.pinned}
							title={item.pinned ? 'Unpin' : 'Pin'}
							onclick={() => togglePin(item)}
						>
							<svg width="14" height="14" viewBox="0 0 24 24" fill={item.pinned ? 'currentColor' : 'none'} stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"></path>
							</svg>
						</button>
						{#if item.status !== 'archived'}
							<button
								class="action-btn"
								title="Archive"
								onclick={() => archiveItem(item)}
							>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
									<polyline points="21 8 21 21 3 21 3 8"></polyline>
									<rect x="1" y="3" width="22" height="5"></rect>
									<line x1="10" y1="12" x2="14" y2="12"></line>
								</svg>
							</button>
						{/if}
						<button
							class="action-btn action-delete"
							title="Delete"
							onclick={() => deleteItem(item)}
						>
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="3 6 5 6 21 6"></polyline>
								<path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"></path>
							</svg>
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Create Modal -->
<VaultCreateModal
	open={showCreateModal}
	onclose={() => (showCreateModal = false)}
	oncreate={handleCreate}
/>

<style>
	.vault-page {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	/* Header */
	.vault-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.vault-title {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: -0.025em;
		color: var(--color-text);
		margin: 0;
	}

	.vault-subtitle {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.125rem 0 0 0;
	}

	.add-btn {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: white;
		background-color: var(--color-accent);
		border: none;
		cursor: pointer;
		transition: background-color 0.15s;
		white-space: nowrap;
		font-family: inherit;
	}

	.add-btn:hover {
		background-color: var(--color-accent-hover);
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.75rem 1rem;
		background-color: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.5rem;
		color: var(--color-danger);
		font-size: 0.875rem;
	}

	.error-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 1.25rem;
		height: 1.25rem;
		border-radius: 50%;
		background-color: var(--color-danger);
		color: white;
		font-size: 0.75rem;
		font-weight: 700;
		flex-shrink: 0;
	}

	.error-text {
		flex: 1;
	}

	.error-dismiss {
		background: none;
		border: none;
		color: var(--color-danger);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0;
		line-height: 1;
		opacity: 0.7;
	}

	.error-dismiss:hover {
		opacity: 1;
	}

	/* Search */
	.search-container {
		position: relative;
		display: flex;
		align-items: center;
	}

	.search-icon {
		position: absolute;
		left: 0.75rem;
		color: var(--color-text-muted);
		display: flex;
		align-items: center;
		pointer-events: none;
	}

	.search-input {
		width: 100%;
		padding: 0.5rem 2rem 0.5rem 2.25rem;
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		font-size: 0.875rem;
		color: var(--color-text);
		outline: none;
		transition: border-color 0.15s;
		font-family: inherit;
		box-sizing: border-box;
	}

	.search-input::placeholder {
		color: var(--color-text-muted);
		opacity: 0.6;
	}

	.search-input:focus {
		border-color: var(--color-accent);
	}

	.search-clear {
		position: absolute;
		right: 0.5rem;
		background: none;
		border: none;
		color: var(--color-text-muted);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0.25rem;
		line-height: 1;
	}

	.search-clear:hover {
		color: var(--color-text);
	}

	/* Tabs */
	.tabs-container {
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		border-radius: 0.5rem;
		background-color: var(--color-surface);
		overflow-x: auto;
	}

	.tab-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--color-text-muted);
		background: none;
		border: none;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
		font-family: inherit;
	}

	.tab-btn:hover {
		color: var(--color-text);
	}

	.tab-active {
		background-color: var(--color-accent);
		color: white;
	}

	.tab-active:hover {
		color: white;
	}

	.tab-count {
		font-size: 0.6875rem;
		background-color: rgba(255, 255, 255, 0.2);
		padding: 0.0625rem 0.375rem;
		border-radius: 9999px;
		min-width: 1.125rem;
		text-align: center;
	}

	.tab-btn:not(.tab-active) .tab-count {
		background-color: var(--color-border);
		color: var(--color-text-muted);
	}

	/* Loading */
	.loading-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		gap: 0.75rem;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid var(--color-border);
		border-top-color: var(--color-accent);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-text {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 3rem 1rem;
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 0.75rem;
		text-align: center;
	}

	.empty-icon {
		margin-bottom: 1rem;
		opacity: 0.5;
	}

	.empty-title {
		font-size: 1rem;
		font-weight: 500;
		color: var(--color-text-muted);
		margin: 0;
	}

	.empty-subtitle {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		margin: 0.375rem 0 0 0;
		opacity: 0.7;
	}

	.empty-cta {
		margin-top: 1rem;
		padding: 0.5rem 1.25rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: white;
		background-color: var(--color-accent);
		border: none;
		cursor: pointer;
		transition: background-color 0.15s;
		font-family: inherit;
	}

	.empty-cta:hover {
		background-color: var(--color-accent-hover);
	}

	/* Item List */
	.item-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.item-row {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 0.75rem;
		padding: 0.75rem;
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		transition: background-color 0.15s, border-color 0.15s;
	}

	.item-row:hover {
		background-color: var(--color-surface-hover);
		border-color: var(--color-text-muted);
	}

	.item-pinned {
		border-left: 2px solid var(--color-accent);
	}

	.item-main {
		display: flex;
		gap: 0.625rem;
		flex: 1;
		min-width: 0;
	}

	/* Icon */
	.item-icon-container {
		flex-shrink: 0;
		width: 1.75rem;
		height: 1.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.375rem;
		background-color: var(--color-bg);
		font-size: 0.875rem;
	}

	.item-favicon {
		width: 1rem;
		height: 1rem;
		border-radius: 0.125rem;
		object-fit: contain;
	}

	.item-type-icon {
		font-size: 0.8125rem;
		line-height: 1;
	}

	.hidden {
		display: none;
	}

	/* Content */
	.item-content {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.item-title-row {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		flex-wrap: wrap;
	}

	.pin-indicator {
		font-size: 0.75rem;
		flex-shrink: 0;
	}

	.item-title {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--color-text);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		max-width: 100%;
	}

	.priority-badge {
		font-size: 0.625rem;
		font-weight: 600;
		padding: 0.0625rem 0.375rem;
		border-radius: 9999px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		flex-shrink: 0;
		white-space: nowrap;
	}

	.item-why {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		line-height: 1.4;
	}

	.item-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.item-tags {
		display: flex;
		gap: 0.25rem;
		flex-wrap: wrap;
	}

	.item-tag {
		font-size: 0.6875rem;
		padding: 0.0625rem 0.375rem;
		border-radius: 9999px;
		background-color: rgba(59, 130, 246, 0.1);
		color: var(--color-accent);
		white-space: nowrap;
	}

	.item-tag-more {
		background-color: var(--color-border);
		color: var(--color-text-muted);
	}

	.item-date {
		font-size: 0.6875rem;
		color: var(--color-text-muted);
		opacity: 0.7;
		white-space: nowrap;
	}

	/* Actions */
	.item-actions {
		display: flex;
		gap: 0.25rem;
		flex-shrink: 0;
		align-items: flex-start;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.item-row:hover .item-actions {
		opacity: 1;
	}

	/* On mobile, always show actions */
	@media (max-width: 639px) {
		.item-actions {
			opacity: 1;
		}
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 1.75rem;
		height: 1.75rem;
		border-radius: 0.375rem;
		background: none;
		border: 1px solid transparent;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
		padding: 0;
	}

	.action-btn:hover {
		background-color: var(--color-bg);
		border-color: var(--color-border);
		color: var(--color-text);
	}

	.action-move:hover {
		color: var(--color-accent);
		border-color: var(--color-accent);
	}

	.action-pinned {
		color: var(--color-accent);
	}

	.action-delete:hover {
		color: var(--color-danger);
		border-color: var(--color-danger);
	}
</style>

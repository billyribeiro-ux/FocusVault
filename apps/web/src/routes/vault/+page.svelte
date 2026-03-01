<script lang="ts">
	// Vault page — Phase 2 implementation
	let activeTab = $state<'inbox' | 'saved' | 'actioned' | 'archived'>('inbox');

	const tabs = [
		{ id: 'inbox' as const, label: 'Inbox' },
		{ id: 'saved' as const, label: 'Saved' },
		{ id: 'actioned' as const, label: 'Actioned' },
		{ id: 'archived' as const, label: 'Archived' }
	];
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-2xl font-bold tracking-tight">Vault</h2>
			<p class="text-sm" style="color: var(--color-text-muted)">Captured items and references</p>
		</div>
		<button
			class="rounded-lg px-4 py-2 text-sm font-medium text-white"
			style="background-color: var(--color-accent)"
		>
			+ Add Item
		</button>
	</div>

	<!-- Pipeline Tabs -->
	<div class="flex gap-1 rounded-lg p-1" style="background-color: var(--color-surface)">
		{#each tabs as tab}
			<button
				class="rounded-md px-4 py-1.5 text-sm font-medium transition-colors"
				style={activeTab === tab.id
					? `background-color: var(--color-accent); color: white`
					: `color: var(--color-text-muted)`}
				onclick={() => (activeTab = tab.id)}
			>
				{tab.label}
			</button>
		{/each}
	</div>

	<!-- Empty State -->
	<div
		class="rounded-xl border p-12 text-center"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p class="text-lg font-medium" style="color: var(--color-text-muted)">No items yet</p>
		<p class="mt-1 text-sm" style="color: var(--color-text-muted)">
			Capture your first item to get started
		</p>
	</div>
</div>

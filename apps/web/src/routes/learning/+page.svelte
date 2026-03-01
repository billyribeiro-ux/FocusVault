<script lang="ts">
	// Learning page — Phase 3 implementation
	let activeTab = $state<'course' | 'language' | 'plan'>('course');

	const tabs = [
		{ id: 'course' as const, label: 'Course' },
		{ id: 'language' as const, label: 'Language' },
		{ id: 'plan' as const, label: 'Plan' }
	];
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-2xl font-bold tracking-tight">Learning</h2>
		<p class="text-sm" style="color: var(--color-text-muted)">
			Course, language track, and 30-day plan
		</p>
	</div>

	<!-- Subtabs -->
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

	<div
		class="rounded-xl border p-12 text-center"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p class="text-lg font-medium" style="color: var(--color-text-muted)">Coming in Phase 3</p>
	</div>
</div>

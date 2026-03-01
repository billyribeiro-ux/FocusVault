<script lang="ts">
	// Mission page — Phase 2 implementation
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-2xl font-bold tracking-tight">Mission</h2>
			<p class="text-sm" style="color: var(--color-text-muted)">
				One active mission at a time
			</p>
		</div>
		<button
			class="rounded-lg px-4 py-2 text-sm font-medium text-white"
			style="background-color: var(--color-accent)"
		>
			+ New Mission
		</button>
	</div>

	<div
		class="rounded-xl border p-12 text-center"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p class="text-lg font-medium" style="color: var(--color-text-muted)">No missions</p>
		<p class="mt-1 text-sm" style="color: var(--color-text-muted)">
			Create your first mission to start focusing
		</p>
	</div>
</div>

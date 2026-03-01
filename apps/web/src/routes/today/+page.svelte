<script lang="ts">
	// Today page — Phase 2 implementation
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-2xl font-bold tracking-tight">Today</h2>
		<p class="text-sm" style="color: var(--color-text-muted)">
			Your daily execution dashboard
		</p>
	</div>

	<!-- Active Mission Card -->
	<div
		class="rounded-xl border p-6"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p class="text-xs font-medium uppercase tracking-wider" style="color: var(--color-text-muted)">
			Active Mission
		</p>
		<p class="mt-1 text-lg font-semibold" style="color: var(--color-accent)">
			No active mission
		</p>
	</div>

	<!-- Cycle Timer Placeholder -->
	<div
		class="rounded-xl border p-6 text-center"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p class="font-mono text-4xl font-bold">50:00</p>
		<p class="mt-1 text-sm" style="color: var(--color-text-muted)">Focus cycle</p>
		<div class="mt-4 flex justify-center gap-3">
			<button
				class="rounded-lg px-4 py-2 text-sm font-medium text-white"
				style="background-color: var(--color-accent)"
			>
				Start Cycle
			</button>
		</div>
	</div>

	<!-- Watch / Build / Prove -->
	<div
		class="rounded-xl border p-6"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p class="mb-3 text-xs font-medium uppercase tracking-wider" style="color: var(--color-text-muted)">
			Daily Checklist
		</p>
		<div class="space-y-2">
			{#each ['Watch', 'Build', 'Prove'] as item}
				<label class="flex items-center gap-3">
					<input type="checkbox" class="h-4 w-4 rounded" />
					<span class="text-sm">{item}</span>
				</label>
			{/each}
		</div>
	</div>
</div>

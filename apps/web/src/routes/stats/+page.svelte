<script lang="ts">
	import { onMount } from 'svelte';
	import { dailyLogStore, type DailyLog } from '$lib/stores/daily-log.svelte';
	import { vaultStore } from '$lib/stores/vault.svelte';
	import { missionStore } from '$lib/stores/mission.svelte';
	import { courseStore } from '$lib/stores/course.svelte';
	import { languageStore } from '$lib/stores/language.svelte';

	onMount(() => {
		dailyLogStore.loadToday();
		dailyLogStore.loadRange();
		vaultStore.load();
		missionStore.load();
		courseStore.load();
		languageStore.load();
	});

	// ── Computation helpers ──

	let streak = $derived.by(() => {
		const sorted = [...dailyLogStore.logs].sort((a, b) => b.date.localeCompare(a.date));
		let count = 0;
		for (const log of sorted) {
			if (log.cycles_completed > 0) count++;
			else break;
		}
		return count;
	});

	let totalCycles = $derived.by(() => {
		return dailyLogStore.logs.reduce((sum, log) => sum + log.cycles_completed, 0);
	});

	let vaultItemCount = $derived(vaultStore.items.length);

	let inboxCount = $derived(vaultStore.items.filter((i) => i.status === 'inbox').length);

	let missionsCompleted = $derived(missionStore.items.filter((m) => m.status === 'done').length);

	// Last 7 days logs (most recent first)
	let last7 = $derived.by(() => {
		const sorted = [...dailyLogStore.logs].sort((a, b) => b.date.localeCompare(a.date));
		return sorted.slice(0, 7);
	});

	let maxCycles = $derived.by(() => {
		if (last7.length === 0) return 1;
		const max = Math.max(...last7.map((l) => l.cycles_completed));
		return max > 0 ? max : 1;
	});

	// Execution checklist rates
	let watchRate = $derived.by(() => {
		if (dailyLogStore.logs.length === 0) return 0;
		const done = dailyLogStore.logs.filter((l) => l.watch_done).length;
		return Math.round((done / dailyLogStore.logs.length) * 100);
	});

	let buildRate = $derived.by(() => {
		if (dailyLogStore.logs.length === 0) return 0;
		const done = dailyLogStore.logs.filter((l) => l.build_done).length;
		return Math.round((done / dailyLogStore.logs.length) * 100);
	});

	let proveRate = $derived.by(() => {
		if (dailyLogStore.logs.length === 0) return 0;
		const done = dailyLogStore.logs.filter((l) => l.prove_done).length;
		return Math.round((done / dailyLogStore.logs.length) * 100);
	});

	// Active constraints
	let activeMissionName = $derived(missionStore.activeMission?.name ?? 'None');
	let activeCourseName = $derived(courseStore.activeCourse?.name ?? 'None');
	let activeLanguageName = $derived(languageStore.activeTrack?.name ?? 'None');
	let tabLimit = $derived(dailyLogStore.today?.tab_limit ?? 10);

	// Format date for bar chart labels (e.g. "Mon 3/1")
	function formatDayLabel(dateStr: string): string {
		const d = new Date(dateStr + 'T00:00:00');
		const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
		const day = days[d.getDay()];
		const month = d.getMonth() + 1;
		const date = d.getDate();
		return `${day} ${month}/${date}`;
	}
</script>

<div class="page">
	<!-- Header -->
	<div class="header">
		<div>
			<h2 class="title">Stats</h2>
			<p class="subtitle">Streaks, trends, and weekly review</p>
		</div>
	</div>

	<!-- Section 1: Overview -->
	<section class="section">
		<p class="section-label">Overview</p>
		<div class="stat-grid">
			<div class="stat-card">
				<span class="stat-label">Current Streak</span>
				<span class="stat-value">{streak}</span>
				<span class="stat-secondary">days</span>
			</div>
			<div class="stat-card">
				<span class="stat-label">Total Cycles</span>
				<span class="stat-value">{totalCycles}</span>
				<span class="stat-secondary">all time</span>
			</div>
			<div class="stat-card">
				<span class="stat-label">Vault Items</span>
				<span class="stat-value">{vaultItemCount}</span>
				<span class="stat-secondary">{inboxCount} in inbox</span>
			</div>
			<div class="stat-card">
				<span class="stat-label">Missions Completed</span>
				<span class="stat-value">{missionsCompleted}</span>
				<span class="stat-secondary">total</span>
			</div>
		</div>
	</section>

	<!-- Section 2: Last 7 Days -->
	<section class="section">
		<p class="section-label">Last 7 Days</p>
		<div class="chart-card">
			{#if last7.length === 0}
				<p class="empty-text">No daily logs yet</p>
			{:else}
				{#each last7 as log (log.id)}
					<div class="bar-row">
						<span class="bar-date">{formatDayLabel(log.date)}</span>
						<div class="bar-track">
							<div
								class="bar-fill"
								style="width: {(log.cycles_completed / maxCycles) * 100}%"
							></div>
						</div>
						<span class="bar-count">{log.cycles_completed}</span>
						<div class="bar-indicators">
							<span class="indicator" class:indicator-done={log.watch_done} class:indicator-miss={!log.watch_done} title="Watch">{log.watch_done ? '\u2713' : '\u2717'}</span>
							<span class="indicator" class:indicator-done={log.build_done} class:indicator-miss={!log.build_done} title="Build">{log.build_done ? '\u2713' : '\u2717'}</span>
							<span class="indicator" class:indicator-done={log.prove_done} class:indicator-miss={!log.prove_done} title="Prove">{log.prove_done ? '\u2713' : '\u2717'}</span>
						</div>
					</div>
				{/each}
			{/if}
		</div>
	</section>

	<!-- Section 3: Execution Checklist Rates -->
	<section class="section">
		<p class="section-label">Execution Checklist Rates</p>
		<div class="rates-card">
			<div class="rate-row">
				<span class="rate-label">Watch</span>
				<div class="rate-track">
					<div class="rate-fill rate-fill-green" style="width: {watchRate}%"></div>
				</div>
				<span class="rate-pct">{watchRate}%</span>
			</div>
			<div class="rate-row">
				<span class="rate-label">Build</span>
				<div class="rate-track">
					<div class="rate-fill rate-fill-blue" style="width: {buildRate}%"></div>
				</div>
				<span class="rate-pct">{buildRate}%</span>
			</div>
			<div class="rate-row">
				<span class="rate-label">Prove</span>
				<div class="rate-track">
					<div class="rate-fill rate-fill-purple" style="width: {proveRate}%"></div>
				</div>
				<span class="rate-pct">{proveRate}%</span>
			</div>
		</div>
	</section>

	<!-- Section 4: Active Constraints -->
	<section class="section">
		<p class="section-label">Active Constraints</p>
		<div class="constraints-card">
			<div class="constraint-row">
				<span class="constraint-label">Active Mission</span>
				<span class="constraint-value" class:constraint-none={activeMissionName === 'None'}>{activeMissionName}</span>
			</div>
			<div class="constraint-row">
				<span class="constraint-label">Active Course</span>
				<span class="constraint-value" class:constraint-none={activeCourseName === 'None'}>{activeCourseName}</span>
			</div>
			<div class="constraint-row">
				<span class="constraint-label">Active Language</span>
				<span class="constraint-value" class:constraint-none={activeLanguageName === 'None'}>{activeLanguageName}</span>
			</div>
			<div class="constraint-row">
				<span class="constraint-label">Tab Limit</span>
				<span class="constraint-value">{tabLimit}</span>
			</div>
		</div>
	</section>
</div>

<style>
	.page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* ── Header ── */
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.title {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: -0.025em;
		margin: 0;
	}

	.subtitle {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.125rem 0 0;
	}

	/* ── Section ── */
	.section {
		display: flex;
		flex-direction: column;
	}

	.section-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin: 0 0 0.5rem;
	}

	/* ── Stat Grid (Overview) ── */
	.stat-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	@media (min-width: 768px) {
		.stat-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.stat-card {
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		letter-spacing: -0.025em;
		line-height: 1.1;
	}

	.stat-secondary {
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	/* ── Bar Chart (Last 7 Days) ── */
	.chart-card {
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		padding: 1rem 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	.empty-text {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		text-align: center;
		padding: 2rem 0;
		margin: 0;
	}

	.bar-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.bar-date {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		width: 4.5rem;
		flex-shrink: 0;
		text-align: right;
	}

	.bar-track {
		flex: 1;
		height: 1.25rem;
		background-color: var(--color-bg);
		border-radius: 0.375rem;
		overflow: hidden;
	}

	.bar-fill {
		height: 100%;
		background-color: var(--color-accent);
		border-radius: 0.375rem;
		min-width: 2px;
		transition: width 0.3s ease;
	}

	.bar-count {
		font-size: 0.8125rem;
		font-weight: 600;
		width: 1.5rem;
		text-align: right;
		flex-shrink: 0;
	}

	.bar-indicators {
		display: flex;
		gap: 0.25rem;
		flex-shrink: 0;
	}

	.indicator {
		font-size: 0.6875rem;
		width: 1.125rem;
		height: 1.125rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.25rem;
	}

	.indicator-done {
		color: var(--color-success);
		background-color: rgba(34, 197, 94, 0.12);
	}

	.indicator-miss {
		color: var(--color-danger);
		background-color: rgba(239, 68, 68, 0.1);
	}

	/* ── Execution Checklist Rates ── */
	.rates-card {
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.rate-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.rate-label {
		font-size: 0.8125rem;
		font-weight: 500;
		width: 3.5rem;
		flex-shrink: 0;
	}

	.rate-track {
		flex: 1;
		height: 0.625rem;
		background-color: var(--color-bg);
		border-radius: 9999px;
		overflow: hidden;
	}

	.rate-fill {
		height: 100%;
		border-radius: 9999px;
		transition: width 0.3s ease;
	}

	.rate-fill-green {
		background-color: var(--color-success);
	}

	.rate-fill-blue {
		background-color: var(--color-accent);
	}

	.rate-fill-purple {
		background-color: #a855f7;
	}

	.rate-pct {
		font-size: 0.8125rem;
		font-weight: 600;
		width: 2.5rem;
		text-align: right;
		flex-shrink: 0;
	}

	/* ── Active Constraints ── */
	.constraints-card {
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		overflow: hidden;
	}

	.constraint-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.875rem 1.25rem;
	}

	.constraint-row + .constraint-row {
		border-top: 1px solid var(--color-border);
	}

	.constraint-label {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
	}

	.constraint-value {
		font-size: 0.875rem;
		font-weight: 500;
	}

	.constraint-none {
		color: var(--color-text-muted);
		font-style: italic;
		font-weight: 400;
	}

	/* ── Responsive ── */
	@media (max-width: 640px) {
		.bar-row {
			gap: 0.5rem;
		}

		.bar-date {
			width: 3.5rem;
			font-size: 0.6875rem;
		}

		.bar-count {
			width: 1.25rem;
			font-size: 0.75rem;
		}
	}
</style>

<script lang="ts">
	import { missionStore, type Mission } from '$lib/stores/mission.svelte';
	import MissionCreateModal from '$lib/components/MissionCreateModal.svelte';

	let showCreateModal = $state(false);

	// Group non-active missions by status
	let pausedMissions = $derived(missionStore.items.filter((m) => m.status === 'paused'));
	let doneMissions = $derived(missionStore.items.filter((m) => m.status === 'done'));
	let activeMission = $derived(missionStore.activeMission);

	// KPI inline editing state
	let kpiCallsBooked = $state(0);
	let kpiRevenue = $state(0);
	let kpiLeads = $state(0);

	// Sync KPI state when active mission changes
	$effect(() => {
		if (activeMission?.kpis) {
			kpiCallsBooked = activeMission.kpis.calls_booked;
			kpiRevenue = activeMission.kpis.revenue;
			kpiLeads = activeMission.kpis.leads;
		}
	});

	// Load missions on mount
	$effect(() => {
		missionStore.load();
	});

	async function handleCreate(input: Parameters<typeof missionStore.create>[0]) {
		try {
			await missionStore.create(input);
			showCreateModal = false;
		} catch {
			// error is displayed via missionStore.error
		}
	}

	async function handlePause() {
		if (!activeMission) return;
		try {
			await missionStore.update(activeMission.id, { status: 'paused' });
		} catch {
			// error is displayed via missionStore.error
		}
	}

	async function handleComplete() {
		if (!activeMission) return;
		try {
			await missionStore.update(activeMission.id, { status: 'done' });
		} catch {
			// error is displayed via missionStore.error
		}
	}

	async function handleActivate(id: string) {
		try {
			await missionStore.activate(id);
		} catch {
			// error is displayed via missionStore.error
		}
	}

	async function handleKpiSave() {
		if (!activeMission) return;
		try {
			await missionStore.update(activeMission.id, {
				kpis: {
					calls_booked: kpiCallsBooked,
					revenue: kpiRevenue,
					leads: kpiLeads
				}
			});
		} catch {
			// error is displayed via missionStore.error
		}
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '--';
		const d = new Date(dateStr);
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function statusLabel(status: Mission['status']): string {
		switch (status) {
			case 'active':
				return 'Active';
			case 'paused':
				return 'Paused';
			case 'done':
				return 'Done';
		}
	}
</script>

<div class="page">
	<!-- Error Banner -->
	{#if missionStore.error}
		<div class="error-banner">
			<span>{missionStore.error}</span>
			<button class="error-dismiss" onclick={() => (missionStore.error = null)}>&times;</button>
		</div>
	{/if}

	<!-- Header -->
	<div class="header">
		<div>
			<h2 class="title">Mission</h2>
			<p class="subtitle">One active mission at a time</p>
		</div>
		<button class="btn-new" onclick={() => (showCreateModal = true)}>
			+ New Mission
		</button>
	</div>

	<!-- Loading -->
	{#if missionStore.loading && missionStore.items.length === 0}
		<div class="empty-card">
			<p class="empty-title">Loading missions...</p>
		</div>
	{:else}
		<!-- Active Mission Card -->
		<section class="active-section">
			<p class="section-label">Active Mission</p>
			{#if activeMission}
				<div class="active-card">
					<div class="active-header">
						<div>
							<h3 class="active-name">{activeMission.name}</h3>
							{#if activeMission.description}
								<p class="active-desc">{activeMission.description}</p>
							{/if}
						</div>
						<span class="badge badge-active">Active</span>
					</div>

					<div class="active-meta">
						<div class="meta-item">
							<span class="meta-label">Started</span>
							<span class="meta-value">{formatDate(activeMission.started_at)}</span>
						</div>
						<div class="meta-item">
							<span class="meta-label">Tab Limit</span>
							<span class="meta-value">{activeMission.tab_limit}</span>
						</div>
					</div>

					<!-- Weekly Targets -->
					{#if activeMission.weekly_targets && (activeMission.weekly_targets.deploy > 0 || activeMission.weekly_targets.outreach > 0 || activeMission.weekly_targets.iterate > 0)}
						<div class="targets-section">
							<p class="inner-label">Weekly Targets</p>
							<div class="targets-grid">
								{#if activeMission.weekly_targets.deploy > 0}
									<div class="target-chip">
										<span class="target-name">Deploy</span>
										<span class="target-count">{activeMission.weekly_targets.deploy}/wk</span>
									</div>
								{/if}
								{#if activeMission.weekly_targets.outreach > 0}
									<div class="target-chip">
										<span class="target-name">Outreach</span>
										<span class="target-count">{activeMission.weekly_targets.outreach}/wk</span>
									</div>
								{/if}
								{#if activeMission.weekly_targets.iterate > 0}
									<div class="target-chip">
										<span class="target-name">Iterate</span>
										<span class="target-count">{activeMission.weekly_targets.iterate}/wk</span>
									</div>
								{/if}
							</div>
						</div>
					{/if}

					<!-- KPI Quick Input -->
					{#if activeMission.kpis}
						<div class="kpi-section">
							<p class="inner-label">KPIs</p>
							<div class="kpi-grid">
								<div class="kpi-field">
									<label for="kpi-calls" class="kpi-label">Calls Booked</label>
									<input
										id="kpi-calls"
										type="number"
										class="kpi-input"
										min="0"
										bind:value={kpiCallsBooked}
									/>
								</div>
								<div class="kpi-field">
									<label for="kpi-revenue" class="kpi-label">Revenue</label>
									<input
										id="kpi-revenue"
										type="number"
										class="kpi-input"
										min="0"
										bind:value={kpiRevenue}
									/>
								</div>
								<div class="kpi-field">
									<label for="kpi-leads" class="kpi-label">Leads</label>
									<input
										id="kpi-leads"
										type="number"
										class="kpi-input"
										min="0"
										bind:value={kpiLeads}
									/>
								</div>
							</div>
							<button class="btn-save-kpi" onclick={handleKpiSave}>Save KPIs</button>
						</div>
					{/if}

					<!-- Actions -->
					<div class="active-actions">
						<button class="btn btn-warning" onclick={handlePause}>Pause</button>
						<button class="btn btn-success" onclick={handleComplete}>Complete</button>
					</div>
				</div>
			{:else}
				<div class="no-active-card">
					<p class="no-active-title">No active mission</p>
					<p class="no-active-hint">
						Create a new mission or activate a paused one to start focusing
					</p>
				</div>
			{/if}
		</section>

		<!-- Mission List -->
		{#if pausedMissions.length > 0 || doneMissions.length > 0}
			<section class="list-section">
				<!-- Paused Missions -->
				{#if pausedMissions.length > 0}
					<div class="group">
						<p class="group-label">Paused</p>
						<div class="mission-list">
							{#each pausedMissions as mission (mission.id)}
								<div class="mission-row">
									<div class="mission-info">
										<span class="mission-name">{mission.name}</span>
										<span class="badge badge-paused">Paused</span>
									</div>
									<div class="mission-dates">
										<span class="date-text">
											Started {formatDate(mission.started_at)}
										</span>
									</div>
									<button
										class="btn btn-activate"
										onclick={() => handleActivate(mission.id)}
									>
										Activate
									</button>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Done Missions -->
				{#if doneMissions.length > 0}
					<div class="group">
						<p class="group-label">Completed</p>
						<div class="mission-list">
							{#each doneMissions as mission (mission.id)}
								<div class="mission-row">
									<div class="mission-info">
										<span class="mission-name">{mission.name}</span>
										<span class="badge badge-done">Done</span>
									</div>
									<div class="mission-dates">
										<span class="date-text">
											{formatDate(mission.started_at)} &mdash; {formatDate(mission.ended_at)}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</section>
		{/if}

		<!-- Empty state: no missions at all -->
		{#if missionStore.items.length === 0 && !missionStore.loading}
			<div class="empty-card">
				<p class="empty-title">No missions</p>
				<p class="empty-hint">Create your first mission to start focusing</p>
			</div>
		{/if}
	{/if}
</div>

<!-- Create Modal -->
<MissionCreateModal
	open={showCreateModal}
	onclose={() => (showCreateModal = false)}
	oncreate={handleCreate}
/>

<style>
	.page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* ── Error Banner ── */
	.error-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		border-radius: 0.75rem;
		background-color: rgba(239, 68, 68, 0.12);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: var(--color-danger);
		font-size: 0.875rem;
	}
	.error-dismiss {
		background: none;
		border: none;
		color: var(--color-danger);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0;
		line-height: 1;
		flex-shrink: 0;
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

	.btn-new {
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
	}
	.btn-new:hover {
		background-color: var(--color-accent-hover);
	}

	/* ── Section Labels ── */
	.section-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin: 0 0 0.5rem;
	}

	.inner-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin: 0 0 0.5rem;
	}

	/* ── Active Mission Card ── */
	.active-section {
		display: flex;
		flex-direction: column;
	}

	.active-card {
		border-radius: 1rem;
		border: 1px solid var(--color-accent);
		background-color: var(--color-surface);
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.active-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.active-name {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.active-desc {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.25rem 0 0;
		line-height: 1.5;
	}

	.active-meta {
		display: flex;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.meta-label {
		font-size: 0.6875rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
	}

	.meta-value {
		font-size: 0.9375rem;
		font-weight: 500;
	}

	/* ── Weekly Targets ── */
	.targets-section {
		padding-top: 0.25rem;
	}

	.targets-grid {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.target-chip {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		border-radius: 0.5rem;
		background-color: var(--color-bg);
		border: 1px solid var(--color-border);
		font-size: 0.8125rem;
	}

	.target-name {
		font-weight: 500;
	}

	.target-count {
		color: var(--color-accent);
		font-weight: 600;
	}

	/* ── KPI Section ── */
	.kpi-section {
		padding-top: 0.25rem;
	}

	.kpi-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(7rem, 1fr));
		gap: 0.75rem;
	}

	.kpi-field {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.kpi-label {
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.kpi-input {
		padding: 0.375rem 0.5rem;
		border-radius: 0.375rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-bg);
		color: var(--color-text);
		font-size: 0.875rem;
		width: 100%;
		outline: none;
		font-family: inherit;
	}
	.kpi-input:focus {
		border-color: var(--color-accent);
	}

	.btn-save-kpi {
		margin-top: 0.5rem;
		padding: 0.375rem 0.75rem;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--color-accent);
		background-color: transparent;
		border: 1px solid var(--color-accent);
		cursor: pointer;
		transition: background-color 0.15s;
	}
	.btn-save-kpi:hover {
		background-color: rgba(59, 130, 246, 0.1);
	}

	/* ── Action Buttons ── */
	.active-actions {
		display: flex;
		gap: 0.75rem;
		padding-top: 0.25rem;
	}

	.btn {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: opacity 0.15s;
	}
	.btn:hover {
		opacity: 0.85;
	}

	.btn-warning {
		background-color: var(--color-warning);
		color: #000;
	}

	.btn-success {
		background-color: var(--color-success);
		color: #000;
	}

	.btn-activate {
		background-color: var(--color-accent);
		color: white;
		padding: 0.375rem 0.75rem;
		font-size: 0.8125rem;
		flex-shrink: 0;
	}

	/* ── No Active Card ── */
	.no-active-card {
		border-radius: 1rem;
		border: 1px dashed var(--color-border);
		background-color: var(--color-surface);
		padding: 3rem 1.5rem;
		text-align: center;
	}

	.no-active-title {
		font-size: 1.125rem;
		font-weight: 500;
		color: var(--color-text-muted);
		margin: 0;
	}

	.no-active-hint {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.375rem 0 0;
	}

	/* ── Mission List ── */
	.list-section {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.group {
		display: flex;
		flex-direction: column;
	}

	.group-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin: 0 0 0.5rem;
	}

	.mission-list {
		display: flex;
		flex-direction: column;
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		overflow: hidden;
	}

	.mission-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.875rem 1rem;
		flex-wrap: wrap;
	}
	.mission-row + .mission-row {
		border-top: 1px solid var(--color-border);
	}

	.mission-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.mission-name {
		font-size: 0.9375rem;
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.mission-dates {
		flex: 1;
		text-align: right;
	}

	.date-text {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		white-space: nowrap;
	}

	/* ── Badges ── */
	.badge {
		display: inline-flex;
		align-items: center;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		flex-shrink: 0;
	}

	.badge-active {
		background-color: rgba(59, 130, 246, 0.15);
		color: var(--color-accent);
	}

	.badge-paused {
		background-color: rgba(245, 158, 11, 0.15);
		color: var(--color-warning);
	}

	.badge-done {
		background-color: rgba(34, 197, 94, 0.15);
		color: var(--color-success);
	}

	/* ── Empty State ── */
	.empty-card {
		border-radius: 1rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		padding: 3rem 1.5rem;
		text-align: center;
	}

	.empty-title {
		font-size: 1.125rem;
		font-weight: 500;
		color: var(--color-text-muted);
		margin: 0;
	}

	.empty-hint {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.375rem 0 0;
	}

	/* ── Responsive ── */
	@media (max-width: 640px) {
		.active-meta {
			gap: 1rem;
		}

		.kpi-grid {
			grid-template-columns: 1fr 1fr 1fr;
		}

		.mission-row {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}

		.mission-dates {
			text-align: left;
		}
	}
</style>

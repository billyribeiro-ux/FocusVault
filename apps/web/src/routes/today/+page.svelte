<script lang="ts">
	import { dailyLogStore } from '$lib/stores/daily-log.svelte';
	import { missionStore } from '$lib/stores/mission.svelte';
	import { vaultStore } from '$lib/stores/vault.svelte';

	// Timer state
	const FOCUS_DURATION = 50 * 60; // 50 minutes in seconds
	const BREAK_DURATION = 10 * 60; // 10 minutes in seconds

	let timeLeft = $state(FOCUS_DURATION);
	let isRunning = $state(false);
	let isBreak = $state(false);

	// End-of-day form state
	let showEndOfDay = $state(false);
	let tabLimitMet = $state<boolean | null>(null);
	let focusNotes = $state('');

	// Plan day selector
	let planDay = $state(1);

	// Timer countdown effect
	$effect(() => {
		if (!isRunning) return;

		const interval = setInterval(() => {
			timeLeft -= 1;
			if (timeLeft <= 0) {
				if (!isBreak) {
					// Focus cycle completed
					dailyLogStore.incrementCycle();
					isBreak = true;
					timeLeft = BREAK_DURATION;
				} else {
					// Break completed, back to focus
					isBreak = false;
					timeLeft = FOCUS_DURATION;
					isRunning = false;
				}
			}
		}, 1000);

		return () => clearInterval(interval);
	});

	// Load stores on mount
	$effect(() => {
		dailyLogStore.loadToday();
		missionStore.load();
		vaultStore.load();
	});

	// Format seconds as MM:SS
	function formatTime(seconds: number): string {
		const m = Math.floor(seconds / 60);
		const s = seconds % 60;
		return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
	}

	function startPause() {
		isRunning = !isRunning;
	}

	function resetTimer() {
		isRunning = false;
		isBreak = false;
		timeLeft = FOCUS_DURATION;
	}

	function handlePlanDayChange(value: number) {
		const clamped = Math.max(1, Math.min(30, value));
		planDay = clamped;
		dailyLogStore.updateToday({ plan_day: clamped });
	}

	async function handleEndOfDay() {
		await dailyLogStore.updateToday({
			tab_limit_met: tabLimitMet ?? false,
			focus_notes: focusNotes
		});
		showEndOfDay = false;
		tabLimitMet = null;
		focusNotes = '';
	}
</script>

<div class="space-y-6">
	<!-- Page Header -->
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
		<p
			class="text-xs font-medium uppercase tracking-wider"
			style="color: var(--color-text-muted)"
		>
			Active Mission
		</p>
		{#if missionStore.activeMission}
			<p class="mt-1 text-lg font-semibold" style="color: var(--color-accent)">
				{missionStore.activeMission.name}
			</p>
			{#if missionStore.activeMission.description}
				<p class="mt-1 text-sm" style="color: var(--color-text-muted)">
					{missionStore.activeMission.description}
				</p>
			{/if}
		{:else}
			<p class="mt-1 text-lg font-semibold" style="color: var(--color-text-muted)">
				No active mission
			</p>
			<a
				href="/mission"
				class="mt-2 inline-block text-sm font-medium underline underline-offset-4"
				style="color: var(--color-accent)"
			>
				Set a mission
			</a>
		{/if}
	</div>

	<!-- 50/10 Cycle Timer -->
	<div
		class="rounded-xl border p-6 text-center"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p
			class="text-xs font-medium uppercase tracking-wider"
			style="color: var(--color-text-muted)"
		>
			{isBreak ? 'Break Time' : 'Focus Cycle'}
		</p>

		<p
			class="mt-3 font-mono text-5xl font-bold"
			style="color: {isBreak ? 'var(--color-success)' : 'var(--color-text)'}"
		>
			{formatTime(timeLeft)}
		</p>

		<p class="mt-2 text-sm" style="color: var(--color-text-muted)">
			Cycles: {dailyLogStore.today?.cycles_completed ?? 0}
		</p>

		<div class="mt-4 flex justify-center gap-3">
			<button
				class="rounded-lg px-5 py-2 text-sm font-medium text-white transition-colors"
				style="background-color: {isRunning ? 'var(--color-warning)' : 'var(--color-accent)'}"
				onclick={startPause}
			>
				{isRunning ? 'Pause' : 'Start'}
			</button>
			<button
				class="rounded-lg border px-5 py-2 text-sm font-medium transition-colors"
				style="border-color: var(--color-border); color: var(--color-text-muted)"
				onclick={resetTimer}
			>
				Reset
			</button>
		</div>
	</div>

	<!-- Two-column grid for Plan Day + Open Loops -->
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
		<!-- Plan Day Selector -->
		<div
			class="rounded-xl border p-6"
			style="background-color: var(--color-surface); border-color: var(--color-border)"
		>
			<p
				class="text-xs font-medium uppercase tracking-wider"
				style="color: var(--color-text-muted)"
			>
				Plan Day
			</p>
			<div class="mt-3 flex items-center gap-3">
				<input
					type="number"
					min="1"
					max="30"
					bind:value={planDay}
					onchange={() => handlePlanDayChange(planDay)}
					class="w-20 rounded-lg border px-3 py-2 text-center font-mono text-lg font-bold"
					style="background-color: var(--color-bg); border-color: var(--color-border); color: var(--color-text)"
				/>
				<span class="text-sm" style="color: var(--color-text-muted)">of 30</span>
			</div>
		</div>

		<!-- Open Loops Counter -->
		<div
			class="rounded-xl border p-6"
			style="background-color: var(--color-surface); border-color: var(--color-border)"
		>
			<p
				class="text-xs font-medium uppercase tracking-wider"
				style="color: var(--color-text-muted)"
			>
				Open Loops
			</p>
			<div class="mt-3 flex items-baseline gap-2">
				<span class="font-mono text-3xl font-bold" style="color: var(--color-warning)">
					{vaultStore.inboxCount}
				</span>
				<span class="text-sm" style="color: var(--color-text-muted)">items in inbox</span>
			</div>
		</div>
	</div>

	<!-- Watch / Build / Prove Checklist -->
	<div
		class="rounded-xl border p-6"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<p
			class="mb-4 text-xs font-medium uppercase tracking-wider"
			style="color: var(--color-text-muted)"
		>
			Daily Checklist
		</p>
		<div class="space-y-3">
			<label class="flex cursor-pointer items-center gap-3">
				<input
					type="checkbox"
					checked={dailyLogStore.today?.watch_done ?? false}
					onchange={() => dailyLogStore.toggleWatch()}
					class="h-5 w-5 rounded accent-[var(--color-accent)]"
				/>
				<span
					class="text-sm font-medium"
					style="color: {dailyLogStore.today?.watch_done ? 'var(--color-success)' : 'var(--color-text)'}"
				>
					Watch
				</span>
				<span class="text-xs" style="color: var(--color-text-muted)">
					-- Learn something new
				</span>
			</label>
			<label class="flex cursor-pointer items-center gap-3">
				<input
					type="checkbox"
					checked={dailyLogStore.today?.build_done ?? false}
					onchange={() => dailyLogStore.toggleBuild()}
					class="h-5 w-5 rounded accent-[var(--color-accent)]"
				/>
				<span
					class="text-sm font-medium"
					style="color: {dailyLogStore.today?.build_done ? 'var(--color-success)' : 'var(--color-text)'}"
				>
					Build
				</span>
				<span class="text-xs" style="color: var(--color-text-muted)">
					-- Ship tangible work
				</span>
			</label>
			<label class="flex cursor-pointer items-center gap-3">
				<input
					type="checkbox"
					checked={dailyLogStore.today?.prove_done ?? false}
					onchange={() => dailyLogStore.toggleProve()}
					class="h-5 w-5 rounded accent-[var(--color-accent)]"
				/>
				<span
					class="text-sm font-medium"
					style="color: {dailyLogStore.today?.prove_done ? 'var(--color-success)' : 'var(--color-text)'}"
				>
					Prove
				</span>
				<span class="text-xs" style="color: var(--color-text-muted)">
					-- Show your progress
				</span>
			</label>
		</div>
	</div>

	<!-- End of Day -->
	{#if !showEndOfDay}
		<button
			class="w-full rounded-xl border px-6 py-4 text-sm font-medium transition-colors"
			style="background-color: var(--color-surface); border-color: var(--color-border); color: var(--color-text-muted)"
			onclick={() => (showEndOfDay = true)}
		>
			End of Day
		</button>
	{:else}
		<div
			class="rounded-xl border p-6"
			style="background-color: var(--color-surface); border-color: var(--color-border)"
		>
			<p
				class="mb-4 text-xs font-medium uppercase tracking-wider"
				style="color: var(--color-text-muted)"
			>
				End of Day Review
			</p>

			<!-- Tab Limit -->
			<div class="mb-4">
				<p class="mb-2 text-sm font-medium">Did you stay within your tab limit?</p>
				<div class="flex gap-3">
					<button
						class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors"
						style="
							border-color: var(--color-border);
							background-color: {tabLimitMet === true ? 'var(--color-success)' : 'transparent'};
							color: {tabLimitMet === true ? 'white' : 'var(--color-text-muted)'}
						"
						onclick={() => (tabLimitMet = true)}
					>
						Yes
					</button>
					<button
						class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors"
						style="
							border-color: var(--color-border);
							background-color: {tabLimitMet === false ? 'var(--color-danger)' : 'transparent'};
							color: {tabLimitMet === false ? 'white' : 'var(--color-text-muted)'}
						"
						onclick={() => (tabLimitMet = false)}
					>
						No
					</button>
				</div>
			</div>

			<!-- Focus Notes -->
			<div class="mb-4">
				<p class="mb-2 text-sm font-medium">Focus notes</p>
				<textarea
					bind:value={focusNotes}
					rows={3}
					placeholder="What did you accomplish? What got in the way?"
					class="w-full rounded-lg border px-3 py-2 text-sm"
					style="background-color: var(--color-bg); border-color: var(--color-border); color: var(--color-text); resize: vertical"
				></textarea>
			</div>

			<!-- Submit / Cancel -->
			<div class="flex gap-3">
				<button
					class="rounded-lg px-5 py-2 text-sm font-medium text-white transition-colors"
					style="background-color: var(--color-accent)"
					onclick={handleEndOfDay}
				>
					Save &amp; Close Day
				</button>
				<button
					class="rounded-lg border px-5 py-2 text-sm font-medium transition-colors"
					style="border-color: var(--color-border); color: var(--color-text-muted)"
					onclick={() => (showEndOfDay = false)}
				>
					Cancel
				</button>
			</div>
		</div>
	{/if}

	<!-- Loading / Error States -->
	{#if dailyLogStore.loading}
		<div class="py-4 text-center text-sm" style="color: var(--color-text-muted)">
			Loading today's log...
		</div>
	{/if}

	{#if dailyLogStore.error}
		<div
			class="rounded-xl border px-4 py-3 text-sm"
			style="border-color: var(--color-danger); color: var(--color-danger)"
		>
			{dailyLogStore.error}
		</div>
	{/if}
</div>

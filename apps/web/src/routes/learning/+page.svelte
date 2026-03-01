<script lang="ts">
	import { onMount } from 'svelte';
	import { courseStore, type Course, type CreateCourseInput } from '$lib/stores/course.svelte';
	import {
		languageStore,
		type LanguageTrack,
		type CreateLanguageTrackInput
	} from '$lib/stores/language.svelte';
	import CourseCreateModal from '$lib/components/CourseCreateModal.svelte';
	import LanguageCreateModal from '$lib/components/LanguageCreateModal.svelte';

	let showCourseModal = $state(false);
	let showLanguageModal = $state(false);

	// Course derived state
	let activeCourse = $derived(courseStore.activeCourse);
	let pausedCourses = $derived(courseStore.items.filter((c) => c.status === 'paused'));
	let completedCourses = $derived(courseStore.items.filter((c) => c.status === 'completed'));

	// Language derived state
	let activeLanguage = $derived(languageStore.activeTrack);
	let pausedLanguages = $derived(languageStore.items.filter((t) => t.status === 'paused'));
	let completedLanguages = $derived(languageStore.items.filter((t) => t.status === 'completed'));

	// Load both stores on mount
	onMount(() => {
		courseStore.load();
		languageStore.load();
	});

	// ── Formatting helpers ──

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '--';
		const d = new Date(dateStr);
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function formatMinutes(totalMinutes: number): string {
		const h = Math.floor(totalMinutes / 60);
		const m = totalMinutes % 60;
		if (h === 0) return `${m}m`;
		if (m === 0) return `${h}h`;
		return `${h}h ${m}m`;
	}

	function formatWeeklyGoal(totalMinutes: number): string {
		return `${formatMinutes(totalMinutes)} / week`;
	}

	// ── Course handlers ──

	async function handleCreateCourse(input: CreateCourseInput) {
		try {
			await courseStore.create(input);
			showCourseModal = false;
		} catch {
			// error displayed via courseStore.error
		}
	}

	function handlePauseCourse() {
		// Note: no update endpoint yet, so pause is a placeholder.
		// TODO: Replace with courseStore.update(id, { status: 'paused' }) when available.
		courseStore.error = 'Pause is not yet supported by the API. Use Complete instead.';
	}

	async function handleCompleteCourse() {
		if (!activeCourse) return;
		try {
			await courseStore.complete(activeCourse.id);
		} catch {
			// error displayed via courseStore.error
		}
	}

	async function handleActivateCourse(id: string) {
		try {
			await courseStore.activate(id);
		} catch {
			// error displayed via courseStore.error
		}
	}

	// ── Language handlers ──

	async function handleCreateLanguage(input: CreateLanguageTrackInput) {
		try {
			await languageStore.create(input);
			showLanguageModal = false;
		} catch {
			// error displayed via languageStore.error
		}
	}

	async function handleCompleteLanguage() {
		if (!activeLanguage) return;
		try {
			await languageStore.complete(activeLanguage.id);
		} catch {
			// error displayed via languageStore.error
		}
	}

	async function handleActivateLanguage(id: string) {
		try {
			await languageStore.activate(id);
		} catch {
			// error displayed via languageStore.error
		}
	}
</script>

<div class="page">
	<!-- ═══════════════════════════════════════════ -->
	<!-- SECTION 1: COURSE                          -->
	<!-- ═══════════════════════════════════════════ -->

	<!-- Course Error Banner -->
	{#if courseStore.error}
		<div class="error-banner">
			<span>{courseStore.error}</span>
			<button class="error-dismiss" onclick={() => (courseStore.error = null)}>&times;</button>
		</div>
	{/if}

	<!-- Course Header -->
	<div class="header">
		<div>
			<h2 class="title">Course</h2>
			<p class="subtitle">One active course at a time</p>
		</div>
		<button class="btn-new" onclick={() => (showCourseModal = true)}>
			+ New Course
		</button>
	</div>

	<!-- Course Loading -->
	{#if courseStore.loading && courseStore.items.length === 0}
		<div class="empty-card">
			<p class="empty-title">Loading courses...</p>
		</div>
	{:else}
		<!-- Active Course Card -->
		<section class="active-section">
			<p class="section-label">Active Course</p>
			{#if activeCourse}
				<div class="active-card">
					<div class="active-header">
						<div>
							<h3 class="active-name">{activeCourse.name}</h3>
							{#if activeCourse.provider}
								<p class="active-provider">{activeCourse.provider}</p>
							{/if}
						</div>
						<span class="badge badge-active">Active</span>
					</div>

					<div class="active-meta">
						{#if activeCourse.url}
							<div class="meta-item">
								<span class="meta-label">URL</span>
								<a
									class="meta-link"
									href={activeCourse.url}
									target="_blank"
									rel="noopener noreferrer"
								>
									{activeCourse.url}
								</a>
							</div>
						{/if}
						<div class="meta-item">
							<span class="meta-label">Started</span>
							<span class="meta-value">{formatDate(activeCourse.started_at)}</span>
						</div>
						<div class="meta-item">
							<span class="meta-label">Progress</span>
							<span class="meta-value">{formatMinutes(activeCourse.progress_minutes)}</span>
						</div>
					</div>

					{#if activeCourse.progress_notes}
						<div class="notes-section">
							<p class="inner-label">Progress Notes</p>
							<p class="notes-text">{activeCourse.progress_notes}</p>
						</div>
					{/if}

					<!-- Actions -->
					<div class="active-actions">
						<button class="btn btn-warning" onclick={handlePauseCourse}>Pause</button>
						<button class="btn btn-success" onclick={handleCompleteCourse}>Complete</button>
					</div>
				</div>
			{:else}
				<div class="no-active-card">
					<p class="no-active-title">No active course</p>
					<p class="no-active-hint">
						Create a new course or activate a paused one to start learning
					</p>
				</div>
			{/if}
		</section>

		<!-- Course Lists -->
		{#if pausedCourses.length > 0 || completedCourses.length > 0}
			<section class="list-section">
				<!-- Paused Courses -->
				{#if pausedCourses.length > 0}
					<div class="group">
						<p class="group-label">Paused</p>
						<div class="item-list">
							{#each pausedCourses as course (course.id)}
								<div class="item-row">
									<div class="item-info">
										<span class="item-name">{course.name}</span>
										{#if course.provider}
											<span class="item-detail">{course.provider}</span>
										{/if}
									</div>
									<button
										class="btn btn-activate"
										onclick={() => handleActivateCourse(course.id)}
									>
										Activate
									</button>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Completed Courses -->
				{#if completedCourses.length > 0}
					<div class="group">
						<p class="group-label">Completed</p>
						<div class="item-list">
							{#each completedCourses as course (course.id)}
								<div class="item-row">
									<div class="item-info">
										<span class="item-name">{course.name}</span>
										{#if course.provider}
											<span class="item-detail">{course.provider}</span>
										{/if}
									</div>
									<div class="item-dates">
										<span class="date-text">
											{formatDate(course.started_at)} &mdash; {formatDate(course.completed_at)}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</section>
		{/if}

		<!-- Empty state: no courses at all -->
		{#if courseStore.items.length === 0 && !courseStore.loading}
			<div class="empty-card">
				<p class="empty-title">No courses</p>
				<p class="empty-hint">Create your first course to start tracking your learning</p>
			</div>
		{/if}
	{/if}

	<!-- ═══════════════════════════════════════════ -->
	<!-- DIVIDER                                     -->
	<!-- ═══════════════════════════════════════════ -->
	<hr class="section-divider" />

	<!-- ═══════════════════════════════════════════ -->
	<!-- SECTION 2: LANGUAGE TRACK                   -->
	<!-- ═══════════════════════════════════════════ -->

	<!-- Language Error Banner -->
	{#if languageStore.error}
		<div class="error-banner">
			<span>{languageStore.error}</span>
			<button class="error-dismiss" onclick={() => (languageStore.error = null)}>&times;</button>
		</div>
	{/if}

	<!-- Language Header -->
	<div class="header">
		<div>
			<h2 class="title">Language Track</h2>
			<p class="subtitle">One active language track at a time</p>
		</div>
		<button class="btn-new" onclick={() => (showLanguageModal = true)}>
			+ New Language
		</button>
	</div>

	<!-- Language Loading -->
	{#if languageStore.loading && languageStore.items.length === 0}
		<div class="empty-card">
			<p class="empty-title">Loading language tracks...</p>
		</div>
	{:else}
		<!-- Active Language Card -->
		<section class="active-section">
			<p class="section-label">Active Language Track</p>
			{#if activeLanguage}
				<div class="active-card">
					<div class="active-header">
						<div>
							<h3 class="active-name">{activeLanguage.name}</h3>
						</div>
						<span class="badge badge-active">Active</span>
					</div>

					<div class="active-meta">
						<div class="meta-item">
							<span class="meta-label">Weekly Goal</span>
							<span class="meta-value">{formatWeeklyGoal(activeLanguage.weekly_goal_minutes)}</span>
						</div>
						<div class="meta-item">
							<span class="meta-label">Started</span>
							<span class="meta-value">{formatDate(activeLanguage.started_at)}</span>
						</div>
					</div>

					{#if activeLanguage.notes}
						<div class="notes-section">
							<p class="inner-label">Notes</p>
							<p class="notes-text">{activeLanguage.notes}</p>
						</div>
					{/if}

					<!-- Actions -->
					<div class="active-actions">
						<button class="btn btn-success" onclick={handleCompleteLanguage}>Complete</button>
					</div>
				</div>
			{:else}
				<div class="no-active-card">
					<p class="no-active-title">No active language track</p>
					<p class="no-active-hint">
						Create a new language track or activate a paused one to start studying
					</p>
				</div>
			{/if}
		</section>

		<!-- Language Lists -->
		{#if pausedLanguages.length > 0 || completedLanguages.length > 0}
			<section class="list-section">
				<!-- Paused Languages -->
				{#if pausedLanguages.length > 0}
					<div class="group">
						<p class="group-label">Paused</p>
						<div class="item-list">
							{#each pausedLanguages as track (track.id)}
								<div class="item-row">
									<div class="item-info">
										<span class="item-name">{track.name}</span>
										<span class="item-detail">{formatWeeklyGoal(track.weekly_goal_minutes)}</span>
									</div>
									<button
										class="btn btn-activate"
										onclick={() => handleActivateLanguage(track.id)}
									>
										Activate
									</button>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Completed Languages -->
				{#if completedLanguages.length > 0}
					<div class="group">
						<p class="group-label">Completed</p>
						<div class="item-list">
							{#each completedLanguages as track (track.id)}
								<div class="item-row">
									<div class="item-info">
										<span class="item-name">{track.name}</span>
									</div>
									<div class="item-dates">
										<span class="date-text">
											{formatDate(track.started_at)} &mdash; {formatDate(track.completed_at)}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</section>
		{/if}

		<!-- Empty state: no language tracks at all -->
		{#if languageStore.items.length === 0 && !languageStore.loading}
			<div class="empty-card">
				<p class="empty-title">No language tracks</p>
				<p class="empty-hint">Create your first language track to start studying</p>
			</div>
		{/if}
	{/if}
</div>

<!-- Create Modals -->
<CourseCreateModal
	open={showCourseModal}
	onclose={() => (showCourseModal = false)}
	oncreate={handleCreateCourse}
/>

<LanguageCreateModal
	open={showLanguageModal}
	onclose={() => (showLanguageModal = false)}
	oncreate={handleCreateLanguage}
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

	/* ── Section Divider ── */
	.section-divider {
		border: none;
		border-top: 1px solid var(--color-border);
		margin: 0.5rem 0;
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

	/* ── Active Card ── */
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

	.active-provider {
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
		min-width: 0;
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

	.meta-link {
		font-size: 0.875rem;
		color: var(--color-accent);
		text-decoration: none;
		word-break: break-all;
	}
	.meta-link:hover {
		text-decoration: underline;
	}

	/* ── Notes Section ── */
	.notes-section {
		padding-top: 0.25rem;
	}

	.notes-text {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		line-height: 1.6;
		margin: 0;
		white-space: pre-wrap;
		padding: 0.75rem;
		background-color: var(--color-bg);
		border-radius: 0.5rem;
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

	/* ── Item Lists ── */
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

	.item-list {
		display: flex;
		flex-direction: column;
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		overflow: hidden;
	}

	.item-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.875rem 1rem;
		flex-wrap: wrap;
	}
	.item-row + .item-row {
		border-top: 1px solid var(--color-border);
	}

	.item-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.item-name {
		font-size: 0.9375rem;
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-detail {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		white-space: nowrap;
	}

	.item-dates {
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

		.item-row {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}

		.item-dates {
			text-align: left;
		}
	}
</style>

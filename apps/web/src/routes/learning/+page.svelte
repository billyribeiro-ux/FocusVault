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

	let activeCourse = $derived(courseStore.activeCourse);
	let pausedCourses = $derived(courseStore.items.filter((c) => c.status === 'paused'));
	let completedCourses = $derived(courseStore.items.filter((c) => c.status === 'completed'));

	let activeTrack = $derived(languageStore.activeTrack);
	let pausedTracks = $derived(languageStore.items.filter((t) => t.status === 'paused'));
	let completedTracks = $derived(languageStore.items.filter((t) => t.status === 'completed'));

	onMount(() => {
		courseStore.load();
		languageStore.load();
	});

	function formatMinutes(mins: number): string {
		const h = Math.floor(mins / 60);
		const m = mins % 60;
		if (h === 0) return `${m}m`;
		if (m === 0) return `${h}h`;
		return `${h}h ${m}m`;
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '--';
		const d = new Date(dateStr);
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	async function handleCreateCourse(input: CreateCourseInput) {
		try {
			await courseStore.create(input);
			showCourseModal = false;
		} catch {
			// error displayed via courseStore.error
		}
	}

	async function handleCreateLanguage(input: CreateLanguageTrackInput) {
		try {
			await languageStore.create(input);
			showLanguageModal = false;
		} catch {
			// error displayed via languageStore.error
		}
	}

	async function handleCompleteCourse(id: string) {
		try {
			await courseStore.complete(id);
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

	async function handleCompleteLanguage(id: string) {
		try {
			await languageStore.complete(id);
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
	<!-- Header -->
	<div class="page-header">
		<h2 class="page-title">Learning</h2>
		<p class="page-subtitle">One active course + one active language at a time</p>
	</div>

	<!-- ═══════════════════════════════════════ COURSE SECTION ═══ -->
	<section class="section">
		<div class="section-header">
			<p class="section-label">Course</p>
			<button class="btn-new" onclick={() => (showCourseModal = true)}>+ New Course</button>
		</div>

		<!-- Error -->
		{#if courseStore.error}
			<div class="error-banner">
				<span>{courseStore.error}</span>
				<button class="error-dismiss" onclick={() => (courseStore.error = null)}>&times;</button>
			</div>
		{/if}

		<!-- Loading -->
		{#if courseStore.loading && courseStore.items.length === 0}
			<div class="empty-card">
				<p class="empty-title">Loading courses...</p>
			</div>
		{:else}
			<!-- Active Course -->
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
						<div class="meta-item">
							<span class="meta-label">Started</span>
							<span class="meta-value">{formatDate(activeCourse.started_at)}</span>
						</div>
						<div class="meta-item">
							<span class="meta-label">Progress</span>
							<span class="meta-value">{formatMinutes(activeCourse.progress_minutes)}</span>
						</div>
						{#if activeCourse.url}
							<div class="meta-item">
								<span class="meta-label">Link</span>
								<a
									href={activeCourse.url}
									target="_blank"
									rel="noopener noreferrer"
									class="meta-link"
								>
									Open course
								</a>
							</div>
						{/if}
					</div>

					{#if activeCourse.progress_notes}
						<p class="progress-notes">{activeCourse.progress_notes}</p>
					{/if}

					<div class="active-actions">
						<button class="btn btn-success" onclick={() => handleCompleteCourse(activeCourse!.id)}>
							Complete
						</button>
					</div>
				</div>
			{:else}
				<div class="no-active-card">
					<p class="no-active-title">No active course</p>
					<p class="no-active-hint">Create or activate a course to start learning</p>
				</div>
			{/if}

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
									<span class="badge badge-paused">Paused</span>
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
									<span class="badge badge-done">Done</span>
								</div>
								<span class="date-text">
									{formatDate(course.started_at)} &mdash; {formatDate(course.completed_at)}
								</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Empty: no courses at all -->
			{#if courseStore.items.length === 0 && !courseStore.loading}
				<div class="empty-card">
					<p class="empty-title">No courses</p>
					<p class="empty-hint">Add your first course to start tracking</p>
				</div>
			{/if}
		{/if}
	</section>

	<!-- ═══════════════════════════════ LANGUAGE SECTION ═══ -->
	<section class="section">
		<div class="section-header">
			<p class="section-label">Language Track</p>
			<button class="btn-new" onclick={() => (showLanguageModal = true)}>+ New Language</button>
		</div>

		<!-- Error -->
		{#if languageStore.error}
			<div class="error-banner">
				<span>{languageStore.error}</span>
				<button class="error-dismiss" onclick={() => (languageStore.error = null)}
					>&times;</button
				>
			</div>
		{/if}

		<!-- Loading -->
		{#if languageStore.loading && languageStore.items.length === 0}
			<div class="empty-card">
				<p class="empty-title">Loading language tracks...</p>
			</div>
		{:else}
			<!-- Active Language Track -->
			{#if activeTrack}
				<div class="active-card">
					<div class="active-header">
						<div>
							<h3 class="active-name">{activeTrack.name}</h3>
						</div>
						<span class="badge badge-active">Active</span>
					</div>

					<div class="active-meta">
						<div class="meta-item">
							<span class="meta-label">Started</span>
							<span class="meta-value">{formatDate(activeTrack.started_at)}</span>
						</div>
						<div class="meta-item">
							<span class="meta-label">Weekly Goal</span>
							<span class="meta-value"
								>{formatMinutes(activeTrack.weekly_goal_minutes)} / week</span
							>
						</div>
					</div>

					{#if activeTrack.notes}
						<p class="progress-notes">{activeTrack.notes}</p>
					{/if}

					<div class="active-actions">
						<button
							class="btn btn-success"
							onclick={() => handleCompleteLanguage(activeTrack!.id)}
						>
							Complete
						</button>
					</div>
				</div>
			{:else}
				<div class="no-active-card">
					<p class="no-active-title">No active language track</p>
					<p class="no-active-hint">Create or activate a language track to start practicing</p>
				</div>
			{/if}

			<!-- Paused Language Tracks -->
			{#if pausedTracks.length > 0}
				<div class="group">
					<p class="group-label">Paused</p>
					<div class="item-list">
						{#each pausedTracks as track (track.id)}
							<div class="item-row">
								<div class="item-info">
									<span class="item-name">{track.name}</span>
									<span class="item-detail"
										>{formatMinutes(track.weekly_goal_minutes)}/wk</span
									>
									<span class="badge badge-paused">Paused</span>
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

			<!-- Completed Language Tracks -->
			{#if completedTracks.length > 0}
				<div class="group">
					<p class="group-label">Completed</p>
					<div class="item-list">
						{#each completedTracks as track (track.id)}
							<div class="item-row">
								<div class="item-info">
									<span class="item-name">{track.name}</span>
									<span class="badge badge-done">Done</span>
								</div>
								<span class="date-text">
									{formatDate(track.started_at)} &mdash; {formatDate(track.completed_at)}
								</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Empty: no language tracks at all -->
			{#if languageStore.items.length === 0 && !languageStore.loading}
				<div class="empty-card">
					<p class="empty-title">No language tracks</p>
					<p class="empty-hint">Add your first language to start tracking</p>
				</div>
			{/if}
		{/if}
	</section>
</div>

<!-- Modals -->
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
		gap: 2rem;
	}

	/* ── Page Header ── */
	.page-header {
		display: flex;
		flex-direction: column;
	}

	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: -0.025em;
		margin: 0;
	}

	.page-subtitle {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.125rem 0 0;
	}

	/* ── Section ── */
	.section {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.section-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin: 0;
	}

	.btn-new {
		padding: 0.375rem 0.75rem;
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: white;
		background-color: var(--color-accent);
		border: none;
		cursor: pointer;
		transition: background-color 0.15s;
		white-space: nowrap;
		font-family: inherit;
	}
	.btn-new:hover {
		background-color: var(--color-accent-hover);
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

	/* ── Active Card ── */
	.active-card {
		border-radius: 1rem;
		border: 1px solid var(--color-accent);
		background-color: var(--color-surface);
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
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
		margin: 0.125rem 0 0;
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

	.meta-link {
		font-size: 0.875rem;
		color: var(--color-accent);
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.progress-notes {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0;
		line-height: 1.5;
		padding: 0.75rem;
		background-color: var(--color-bg);
		border-radius: 0.5rem;
	}

	.active-actions {
		display: flex;
		gap: 0.75rem;
		padding-top: 0.25rem;
	}

	/* ── No Active Card ── */
	.no-active-card {
		border-radius: 1rem;
		border: 1px dashed var(--color-border);
		background-color: var(--color-surface);
		padding: 2.5rem 1.5rem;
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

	/* ── Buttons ── */
	.btn {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: opacity 0.15s;
		font-family: inherit;
	}
	.btn:hover {
		opacity: 0.85;
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

	/* ── Groups / Lists ── */
	.group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.group-label {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		margin: 0;
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
		flex-wrap: wrap;
	}

	.item-name {
		font-size: 0.9375rem;
		font-weight: 500;
	}

	.item-detail {
		font-size: 0.75rem;
		color: var(--color-text-muted);
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
		padding: 2.5rem 1.5rem;
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
	}
</style>

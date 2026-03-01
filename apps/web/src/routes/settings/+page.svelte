<script lang="ts">
	import { onMount } from 'svelte';

	let theme = $state<'dark' | 'light'>('dark');
	let defaultTabLimit = $state(10);
	let captureCap = $state<number | null>(null);

	const lightThemeVars: Record<string, string> = {
		'--color-bg': '#fafafa',
		'--color-surface': '#ffffff',
		'--color-surface-hover': '#f5f5f5',
		'--color-border': '#e5e5e5',
		'--color-text': '#171717',
		'--color-text-muted': '#737373'
	};

	const darkThemeVars: Record<string, string> = {
		'--color-bg': '#0a0a0a',
		'--color-surface': '#141414',
		'--color-surface-hover': '#1a1a1a',
		'--color-border': '#262626',
		'--color-text': '#fafafa',
		'--color-text-muted': '#a1a1aa'
	};

	function applyTheme(t: 'dark' | 'light') {
		const vars = t === 'light' ? lightThemeVars : darkThemeVars;
		const root = document.documentElement;
		for (const [key, value] of Object.entries(vars)) {
			root.style.setProperty(key, value);
		}
	}

	// Load from localStorage on mount
	onMount(() => {
		theme = (localStorage.getItem('focusvault-theme') as 'dark' | 'light') || 'dark';
		defaultTabLimit = parseInt(localStorage.getItem('focusvault-default-tab-limit') || '10', 10);
		const cap = localStorage.getItem('focusvault-capture-cap');
		captureCap = cap ? parseInt(cap, 10) : null;
	});

	// Persist and apply theme
	$effect(() => {
		localStorage.setItem('focusvault-theme', theme);
		applyTheme(theme);
	});

	// Persist default tab limit
	$effect(() => {
		localStorage.setItem('focusvault-default-tab-limit', String(defaultTabLimit));
	});

	// Persist capture cap
	$effect(() => {
		if (captureCap !== null) {
			localStorage.setItem('focusvault-capture-cap', String(captureCap));
		} else {
			localStorage.removeItem('focusvault-capture-cap');
		}
	});

	function handleExport() {
		alert('Export coming soon');
	}

	function handleClearData() {
		if (confirm('Are you sure you want to clear all local data? This cannot be undone.')) {
			localStorage.clear();
			theme = 'dark';
			defaultTabLimit = 10;
			captureCap = null;
		}
	}

	function handleCaptureCapInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		captureCap = value === '' ? null : parseInt(value, 10);
	}
</script>

<div class="page">
	<!-- Header -->
	<div class="header">
		<div>
			<h2 class="title">Settings</h2>
			<p class="subtitle">Configuration and preferences</p>
		</div>
	</div>

	<!-- Section 1: Appearance -->
	<section class="settings-section">
		<p class="section-label">Appearance</p>
		<div class="settings-card">
			<div class="field">
				<!-- svelte-ignore a11y_label_has_associated_control -->
				<label class="field-label">Theme</label>
				<div class="theme-toggle">
					<button
						class="toggle-btn"
						class:toggle-active={theme === 'dark'}
						onclick={() => (theme = 'dark')}
					>
						Dark
					</button>
					<button
						class="toggle-btn"
						class:toggle-active={theme === 'light'}
						onclick={() => (theme = 'light')}
					>
						Light
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Section 2: Defaults -->
	<section class="settings-section">
		<p class="section-label">Defaults</p>
		<div class="settings-card">
			<div class="field">
				<label class="field-label" for="default-tab-limit">Default Tab Limit</label>
				<input
					id="default-tab-limit"
					type="number"
					class="field-input"
					min="1"
					max="100"
					bind:value={defaultTabLimit}
				/>
			</div>
			<div class="field-divider"></div>
			<div class="field">
				<label class="field-label" for="capture-cap">Daily Vault Capture Cap</label>
				<input
					id="capture-cap"
					type="number"
					class="field-input"
					min="1"
					max="1000"
					placeholder="No limit"
					value={captureCap ?? ''}
					oninput={handleCaptureCapInput}
				/>
			</div>
		</div>
	</section>

	<!-- Section 3: Data -->
	<section class="settings-section">
		<p class="section-label">Data</p>
		<div class="settings-card">
			<div class="field field-row">
				<div>
					<p class="field-label">Export Data</p>
					<p class="field-hint">Download all your data as a file</p>
				</div>
				<button class="btn btn-secondary" onclick={handleExport}>Export</button>
			</div>
			<div class="field-divider"></div>
			<div class="field field-row">
				<div>
					<p class="field-label">Clear Local Data</p>
					<p class="field-hint">Remove all locally stored preferences and data</p>
				</div>
				<button class="btn btn-danger" onclick={handleClearData}>Clear</button>
			</div>
		</div>
	</section>

	<!-- Section 4: About -->
	<section class="settings-section">
		<p class="section-label">About</p>
		<div class="settings-card about-card">
			<p class="about-name">FocusVault v2</p>
			<p class="about-tagline">Local-first execution OS for high-output builders</p>
		</div>
	</section>
</div>

<style>
	.page {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* -- Header -- */
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

	/* -- Section -- */
	.settings-section {
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

	.settings-card {
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
		padding: 1rem 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	/* -- Fields -- */
	.field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.field-row {
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 500;
		margin: 0;
	}

	.field-hint {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		margin: 0.125rem 0 0;
	}

	.field-divider {
		height: 1px;
		background-color: var(--color-border);
		margin: 0;
	}

	.field-input {
		padding: 0.375rem 0.5rem;
		border-radius: 0.375rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-bg);
		color: var(--color-text);
		font-size: 0.875rem;
		width: 8rem;
		outline: none;
		font-family: inherit;
	}
	.field-input:focus {
		border-color: var(--color-accent);
	}
	.field-input::placeholder {
		color: var(--color-text-muted);
		opacity: 0.6;
	}

	/* -- Theme Toggle (pill selector) -- */
	.theme-toggle {
		display: inline-flex;
		gap: 0.25rem;
		padding: 0.25rem;
		border-radius: 0.5rem;
		background-color: var(--color-bg);
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.375rem 1rem;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--color-text-muted);
		background: none;
		border: none;
		cursor: pointer;
		transition: all 0.15s;
		font-family: inherit;
	}

	.toggle-btn:hover {
		color: var(--color-text);
	}

	.toggle-active {
		background-color: var(--color-accent);
		color: white;
	}

	.toggle-active:hover {
		color: white;
	}

	/* -- Buttons -- */
	.btn {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: opacity 0.15s;
		white-space: nowrap;
		flex-shrink: 0;
		font-family: inherit;
	}
	.btn:hover {
		opacity: 0.85;
	}

	.btn-secondary {
		background-color: var(--color-surface-hover);
		color: var(--color-text);
		border: 1px solid var(--color-border);
	}

	.btn-danger {
		background-color: rgba(239, 68, 68, 0.12);
		color: var(--color-danger);
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	/* -- About -- */
	.about-card {
		align-items: center;
		text-align: center;
		padding: 2rem 1.25rem;
	}

	.about-name {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.about-tagline {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.25rem 0 0;
	}

	/* -- Responsive -- */
	@media (max-width: 640px) {
		.field-row {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>

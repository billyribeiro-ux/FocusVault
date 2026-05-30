<script lang="ts">
	import type { CreateMissionInput } from '$lib/stores/mission.svelte';

	interface Props {
		open: boolean;
		onclose: () => void;
		oncreate: (input: CreateMissionInput) => void;
	}

	let { open, onclose, oncreate }: Props = $props();

	let name = $state('');
	let description = $state('');
	let tabLimit = $state(10);
	let deployEnabled = $state(false);
	let deployValue = $state(1);
	let outreachEnabled = $state(false);
	let outreachValue = $state(1);
	let iterateEnabled = $state(false);
	let iterateValue = $state(1);
	let submitting = $state(false);

	function resetForm() {
		name = '';
		description = '';
		tabLimit = 10;
		deployEnabled = false;
		deployValue = 1;
		outreachEnabled = false;
		outreachValue = 1;
		iterateEnabled = false;
		iterateValue = 1;
		submitting = false;
	}

	function handleClose() {
		resetForm();
		onclose();
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
	}

	function handleSubmit(e: Event) {
		e.preventDefault();
		if (!name.trim() || submitting) return;

		submitting = true;

		const input: CreateMissionInput = {
			name: name.trim(),
			description: description.trim() || undefined,
			tab_limit: tabLimit
		};

		const hasTargets = deployEnabled || outreachEnabled || iterateEnabled;
		if (hasTargets) {
			input.weekly_targets = {
				deploy: deployEnabled ? deployValue : 0,
				outreach: outreachEnabled ? outreachValue : 0,
				iterate: iterateEnabled ? iterateValue : 0
			};
		}

		oncreate(input);
		resetForm();
	}
</script>

{#if open}
	<div
		class="modal-backdrop"
		role="dialog"
		tabindex="-1"
		aria-modal="true"
		aria-labelledby="modal-title"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
	>
		<div class="modal-content">
			<div class="modal-header">
				<h3 id="modal-title" class="modal-title">New Mission</h3>
				<button
					class="modal-close"
					onclick={handleClose}
					aria-label="Close"
				>
					&times;
				</button>
			</div>

			<form onsubmit={handleSubmit}>
				<div class="form-body">
					<!-- Name -->
					<div class="field">
						<label for="mission-name" class="label">Name <span class="required">*</span></label>
						<input
							id="mission-name"
							type="text"
							class="input"
							placeholder="e.g. Launch MVP"
							bind:value={name}
							required
						/>
					</div>

					<!-- Description -->
					<div class="field">
						<label for="mission-desc" class="label">Description</label>
						<textarea
							id="mission-desc"
							class="input textarea"
							placeholder="What does success look like?"
							rows="3"
							bind:value={description}
						></textarea>
					</div>

					<!-- Tab Limit -->
					<div class="field">
						<label for="mission-tab-limit" class="label">Tab Limit</label>
						<input
							id="mission-tab-limit"
							type="number"
							class="input input-number"
							min="1"
							max="100"
							bind:value={tabLimit}
						/>
						<p class="hint">Max browser tabs allowed during this mission</p>
					</div>

					<!-- Weekly Targets -->
					<div class="field">
						<p class="label">Weekly Targets</p>
						<div class="targets">
							<div class="target-row">
								<label class="toggle-label">
									<input type="checkbox" bind:checked={deployEnabled} />
									<span>Deploy</span>
								</label>
								{#if deployEnabled}
									<input
										type="number"
										class="input input-small"
										min="0"
										bind:value={deployValue}
									/>
								{/if}
							</div>

							<div class="target-row">
								<label class="toggle-label">
									<input type="checkbox" bind:checked={outreachEnabled} />
									<span>Outreach</span>
								</label>
								{#if outreachEnabled}
									<input
										type="number"
										class="input input-small"
										min="0"
										bind:value={outreachValue}
									/>
								{/if}
							</div>

							<div class="target-row">
								<label class="toggle-label">
									<input type="checkbox" bind:checked={iterateEnabled} />
									<span>Iterate</span>
								</label>
								{#if iterateEnabled}
									<input
										type="number"
										class="input input-small"
										min="0"
										bind:value={iterateValue}
									/>
								{/if}
							</div>
						</div>
					</div>
				</div>

				<!-- Footer -->
				<div class="modal-footer">
					<button type="button" class="btn btn-secondary" onclick={handleClose}>
						Cancel
					</button>
					<button type="submit" class="btn btn-primary" disabled={!name.trim() || submitting}>
						{submitting ? 'Creating...' : 'Create Mission'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 100;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: rgba(0, 0, 0, 0.6);
		padding: 1rem;
	}

	.modal-content {
		width: 100%;
		max-width: 32rem;
		max-height: 90vh;
		overflow-y: auto;
		border-radius: 1rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-surface);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--color-border);
	}

	.modal-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.modal-close {
		background: none;
		border: none;
		color: var(--color-text-muted);
		font-size: 1.5rem;
		cursor: pointer;
		padding: 0;
		line-height: 1;
	}
	.modal-close:hover {
		color: var(--color-text);
	}

	.form-body {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.label {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--color-text-muted);
		margin: 0;
	}

	.required {
		color: var(--color-danger);
	}

	.hint {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		margin: 0;
	}

	.input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border-radius: 0.5rem;
		border: 1px solid var(--color-border);
		background-color: var(--color-bg);
		color: var(--color-text);
		font-size: 0.875rem;
		outline: none;
		transition: border-color 0.15s;
		font-family: inherit;
	}
	.input:focus {
		border-color: var(--color-accent);
	}
	.input::placeholder {
		color: var(--color-text-muted);
		opacity: 0.6;
	}

	.textarea {
		resize: vertical;
	}

	.input-number {
		max-width: 6rem;
	}

	.input-small {
		max-width: 5rem;
		padding: 0.375rem 0.5rem;
	}

	.targets {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.target-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.toggle-label input[type='checkbox'] {
		width: 1rem;
		height: 1rem;
		accent-color: var(--color-accent);
		cursor: pointer;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--color-border);
	}

	.btn {
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border: none;
		transition: background-color 0.15s, opacity 0.15s;
	}
	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		background-color: var(--color-accent);
		color: white;
	}
	.btn-primary:hover:not(:disabled) {
		background-color: var(--color-accent-hover);
	}

	.btn-secondary {
		background-color: transparent;
		color: var(--color-text-muted);
		border: 1px solid var(--color-border);
	}
	.btn-secondary:hover {
		background-color: var(--color-surface-hover);
	}
</style>

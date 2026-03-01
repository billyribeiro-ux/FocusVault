<script lang="ts">
	import type { CreateVaultItemInput, VaultItem } from '$lib/stores/vault.svelte';

	interface Props {
		open: boolean;
		onclose: () => void;
		oncreate: (input: CreateVaultItemInput) => void;
	}

	let { open, onclose, oncreate }: Props = $props();

	let itemType = $state<VaultItem['type']>('link');
	let url = $state('');
	let title = $state('');
	let why = $state('');
	let notes = $state('');
	let priority = $state<VaultItem['priority']>('med');
	let tagInput = $state('');
	let tags = $state<string[]>([]);
	let submitting = $state(false);

	const typeOptions: { value: VaultItem['type']; label: string; icon: string }[] = [
		{ value: 'link', label: 'Link', icon: '🔗' },
		{ value: 'note', label: 'Note', icon: '📝' },
		{ value: 'snippet', label: 'Snippet', icon: '< >' },
		{ value: 'file_ref', label: 'File', icon: '📁' }
	];

	const priorityOptions: { value: VaultItem['priority']; label: string }[] = [
		{ value: 'low', label: 'Low' },
		{ value: 'med', label: 'Med' },
		{ value: 'high', label: 'High' }
	];

	let whyCharCount = $derived(why.length);
	let whyWordCount = $derived(
		why
			.trim()
			.split(/\s+/)
			.filter((w) => w.length > 0).length
	);
	let whyValid = $derived(whyCharCount >= 12 && whyWordCount >= 3);

	function resetForm() {
		itemType = 'link';
		url = '';
		title = '';
		why = '';
		notes = '';
		priority = 'med';
		tagInput = '';
		tags = [];
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

	function addTag() {
		const tag = tagInput.trim().toLowerCase();
		if (tag && !tags.includes(tag)) {
			tags = [...tags, tag];
		}
		tagInput = '';
	}

	function handleTagKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ',') {
			e.preventDefault();
			addTag();
		} else if (e.key === 'Backspace' && tagInput === '' && tags.length > 0) {
			tags = tags.slice(0, -1);
		}
	}

	function removeTag(tag: string) {
		tags = tags.filter((t) => t !== tag);
	}

	async function handleSubmit() {
		if (!whyValid || submitting) return;

		submitting = true;
		try {
			const input: CreateVaultItemInput = {
				type: itemType,
				why,
				priority,
				source: 'manual'
			};
			if (itemType === 'link' && url.trim()) input.url = url.trim();
			if (title.trim()) input.title = title.trim();
			if (notes.trim()) input.notes = notes.trim();
			if (tags.length > 0) input.tags = [...tags];

			oncreate(input);
			resetForm();
		} catch {
			submitting = false;
		}
	}
</script>

{#if open}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<!-- svelte-ignore a11y_interactive_supports_focus -->
	<div
		class="modal-backdrop"
		role="dialog"
		aria-modal="true"
		aria-label="Create vault item"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
	>
		<div class="modal-panel">
			<!-- Header -->
			<div class="modal-header">
				<h2 class="modal-title">Add to Vault</h2>
				<button class="modal-close-btn" onclick={handleClose} aria-label="Close">
					&times;
				</button>
			</div>

			<!-- Body -->
			<form class="modal-body" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<!-- Type Selector -->
				<div class="form-group">
					<!-- svelte-ignore a11y_label_has_associated_control -->
					<label class="form-label">Type</label>
					<div class="type-selector">
						{#each typeOptions as opt}
							<button
								type="button"
								class="type-btn"
								class:type-btn-active={itemType === opt.value}
								onclick={() => (itemType = opt.value)}
							>
								<span class="type-icon">{opt.icon}</span>
								<span class="type-label">{opt.label}</span>
							</button>
						{/each}
					</div>
				</div>

				<!-- URL (shown for links) -->
				{#if itemType === 'link'}
					<div class="form-group">
						<label class="form-label" for="vault-url">URL</label>
						<input
							id="vault-url"
							type="url"
							class="form-input"
							placeholder="https://..."
							bind:value={url}
						/>
					</div>
				{/if}

				<!-- Title -->
				<div class="form-group">
					<label class="form-label" for="vault-title">Title</label>
					<input
						id="vault-title"
						type="text"
						class="form-input"
						placeholder="What is this item?"
						bind:value={title}
					/>
				</div>

				<!-- WHY (required) -->
				<div class="form-group">
					<label class="form-label" for="vault-why">
						Why <span class="required-star">*</span>
					</label>
					<textarea
						id="vault-why"
						class="form-input form-textarea"
						placeholder="Why are you saving this? (min 12 chars, 3 words)"
						rows={3}
						bind:value={why}
					></textarea>
					<div class="why-validation">
						<span class:valid={whyCharCount >= 12} class:invalid={whyCharCount > 0 && whyCharCount < 12}>
							{whyCharCount}/12 chars
						</span>
						<span class:valid={whyWordCount >= 3} class:invalid={whyWordCount > 0 && whyWordCount < 3}>
							{whyWordCount}/3 words
						</span>
					</div>
				</div>

				<!-- Notes -->
				<div class="form-group">
					<label class="form-label" for="vault-notes">Notes</label>
					<textarea
						id="vault-notes"
						class="form-input form-textarea"
						placeholder="Additional notes (optional)"
						rows={2}
						bind:value={notes}
					></textarea>
				</div>

				<!-- Priority -->
				<div class="form-group">
					<!-- svelte-ignore a11y_label_has_associated_control -->
					<label class="form-label">Priority</label>
					<div class="priority-selector">
						{#each priorityOptions as opt}
							<button
								type="button"
								class="priority-btn priority-{opt.value}"
								class:priority-btn-active={priority === opt.value}
								onclick={() => (priority = opt.value)}
							>
								{opt.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Tags -->
				<div class="form-group">
					<label class="form-label" for="vault-tags">Tags</label>
					<div class="tags-container">
						{#each tags as tag}
							<span class="tag-chip">
								{tag}
								<button type="button" class="tag-remove" onclick={() => removeTag(tag)}>
									&times;
								</button>
							</span>
						{/each}
						<input
							id="vault-tags"
							type="text"
							class="tag-input"
							placeholder={tags.length === 0 ? 'Add tags (press Enter)' : ''}
							bind:value={tagInput}
							onkeydown={handleTagKeydown}
							onblur={addTag}
						/>
					</div>
				</div>

				<!-- Actions -->
				<div class="modal-actions">
					<button type="button" class="btn btn-cancel" onclick={handleClose}>
						Cancel
					</button>
					<button
						type="submit"
						class="btn btn-submit"
						disabled={!whyValid || submitting}
					>
						{submitting ? 'Saving...' : 'Save to Vault'}
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
		align-items: flex-end;
		justify-content: center;
		background-color: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		padding: 0;
	}

	@media (min-width: 640px) {
		.modal-backdrop {
			align-items: center;
			padding: 1rem;
		}
	}

	.modal-panel {
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 1rem 1rem 0 0;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
		animation: slide-up 0.2s ease-out;
	}

	@media (min-width: 640px) {
		.modal-panel {
			border-radius: 1rem;
			max-width: 32rem;
			animation: fade-scale-in 0.2s ease-out;
		}
	}

	@keyframes slide-up {
		from {
			transform: translateY(100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	@keyframes fade-scale-in {
		from {
			transform: scale(0.95);
			opacity: 0;
		}
		to {
			transform: scale(1);
			opacity: 1;
		}
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid var(--color-border);
	}

	.modal-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--color-text);
	}

	.modal-close-btn {
		background: none;
		border: none;
		color: var(--color-text-muted);
		font-size: 1.5rem;
		cursor: pointer;
		padding: 0.25rem;
		line-height: 1;
		border-radius: 0.375rem;
		transition: color 0.15s;
	}

	.modal-close-btn:hover {
		color: var(--color-text);
	}

	.modal-body {
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.form-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.required-star {
		color: var(--color-danger);
	}

	.form-input {
		background-color: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
		color: var(--color-text);
		outline: none;
		transition: border-color 0.15s;
		width: 100%;
		box-sizing: border-box;
		font-family: inherit;
	}

	.form-input::placeholder {
		color: var(--color-text-muted);
		opacity: 0.6;
	}

	.form-input:focus {
		border-color: var(--color-accent);
	}

	.form-textarea {
		resize: vertical;
		min-height: 3rem;
	}

	/* Type selector */
	.type-selector {
		display: flex;
		gap: 0.5rem;
	}

	.type-btn {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
		padding: 0.625rem 0.5rem;
		background-color: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
		font-family: inherit;
	}

	.type-btn:hover {
		border-color: var(--color-accent);
		color: var(--color-text);
	}

	.type-btn-active {
		border-color: var(--color-accent);
		background-color: rgba(59, 130, 246, 0.1);
		color: var(--color-accent);
	}

	.type-icon {
		font-size: 1.125rem;
	}

	.type-label {
		font-size: 0.75rem;
		font-weight: 500;
	}

	/* WHY validation */
	.why-validation {
		display: flex;
		gap: 0.75rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.valid {
		color: var(--color-success);
	}

	.invalid {
		color: var(--color-danger);
	}

	/* Priority */
	.priority-selector {
		display: flex;
		gap: 0.5rem;
	}

	.priority-btn {
		flex: 1;
		padding: 0.375rem 0.75rem;
		background-color: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		color: var(--color-text-muted);
		font-family: inherit;
	}

	.priority-btn:hover {
		border-color: var(--color-text-muted);
	}

	.priority-btn-active.priority-low {
		border-color: var(--color-success);
		color: var(--color-success);
		background-color: rgba(34, 197, 94, 0.1);
	}

	.priority-btn-active.priority-med {
		border-color: var(--color-warning);
		color: var(--color-warning);
		background-color: rgba(245, 158, 11, 0.1);
	}

	.priority-btn-active.priority-high {
		border-color: var(--color-danger);
		color: var(--color-danger);
		background-color: rgba(239, 68, 68, 0.1);
	}

	/* Tags */
	.tags-container {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
		background-color: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		padding: 0.375rem 0.5rem;
		min-height: 2.375rem;
		align-items: center;
		transition: border-color 0.15s;
	}

	.tags-container:focus-within {
		border-color: var(--color-accent);
	}

	.tag-chip {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		background-color: rgba(59, 130, 246, 0.15);
		color: var(--color-accent);
		font-size: 0.75rem;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		white-space: nowrap;
	}

	.tag-remove {
		background: none;
		border: none;
		color: var(--color-accent);
		cursor: pointer;
		font-size: 0.875rem;
		line-height: 1;
		padding: 0;
		opacity: 0.7;
	}

	.tag-remove:hover {
		opacity: 1;
	}

	.tag-input {
		background: none;
		border: none;
		outline: none;
		color: var(--color-text);
		font-size: 0.875rem;
		flex: 1;
		min-width: 6rem;
		padding: 0.125rem 0;
		font-family: inherit;
	}

	.tag-input::placeholder {
		color: var(--color-text-muted);
		opacity: 0.6;
	}

	/* Actions */
	.modal-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
		padding-top: 0.5rem;
		border-top: 1px solid var(--color-border);
		margin-top: 0.25rem;
	}

	.btn {
		padding: 0.5rem 1.25rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		border: 1px solid transparent;
		font-family: inherit;
	}

	.btn-cancel {
		background-color: transparent;
		border-color: var(--color-border);
		color: var(--color-text-muted);
	}

	.btn-cancel:hover {
		border-color: var(--color-text-muted);
		color: var(--color-text);
	}

	.btn-submit {
		background-color: var(--color-accent);
		color: white;
		border: none;
	}

	.btn-submit:hover:not(:disabled) {
		background-color: var(--color-accent-hover);
	}

	.btn-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>

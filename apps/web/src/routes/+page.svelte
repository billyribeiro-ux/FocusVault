<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	onMount(() => {
		goto('/today');
	});
</script>

<p style="color: var(--color-text-muted)">Redirecting...</p>

<script lang="ts">
	import '../app.css';

	interface Props {
		children: import('svelte').Snippet;
	}

	let { children }: Props = $props();

	const navItems = [
		{ href: '/today', label: 'Today', icon: '◎' },
		{ href: '/vault', label: 'Vault', icon: '◆' },
		{ href: '/mission', label: 'Mission', icon: '▶' },
		{ href: '/learning', label: 'Learning', icon: '◈' },
		{ href: '/stats', label: 'Stats', icon: '▤' },
		{ href: '/settings', label: 'Settings', icon: '⚙' }
	];
</script>

<div class="flex h-screen">
	<!-- Sidebar -->
	<nav
		class="hidden w-56 flex-col border-r md:flex"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		<div class="p-4">
			<h1 class="text-lg font-bold tracking-tight" style="color: var(--color-accent)">
				FocusVault
			</h1>
			<p class="text-xs" style="color: var(--color-text-muted)">Execution OS</p>
		</div>

		<div class="flex flex-1 flex-col gap-1 px-2">
			{#each navItems as item}
				<a
					href={item.href}
					class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors"
					style="color: var(--color-text-muted)"
				>
					<span class="text-base">{item.icon}</span>
					{item.label}
				</a>
			{/each}
		</div>
	</nav>

	<!-- Mobile bottom nav -->
	<nav
		class="fixed bottom-0 left-0 right-0 z-50 flex border-t md:hidden"
		style="background-color: var(--color-surface); border-color: var(--color-border)"
	>
		{#each navItems as item}
			<a
				href={item.href}
				class="flex flex-1 flex-col items-center gap-0.5 py-2 text-xs"
				style="color: var(--color-text-muted)"
			>
				<span class="text-lg">{item.icon}</span>
				{item.label}
			</a>
		{/each}
	</nav>

	<!-- Main content -->
	<main class="flex-1 overflow-y-auto pb-16 md:pb-0">
		<div class="mx-auto max-w-5xl p-4 md:p-6 lg:p-8">
			{@render children()}
		</div>
	</main>
</div>

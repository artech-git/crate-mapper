<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>CrateMapper</title>
	<meta name="description" content="Visual explorer for the Rust crate ecosystem" />
</svelte:head>

{@render children()}

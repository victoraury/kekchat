<script>
	import { onMount } from 'svelte';
	import Chat from './components/Chat.svelte';
	import LogIn from './components/Login.svelte';
	import { HOST } from './const';

	import { isLoggedIn } from './stores.js';
	import { startWebsocket } from './websocket';
	
	onMount(() => {
		startWebsocket(HOST);
	});

</script>

{#if $isLoggedIn}
	<Chat/>
{:else}
	<LogIn/>
{/if}
<script>
	import { onMount } from 'svelte';
	import Chat from './components/Chat.svelte';
	import LogIn from './components/Login.svelte';
	import HOST from '../../util/ip.json';

	import { isLoggedIn } from './stores.js';
	import { startWebsocket } from './websocket';
	
	onMount(() => {
		startWebsocket(`ws://${HOST.ip}:8021`);
	});

</script>

{#if $isLoggedIn}
	<Chat/>
{:else}
	<LogIn/>
{/if}
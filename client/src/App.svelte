<script lang="ts">
	import { onMount } from 'svelte';
	import Username from './components/Username.svelte';
	import Chat from './components/Chat.svelte'
	
	let socket = new WebSocket("ws://localhost");

	socket.onopen = function(this: WebSocket, ev: Event) {
		console.log("[WebSocket]: Connected!");
		socket.send("Yo!");
	}

	socket.onmessage = (ev: MessageEvent) => {
		console.log("Received message", ev.data);
	}

	socket.onclose = (ev: CloseEvent) => {
		console.log("[WebSocket]: Connection closed");
	}

	socket.onerror = (ev: Event) => {
		console.log(ev);
	}

</script>

<main>
	<!-- <Username/> -->
	<Chat/>
</main>
<footer>
	
</footer>


<style>
	main {
		display: grid;
		/* flex-direction: column; */
		width: 100%;
		min-height: 100vh;
		place-items: center;
	}
</style>
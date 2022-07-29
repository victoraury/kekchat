<script>
    import ChatMessage from "./ChatMessage.svelte";
    import { messages, users } from '../stores';
    import { afterUpdate } from 'svelte';
    import { MESSAGE_COLORS } from '../const';

    let chatElement;
    afterUpdate(() => {
		if($messages) scrollToBottom(chatElement);
    });
    const scrollToBottom = async (node) => {
        node.scroll({ top: node.scrollHeight, behavior: 'smooth' });
    }

    $: if($messages && chatElement) {
		scrollToBottom(chatElement);
	}
</script>

<div class="chat-history" bind:this={chatElement}>
    <ul class="m-b-0">
        {#each $messages as msg}
            <ChatMessage {msg} color={MESSAGE_COLORS[$users.indexOf(msg.from) % MESSAGE_COLORS.length]}/>
        {/each}                      
    </ul>
</div>

<style>
.chat-history ul {
    padding: 0
}

.chat-history {
    overflow: auto;
    padding: 20px;
    border-bottom: 1px solid var(--kekw-orange);
    height: 400px;
    max-height:400px;
}

@media (min-width: 576px) {
    .chat-history {
        height: 350px;
    }
}

@media (min-width: 768px) {
    .chat-history {
        height: 400px;
    }
 }


@media (min-width: 992px) { 
    .chat-history {
        height: 450px;
    }
}


@media (min-width: 1200px) {
    .chat-history {
        height: 500px;
    } 
}



</style>
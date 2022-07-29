<script>
    export let msg, color;
</script>

<li class="clearfix">
    {#if msg.hasOwnProperty('from')}
        <div class="chat-sender"  style="color: {color} !important">{msg.from}</div>
    {/if}
    <div class="message {msg.hasOwnProperty('from') ? 'my-message' : 'other-message'}" style={msg.hasOwnProperty('from') ? "color: {color} !important" : ''}> {msg.message} </div>
</li>

<style>
.chat-sender {
    color: var(--kekw-orange);
}
li {
    list-style: none;
    margin-bottom: 16px
}
.clearfix:after {
    visibility: hidden;
    display: block;
    font-size: 0;
    content: " ";
    clear: both;
    height: 0
}

.message {
    color: yellow;
    padding: 8px 16px;
    font-size: 16px;
    border-radius: 7px;
    display: inline-block;
    position: relative;
    word-wrap: anywhere;
}

.message:before {
    content: " ";
    border: solid transparent;
    top: 16px;
    right: -10px;
    height: 0;
    width: 0;
    position: absolute;
    pointer-events: none;
    border-top: 5px solid transparent;
    border-bottom: 5px solid transparent;
    border-left:5px solid gold; 
}

.message {
    background: transparent;
}

.my-message {
    color:white !important;
}

.other-message {
    color: gold !important;
    float: right
}

.my-message:before {
    top: 16px;
    left: -10px;
    border: solid transparent;
    content: " ";
    height: 0;
    width: 0;
    position: absolute;
    pointer-events: none;
    border-top: 5px solid transparent;
    border-bottom: 5px solid transparent;
    border-right:5px solid white; 
}


.other-message:after {
    border-bottom-color: #e8f1f3;
    left: 93%
}

</style>
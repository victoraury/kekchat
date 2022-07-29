<script>
    import ChatHistory from "./ChatHistory.svelte";
    import KekLogo from './KekLogo.svelte';
    import { MESSAGE_COLORS } from '../const';
    import { messages, users } from '../stores';
    import { socket, sendMessage } from '../websocket';

    var usersFilter = '';
    function handleMessageKeyUp(key) {
        if (key.code == 'Enter' && key.target !== null) {

            if (socket.readyState === socket.OPEN) {
                sendMessage({
                    message: key.target.value
                });

                $messages = [...$messages, {
                    message: key.target.value
                }];

                key.target.value = '';
            }
        }
    }
</script>

<link href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css" rel="stylesheet" />

<div class="container">
    <div class="row clearfix">
        <div class="col-lg-12">
            <div class="card chat-app">
                <div id="plist" class="people-list">
                    <div class="input-group">
                        <div class="input-group-prepend">
                            <span class="input-group-text kek-icon-bg"><i class="fa fa-search kek-icon"></i></span>
                        </div>
                        <input type="text" class="form-control kek-input" placeholder="Search user list" bind:value={usersFilter}>
                    </div>
                    <ul class="list-unstyled chat-list mt-2 mb-0">
                        {#each $users.filter(user => usersFilter === '' || user.includes(usersFilter) ) as user, i}
                            <li class="clearfix mb-1 user-list-user">
                                <div class="about">
                                    <div class="name" style="color: {MESSAGE_COLORS[i % MESSAGE_COLORS.length]}">{user}</div>
                                </div>
                            </li>
                        {/each}
                    </ul>
                </div>

                <div class="chat">
                    <div class="chat-header clearfix">
                        <div class="row">
                            <div class="col-lg-12 kek-col">
                                <KekLogo/>
                            </div>
                        </div>
                    </div>
                    <ChatHistory/>
                    <div class="chat-message clearfix">
                        <div class="input-group mb-0">
                            <div class="input-group-prepend">
                                <span class="input-group-text kek-icon-bg"><i class="fa fa-send kek-icon"></i></span>
                            </div>
                            <input type="text" class="form-control kek-input" placeholder="Enter yours keks here." on:keyup={handleMessageKeyUp}>                                    
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>

.card {
    background: #000;
    color:#fff;
    transition: .5s;
    border: 0;
    margin-bottom: 30px;
    border-radius: .55rem;
    position: relative;
    width: 100%;
    box-shadow: 0 1px 2px 0 rgb(0 0 0 / 10%);
    border: 1px solid var(--kekw-orange);
}
.chat-app .people-list {
    width: 280px;
    position: absolute;
    left: 0;
    top: 0;
    padding: 20px;
    z-index: 7
}

.chat-app .chat {
    margin-left: 280px;
    border-left: 1px solid var(--kekw-orange)
}

.people-list {
    -moz-transition: .5s;
    -o-transition: .5s;
    -webkit-transition: .5s;
    transition: .5s
}

.chat-list {
    overflow: auto;
    height: 500px;
}

.people-list .chat-list li {
    padding: 10px 15px;
    list-style: none;
    border-radius: 3px
}

.people-list .chat-list li:hover {
    background: rgba(166, 166, 166, 0.09);
    border-radius: 8px;
}

.people-list .chat-list li .name {
    font-size: 15px
}

.chat .chat-header {
    border-bottom: 1px solid var(--kekw-orange)
}


.chat .chat-history ul li {
    list-style: none;
    margin-bottom: 16px
}

.chat .chat-history ul li:last-child {
    margin-bottom: 0px
}

.chat .chat-message {
    padding: 20px
}

.clearfix:after {
    visibility: hidden;
    display: block;
    font-size: 0;
    content: " ";
    clear: both;
    height: 0
}

@media only screen and (max-width: 767px) {
    .chat-app .people-list {
        height: 465px;
        width: 100%;
        overflow-x: auto;
        background: #fff;
        left: -400px;
        display: none
    }
    .chat-app .chat {
        margin: 0
    }
    .chat-app .chat .chat-header {
        border-radius: 0.55rem 0.55rem 0 0
    }
}

@media only screen and (min-width: 768px) and (max-width: 992px) {
    .chat-app .chat-list {
        height: 650px;
        overflow-x: auto
    }
}

@media only screen and (min-device-width: 768px) and (max-device-width: 1024px) and (orientation: landscape) and (-webkit-min-device-pixel-ratio: 1) {
    .chat-app .chat-list {
        height: 480px;
        overflow-x: auto
    }
}

.kek-input {
    background-color: rgba(39, 33, 33, 0.8);
    border-color: var(--kekw-orange);
}

.kek-icon { 
    color: var(--kekw-orange);
}

.kek-icon-bg {
    background-color: rgba(39, 33, 33, 0.8);
    border: 1px solid var(--kekw-orange);
}

.kek-col {
    padding-left: 2px;
}
</style>
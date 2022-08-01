<script>
    import { sendMessage } from '../websocket';
    import { username } from '../stores';
    import KekLogo from './KekLogo.svelte';
    import { loginAttemptStatus } from '../stores';
    import { onMount, onDestroy } from 'svelte';

    const audio = new Audio('/assets/audio/kekw_welcome.mp3');
    onMount(() => {
        audio.play();
    })

    onDestroy(() => {
        audio.pause()
        audio.currentTime = 0;
    });

    function sendUsernameMessage() {
        if ($username !== '') {
            
        }

        $loginAttemptStatus = 'ongoing';
        sendMessage({ username: $username });
    }
</script>

<svelte:head>
	<title>Login - KekChat</title>
</svelte:head>

<div class="container align-items-center d-flex h-100">
    <div class="row justify-content-center">
        <div class="col-12">
            <KekLogo/>
        </div>
        <div class="col-6 mt-5">
            <div class="input-group mb-0">
                <div class="input-group-prepend">
                    <span class="input-group-text kek-icon-bg"><i class="fa fa-user kek-icon"></i></span>
                </div>
                <input type="text" maxlength="16" class="form-control kek-input" bind:value={$username} placeholder="Enter your username." on:keyup={
                    (ev) => { 
                        if (ev.code !== 'Enter') { 
                            $loginAttemptStatus = 'done'; 
                            return;
                        }

                        sendUsernameMessage();
                    } 
                }/>                                    
                <input type="button" class="kek-button" value="Enter" on:click={sendUsernameMessage}/>
            </div>
            <span></span>
        </div>
        <div class="col-12 mt-2 text-center">
            <span class="badge {$loginAttemptStatus === 'error' ? 'badge-danger' : 'badge-success'}">
                {$loginAttemptStatus === 'error' ? 'Username already in use. Please, choose another one.' : 'Choose an username to login.'}
            </span>
        </div>
    </div>
</div>

<style>
.kek-input {
    background-color: rgba(39, 33, 33, 0.8);
    border-color: var(--kekw-orange);
}

.kek-icon { 
    color: var(--kekw-orange);
}

.kek-button {
    color:white;
    border:none;
    border-radius: 4px;
    margin-left: 3px;
    background-color: var(--kekw-orange);
    padding: 0 15px 0 15px;
}

.kek-button:hover {
    background-color: var(--kekw-orange-hover);
}
</style>
import { users, messages, isLoggedIn, loginAttemptStatus} from './stores';

export var socket;

export function startWebsocket(ip) {
    socket = new WebSocket(ip);

    socket.onopen = function(ev) {
		console.log("[WebSocket]: Connected!");
	}

	socket.onmessage = (ev) => {
		console.log("Received message", ev.data);
        let payload = JSON.parse(ev.data);

        // 1 new message
        // 2 set username response
        // 3 new user
        // 4 user disc
        // 5 user list
        switch (payload.op) {
            case 1:
                messages.update(messages => [...messages, payload]);
                break;
            case 2:
                if (payload.ok) {
                    isLoggedIn.set(true);
                    loginAttemptStatus.set('good');
                }
                else {
                    loginAttemptStatus.set('error');
                }
                break;
            case 3:
                users.update( users => [...users, payload.user]);
                break;
            case 4:
                users.update( users => users.filter(k => k != payload.user));
                break;
            case 5:
                users.update( users => payload.users);
            default:
                break;
        }
	}

	socket.onclose = (ev) => {
		console.log("[WebSocket]: Connection closed");
        isLoggedIn.set(false);
	}

	socket.onerror = (ev) => {
		console.log(ev);
	}
}

export function sendMessage(payload) {
    socket.send(JSON.stringify(payload));
}

use std::{
    collections::HashMap,
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
    fs,
    env
};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

enum Ops {
    NewMessage = 1,
    SetUsernameResponse = 2,
    UserConnected = 3,
    UserDisconnected = 4,
    UserList = 5
}

#[derive(Deserialize)]
struct HostConfig {
    ip: String
}

#[derive(Deserialize)]
struct SetUsernameMessage {
    username: String
}
#[derive(Serialize)]
struct SetUsernameResponse {
    op: u8,
    ok: bool
}
#[derive(Deserialize)]
struct ContentMessage {
    message: String
}
#[derive(Serialize)]
struct ContentMessageWithName {
    op: u8,
    message: String,
    from: String
}
#[derive(Serialize)]
struct Userlist {
    op: u8,
    users: Vec<String>
}

#[derive(Serialize)]
struct UserDisCo {
    op: u8,
    user: String
}

type WsConn = tokio_tungstenite::WebSocketStream<TcpStream>;
type ConnTable = Arc<Mutex<HashMap< SocketAddr, (UnboundedSender<Message>, bool) >>>;
type UserSet = Arc<Mutex<HashSet<String>>>;

fn send_to_all(peer_table: ConnTable, message: Message, own_address: &SocketAddr) {
    let table = peer_table.lock().unwrap();

    for (recipient, active) in table.iter().filter( |(peer_addr, _)| **peer_addr != *own_address ).map( |(_, sender)| sender ) {
        if *active {
            recipient.unbounded_send(message.clone()).unwrap();
        }
    }
}

fn send_to_self(peer_table: ConnTable, message: Message, own_address: &SocketAddr) {
    let peer_lock = peer_table.lock().unwrap();
    
    let (recipient, _) = peer_lock.get(own_address).unwrap();

    recipient.unbounded_send(message).unwrap();
}

async fn handle_connection(address: std::net::SocketAddr, connection: WsConn, peer_table: ConnTable, user_set: UserSet) {

    // cria um canal para comunicação com esta thread
    let (tx, rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = unbounded();

    let mut username = "".to_string();
    let (outgoing, incoming) = connection.split();
    
    // insere o canal desta thread na tabela
    peer_table.lock().unwrap().insert(address, (tx, false));
    
    // declara um listener que espera por mensagens e as propaga para as outras threads
    let incoming_messages = incoming.try_for_each( |msg| {
        
        // se a mesnsagem não for texto, ignora
        if !msg.is_text() {
            return future::ok(());
        }
        
        let payload = msg.clone().into_text().unwrap();

        // se o usuário ainda não tiver escolhido um username
        // seta o username (se tudo ocorrer bem)
        if username.is_empty() {
            let set_user: Result<SetUsernameMessage> = serde_json::from_str(&payload);

            // se a mensagem estiver com formato incorreto, ignora
            let user = match set_user {
                Ok(user_message) => user_message,
                Err(_) => return future::ok(())
            };

            let mut users = user_set.lock().unwrap();

            // se a mensagem estiver formatada corretamente, mas o username já estiver em uso
            // notifica para o usuário escolher outro nome
            if users.contains(&user.username) {
                send_to_self(peer_table.clone(), Message::text(
                    serde_json::to_string(
                        &SetUsernameResponse{
                            op: (Ops::SetUsernameResponse as u8),
                            ok: false
                    }).unwrap()),
                    &address
                );
                return future::ok(());
            }

            // registra na tabela o username deste usuário
            username = user.username;
            users.insert(username.clone());
            
            {
                let mut peer_lock = peer_table.lock().unwrap();
                let (_, verified) = peer_lock.get_mut(&address).unwrap();
                *verified = true;
            }

            // envia pro cliente uma mensagem sinalizando que o set de username foi OK
            send_to_self(peer_table.clone(), Message::text(
                serde_json::to_string(
                    &SetUsernameResponse{
                        op: (Ops::SetUsernameResponse as u8),
                        ok: true
                    }
                ).unwrap()),
                &address
            );

            // envia pro cliente a lista de usuários conectados
            send_to_self(peer_table.clone(), Message::text(
                serde_json::to_string(
                    &Userlist{
                        op: (Ops::UserList as u8),
                        users: (&users).iter().filter(|usr| **usr != *username).cloned().collect()
                    }
                ).unwrap()),
                &address
            );
            
            // envia o username do cliente para os outros users
            send_to_all(
                peer_table.clone(),
                Message::text(
                    serde_json::to_string(
                        &UserDisCo {
                            op: (Ops::UserConnected as u8),
                            user: username.clone()
                        }
                    )
                    .unwrap()
                ),
                &address
            );
        }
        else {
            // verifica se o formato da mensagem está certo antes de propagar para os outros users
            let check_content: Result<ContentMessage> = serde_json::from_str(&payload);
    
            // se não estiver no formato, ignora a mensagem
            if check_content.is_err() {
                return future::ok(());
            }
            
            // propaga a mensagem recebida para os outros clientes
            send_to_all(peer_table.clone(),
                Message::from(
                    serde_json::to_string(&{
                        &ContentMessageWithName{
                            op: (Ops::NewMessage as u8),
                            from: username.clone(),
                            message: check_content.unwrap().message
                        }
                    }).unwrap()
                ),
                &address
            );
        }
        future::ok(())
    });

    // declara um listener que aguarda por mensagens enviadas no canal desta thread e as envia para o usuário
    let channel_listener = rx.map(Ok).forward(outgoing);

    // alterna entre receber mensagens do usuário e receber mensagens das outras threads
    pin_mut!(incoming_messages, channel_listener);
    future::select(incoming_messages, channel_listener).await;

    // quando a conexão for fechada, remove o canal da tabela e nome do usuário
    // também notifica os outros usuários da desconexão
    peer_table.lock().unwrap().remove(&address);

    if !username.is_empty() {

        user_set.lock().unwrap().remove(&*username);

        send_to_all(peer_table.clone(), Message::from(
            serde_json::to_string(
                &UserDisCo {
                    op: (Ops::UserDisconnected as u8),
                    user: username.clone()
                }
            ).unwrap()),
            &address
        );
    }

    println!("Connection closed! = {}", address);
}

fn get_local_ip() -> std::string::String {
    let dir = env::current_dir().unwrap();

    let ip_file = fs::read_to_string(format!("{}{}", dir.display(), "/util/ip.json"))
                        .expect("couldnt read file");
                        
    let host_config : Result<HostConfig> = serde_json::from_str(&ip_file);

    host_config.unwrap().ip
}

#[tokio::main]
async fn main() -> Result<()>{
    // endereço para aguardar novas conexões
    let local_ip = get_local_ip();
    let host = format!("{}:8021", local_ip);
    
    // cria um listener TCP na porta especificada
    let listener = TcpListener::bind(host.clone()).await.expect("Failed to start server") ;
    println!("Server listening on {}", host.clone());

    // define a tabela de streams websocket dos usuários
    let conn_map: ConnTable = Arc::new(Mutex::new(HashMap::new()));

    // conjunto de nomes dos usuários do chat
    let user_set: UserSet = Arc::new(Mutex::new(HashSet::new()));

    loop {

        // estabelecimento da stream tcp
        let (stream, addr) = listener.accept()
            .await
            .expect("Failed to establish TCP connection")
        ;

        // handshake websocket através da stream estabelecida
        let handshake = tokio_tungstenite::accept_async(stream).await;

        match handshake {
            Err(_) => {
                println!("Failed to stablish WebSocket handshake with {}", addr)
            },
            Ok(connection) => {

                // spawna uma thread para gerenciar a conexão websocket
                tokio::spawn(handle_connection(addr, connection, conn_map.clone(), user_set.clone()));

                println!("Established websocket connection @ {:}", addr);
            }
        }
    }
}

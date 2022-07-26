
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;



type ConnTable = Arc<Mutex<HashMap< SocketAddr, UnboundedSender<Message> >>>;
type WsConn = tokio_tungstenite::WebSocketStream<TcpStream>;

async fn handle_connection(_conn: WsConn, _conn_table: ConnTable) {
    println!("spawned thread");
}

#[tokio::main]
async fn main() -> Result<(), IoError >{

    // endereço para aguardar novas conexões
    let HOST = "127.0.0.1:8080";
    
    // cria um listener TCP na porta especificada
    let listener = TcpListener::bind(HOST).await.expect("Failed to start server") ;
    println!("Server listening on {}", HOST);

    // define a tabela de streams websocket dos usuários
    let conn_map: ConnTable = Arc::new(Mutex::new(HashMap::new()));

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

                // cria um canal para comunicação para a thread responsável por gerencial a conexão do cliente
                let (tx, rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = unbounded();
                
                // insere a conexão na tabela
                conn_map.lock().unwrap().insert(addr, tx);

                // cria uma referencia da tabela para a thread
                let peer_table = conn_map.clone();
                
                // spawna uma thread para gerenciar a conexão websocket
                std::thread::spawn(move || {
                    let (outgoing, incoming) = connection.split();

                    let incoming_message = incoming.try_for_each( |msg| {

                        let _: Vec<_> = peer_table
                            .lock()
                            .unwrap()
                            .iter()
                            .filter( |(peer_addr, _)| **peer_addr != addr)
                            .map( |(_, sender)| {
                                sender.unbounded_send(msg.clone()).unwrap();
                            })
                            .collect()
                        ;
                    
                        future::ok(())
                    });
                });

                println!("{:?}", conn_map);
                println!("Established websocket connection @ {:}", addr);
            }
        }
    }
}

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (sender, _receiver) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let sender = sender.clone();
        let mut receiver = sender.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        sender.send((line.clone(), addr)).unwrap();
                        line.clear()
                    }

                    result = receiver.recv() => {
                        let (msg, incoming_addr) = result.unwrap();
                        // don't sent message to yourself
                        if addr != incoming_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

type Db = Arc<RwLock<HashMap<String, Vec<u8>>>>;

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut dbs = db.write().unwrap();
                // The value is stored as `Vec<u8>`
                dbs.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let dbs = db.read().unwrap();

                if let Some(value) = dbs.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(RwLock::new(HashMap::new()));

    let v = Arc::new(vec![1, 2, 5]);

    task::spawn(async move {
        println!("{:?}", v);
    });

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

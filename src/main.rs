use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame, client, Result, Command};

#[tokio::main]
async fn main() -> Result<()> {
    // let handle = tokio::spawn(async {
    //     "hello_world".to_string()
    // });
    // let result = handle.await?;
    // println!("{:?}", result);

    let v = vec![1, 2, 3, 4, 5];

    let handle = tokio::spawn(async {
        println!("Here's a vec: {:?}", v);
    });

    let result = handle.await?;
    println!("{:?}", result);
    Ok(())
    // let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    //
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //
    //     tokio::spawn(async move {
    //         process(socket).await;
    //     });
    // }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    let mut db = HashMap::new();

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };
        // Respond with an error
        // let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
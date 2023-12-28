use mini_redis::{Connection, Frame, client, Result, Command};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("foo.txt").await?;
    let res = file.write(b"Some thing").await?;
    println!("{:?}", res);

    let mut buffer = Vec::new();
    let res2 = file.read_to_end(&mut buffer).await?;
    println!("{:?}", res2);
    Ok(())
}
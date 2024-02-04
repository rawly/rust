extern crate transmission_rpc;

use dotenv::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse, Torrents, Torrent};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // env_logger::init();
    let url = env::var("TURL")?;
    let mut client;
    if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        client = TransClient::with_auth(url.parse()?, BasicAuth { user, password });
    } else {
        client = TransClient::new(url.parse()?);
    }
    // let response: Result<RpcResponse<SessionStats>> = client.session_stats().await;
    // match response {
    //     Ok(_) => println!("Yay!"),
    //     Err(e) => panic!("{}", e),
    // }
    // println!("Rpc response is ok: {}", response?.arguments.active_torrent_count);

    let res: Result<RpcResponse<Torrents<Torrent>>> = client.torrent_get(None, None).await;
    match res {
        Ok(response) => {
          println!("{:#?}", response);
          for r in response.arguments.torrents {            
            println!("Rpc response is ok: {:#?}", r);      
          }
        },
        Err(e) => panic!("{}", e),
    }   
    Ok(())
}
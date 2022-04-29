use std::{io::ErrorKind};

use crossbeam::channel::{TryRecvError, Receiver, Sender};
use serde::{Serialize, Deserialize};
use tokio::{net::TcpStream, io::{self, AsyncReadExt, AsyncWriteExt}};

const WS_ADDRESS: &str = "wss://ws.lightstream.bitflyer.com/json-rpc";

#[derive(Serialize, Deserialize)]
struct Channel {
    channel: String
}

#[derive(Serialize, Deserialize)]
struct JsonRPCRequest {
    id: u64,
    jsonrpc: &'static str,
    method: &'static str,
    params: Channel
}

struct JsonRpcResponse {
    id: u64,
    jsonrpc: String,
    method: &'static str,
    params: Vec<String>
}


pub struct OrderBook {
    pub rx_result: Receiver<Vec<u8>>,
}

impl OrderBook {
    pub async fn connect(product_code: &[&str]) -> io::Result<Self> {
        let snapshot = "lightning_board_snapshot_";
        let lightning_board = "lightning_board_";
        let lightning_execution = "lightning_executions_";
        
        let mut stream = TcpStream::connect(WS_ADDRESS).await?;
        let mut id = 0;
        for pc in product_code.iter() {
            for i in [snapshot, lightning_board, lightning_execution] {
                id += 1;
                let channel = format!("{}_{}", i, pc);
                let req = JsonRPCRequest {
                    jsonrpc: "2.0",
                    method: "subscribe",
                    params: Channel { channel },
                    id
                };
                let src = serde_json::to_string(&req).unwrap();
                stream.write_all(src.as_bytes()).await?;
            }
        }

        let (tx, rx) = crossbeam::channel::unbounded();
        tokio::task::spawn(Self::event_loop(stream, tx));
        Ok(Self{
            rx_result: rx
        })
    }
    
    
    async fn event_loop(stream: TcpStream, tx: Sender<Vec<u8>>) {
        let mut buf = vec![];
        loop {
            if stream.readable().await.is_err() {
                continue;
            }
            match stream.try_read(&mut buf) {
                Ok(i) => {
                    tx.send(buf);
                    buf = Vec::with_capacity(i);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
                Err(e) => eprintln!("{:?}", e)
            }
        }
    }
}

pub enum APISendError {
    TxSendError,
    RecvDisconnected,
    IO(io::Error)
}
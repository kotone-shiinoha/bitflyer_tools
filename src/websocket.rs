use std::{io::ErrorKind};

use crossbeam::channel::{TryRecvError, Receiver, Sender};
use tokio::{net::TcpStream, io::{self, AsyncReadExt}};

const WS_ADDRESS: &str = "wss://ws.lightstream.bitflyer.com/json-rpc";


pub struct BitflyerRealtimeAPI
{
    tx_send: Sender<Box<[u8]>>,
    rx_send_result: Receiver<io::Result<()>>,
}

impl BitflyerRealtimeAPI {
    pub async fn connect(message_handler: fn(&[u8]) -> ()) -> io::Result<Self> {
        let stream = TcpStream::connect(WS_ADDRESS).await?;
        let (tx_send, rx_send) = crossbeam::channel::unbounded();
        let (tx_send_result, rx_send_result) = crossbeam::channel::unbounded();
        
        let _handle = tokio::task::spawn(Self::event_loop(stream, rx_send.clone(), tx_send_result.clone()));
        Ok(Self {
            tx_send,
            rx_send_result,
        })
    }

    pub async fn send(&self, bytes: Box<[u8]>) -> Result<(), APISendError> {
        if let Err(_) = self.tx_send.send(bytes) {
            return Err(APISendError::TxSendError)
        }

        let rx = self.rx_send_result.clone();
        loop {
            match rx.try_recv() {
                Err(TryRecvError::Empty) => continue,
                Err(TryRecvError::Disconnected) => return Err(APISendError::RecvDisconnected),
                Ok(a) => {
                    return if let Err(e) = a {
                        Err(APISendError::IO(e))
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }
    
    async fn event_loop(stream: TcpStream, rx: Receiver<Box<[u8]>>, tx_send_result: Sender<io::Result<()>>, message_handler: fn(&[u8]) -> ()) {
        let task2 = async move {
            loop {
                match rx.try_recv() {
                    Err(TryRecvError::Disconnected) => return,
                    Err(TryRecvError::Empty) => continue,
                    Ok(item) => {
                        if let Err(e) = stream.writable().await {
                            println!("{}", e);
                            continue;
                        }
                        match stream.try_write(&*item) {
                            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                                continue;
                            }
                            Ok(_) => {
                                let r = tx_send_result.send(Ok(()));
                                println!("{:?}", r);
                                return;
                            }
                            Err(e) => {
                                let r = tx_send_result.send(Err(e));
                                println!("{:?}", r);
                                return;
                            }
                        }
                    }
                }
            }
        };

        tokio::task::spawn(task2);
    }
}

pub enum APISendError {
    TxSendError,
    RecvDisconnected,
    IO(io::Error)
}
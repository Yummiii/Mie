use arguments::{Comandos, Opts};
use clap::Parser;
use estruturas::Tecla;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::mpsc::{self, Receiver}, task};

use crate::serial::enviar_dados;

mod arguments;
mod estruturas;
mod input;
mod serial;

#[tokio::main]
async fn main() -> io::Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        Comandos::Servidor(serv) => iniciar_servidor(serv.porta, serv.porta_serial).await,
        Comandos::Cliente(cliente) => iniciar_cliente(cliente.ip_servidor).await,
    }
}

lazy_static::lazy_static! {
    static ref CHAVE: Vec<u8> = base64::decode("M3Q2dzl6JEMmRilKQE5jUmZValhuWnI0dTd4IUElRCo=").unwrap();
}

async fn iniciar_servidor(porta: i32, porta_serial: String) -> io::Result<()> {
    println!("Iniciando o servidor na porta: {}", porta);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", porta)).await?;
    let serial_port = serialport::new(porta_serial, 9600).open().unwrap();

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) =>  {
                println!("Conexão de: {:?}", addr);
                task::spawn(async move {
                    loop {
                        let mut buf = [0; 1024];
                        let n = match socket.read(&mut buf).await {
                            Ok(n) if n == 0 => return,
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("{:?}", e);
                                return;
                            }
                        };

                        let bc =
                            bincode_aes::with_key(bincode_aes::create_key(CHAVE.clone()).unwrap());
                        let mut buff = buf[0..n].to_vec();
                        let decoded = bc
                            .deserialize::<Tecla>(&mut buff)
                            .expect("Algum macaco fez merda");
                        //println!("{:#?}", decoded);
                        //let a = serial_port.
                        enviar_dados(decoded, serial_port.as_ref().clone());
                    }
                });
            }
            Err(e) => println!("Erro: {:?}", e),
        };
    }
}

#[cfg(not(target_family = "windows"))]
async fn iniciar_cliente(ip_servidor: String) -> io::Result<()> {
    use input::ler_input;
    let stream = TcpStream::connect(ip_servidor).await?;
    let (tx, rx) = mpsc::channel(10);

    ler_input(tx);
    enviar_key(rx, stream).await;

    Ok(())
}

#[cfg(target_family = "windows")]
async fn iniciar_cliente(ip_servidor: String) -> io::Result<()> {
    println!("Sai do windows colega");
    Ok(())
}

async fn enviar_key(mut rx: Receiver<(bool, u8, bool)>, mut stream: TcpStream) {
    while let Some((pressed,  key, special)) = rx.recv().await {
        let bc = bincode_aes::with_key(bincode_aes::create_key(CHAVE.clone()).unwrap());
        let payload = bc.serialize(&Tecla { key, pressed, special }).unwrap();
        stream.write_all(&payload).await.unwrap();
    }
}

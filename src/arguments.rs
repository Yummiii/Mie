use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Comandos
}

#[derive(Parser, Debug)]
pub enum Comandos {
    #[clap(about = "Inicia um servidor")]
    Servidor(Servidor),
    #[clap(about = "Inicia um cliente")]
    Cliente(Cliente)
}

#[derive(Parser, Debug)]
pub struct Servidor {
    #[clap(short, long, about = "Porta que o servidor ira iniciar (default: 5000)", default_value = "5000")]
    pub porta: i32,
    #[clap(short, long, about = "Porta serial que esta o arduino")]
    pub porta_serial: String
}

#[derive(Parser, Debug)]
pub struct Cliente {
    #[clap(short, long, about = "Ip do servidor (ex: 192.168.1.100:5000)")]
    pub ip_servidor: String
}
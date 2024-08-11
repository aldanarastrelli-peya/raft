use actix::{Actor, Addr, Context, StreamHandler};
use tokio::io::{AsyncBufReadExt, split, WriteHalf, BufReader};
use tokio::net::TcpStream;
use tokio::task;
extern crate actix;
use tokio_stream::wrappers::LinesStream;

/// Raft RPCs
pub enum RPC {
    RequestVotes,
    AppendEntries
}
pub struct HealthConnection {
    write_half: WriteHalf<TcpStream>,
    id: Option<usize>
}
impl Actor for HealthConnection {
    type Context = Context<Self>;
}
impl StreamHandler<Result<String, std::io::Error>> for HealthConnection {
    fn handle(&mut self, read: Result<String, std::io::Error>, ctx: &mut Self::Context) {
        if let Ok(line) = read {
            println!("Hello {}",line);
        } else {
            println!(" Failed to read line");
        }
    }
}
impl HealthConnection {
    pub fn create_actor(stream: TcpStream, id: usize) -> Addr<HealthConnection> {
        HealthConnection::create(|ctx| {
            let (read_half, write_half) = split(stream);
            HealthConnection::add_stream(LinesStream::new(BufReader::new(read_half).lines()), ctx);
            HealthConnection {
                write_half,
                id: Some(id)
            }
        })
    }
}
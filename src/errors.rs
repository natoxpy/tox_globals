use std::num::ParseIntError;

#[derive(Debug)]
pub enum Errors {
    IO(std::io::Error),
    ParseInt(ParseIntError),

    // Tokio Error
    SendError(String),


    // Tick errors
    // 0: expected tick
    // 1: received tick
    UnexpectedTick(usize, usize),

    /// When received is empty or does not contain a `Some`
    RecvError,

    // when received unexpected tick
    TickError(String)
}

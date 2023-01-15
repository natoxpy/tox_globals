/*
@docs
The TICK sequence is a communication protocol between a server and a client. 
It begins when the server sends a `TICK_START` message to the client, 
and the client responds with a `TICK_CLIENT_READY` message. 
The TICK sequence then continues with the exchange of other messages in a specific order. 
If all TICK messages are exchanged correctly, 
the server sends a `TICK_END` message to indicate that the sequence was executed successfully. 
The TICK sequence then repeats until the client disconnects.
*/
pub mod tick_values {
    // Info: Message to start the TICK Communication sequence
    pub static TICK_START: usize = 1;
    // Info: Message indicating the end of the TICK Communication sequence
    pub static TICK_END: usize = 2;

    // SECTION: SERVER

    // Info: Message from server indicating that it has finished reading data
    pub static TICK_SERVER_ENDED_READING_DATA: usize = 5;
    // Info: Message from server indicating that it has no data to send
    pub static TICK_SERVER_NO_DATA_TO_SEND: usize = 6;
    // Info: Message from server indicating that it is listening for client data
    pub static TICK_SERVER_LISTENING: usize = 7;

    // SECTION: CLIENT
    
    // Info: Message from client indicating that it has no data to send
    pub static TICK_CLIENT_NO_DATA_TO_SEND: usize = 10;
    // Info: Message from client indicating that it is listening for server data
    pub static TICK_CLIENT_LISTENING: usize = 11;
    // Info: Message from client acknowledging receipt of `TICK_START` message
    pub static TICK_CLIENT_READY: usize = 12;
    // Info: Message from client indicating that it has finished reading data
    pub static TICK_CLIENT_ENDED_READING_DATA: usize = 13;
}

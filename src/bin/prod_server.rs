use rust_poker::tcp::server_messenger::ServerMessenger;

fn main() {
    let messenger = ServerMessenger::new("127.0.0.1:7878");
    loop {}

}

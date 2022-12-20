use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //let req = stream.req.remote();
        //let remote = listener.req.remote();
        println!("H2S!");
    }
}

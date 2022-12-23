use std::net::TcpListener;
use std::thread;

fn main() {
    thread::spawn(|| {
        let listener445 = TcpListener::bind("0.0.0.0:445").unwrap();
        for stream in listener445.incoming() {
            let stream = stream.unwrap();
            println!("CO!");
        }
    });
    thread::spawn(|| {
        let listener80 = TcpListener::bind("0.0.0.0:80").unwrap();
        for stream in listener80.incoming() {
            let stream = stream.unwrap();
            //let req = stream.req.remote();
            //let remote = listener.req.remote();
            println!("H2S!");
        }
    });
    println!("Started everything we could.");
    let listener443 = TcpListener::bind("0.0.0.0:443").unwrap();
    for stream in listener443.incoming() {
        let stream = stream.unwrap();
        //let req = stream.req.remote();
        //let remote = listener.req.remote();
        println!("CH4!");
    }
}

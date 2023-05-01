use std::net::TcpListener;
use std::thread;

fn main() {
    thread::spawn(|| {
        let listener445 = TcpListener::bind("0.0.0.0:445").unwrap();
        for stream in listener445.incoming() {
            let stream = stream.unwrap();
            println!(alert(445));
    // TODO: call a processAlert method that rather than println here
        }
    });
    thread::spawn(|| {
        let listener80 = TcpListener::bind("0.0.0.0:80").unwrap();
        for stream in listener80.incoming() {
            let stream = stream.unwrap();
            //let req = stream.req.remote();
            //let remote = listener.req.remote();
            println!(alert(80));
    // TODO: call a processAlert method that rather than println here
        }
    });
    thread::spawn(|| {
    let listener443 = TcpListener::bind("0.0.0.0:443").unwrap();
        for stream in listener443.incoming() {
            let stream = stream.unwrap();
            //let req = stream.req.remote();
            //let remote = listener.req.remote();
            println!(alert(443));
    // TODO: call a processAlert method that rather than println here
        }
    });
    println!("Started everything we could.");
}

fn alert(n: u32) {
    let message = String::from("Yikes! We don't know what just hit us!");
    match n{
        80=>message = "H2S!",
        443=>message = "CH4!",
        445=>message = "CO!",
        _=>println!("Uh oh, we need to update our match list!"),
    }
    message
    // Rather than returning here, we could make our alert here. Or have a separate processing method
}

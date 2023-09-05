use std::net::TcpListener;


fn main() {
    // creating a local host address and port to create final Endpoint

    const HOST : &str ="127.0.0.1";
    const PORT : &str="8477";
    let end_point : String = HOST.to_owned()+ ":" + PORT;
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Webserver is listening on Port {}",PORT);
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
    }

    println!("Hello, world!");
}

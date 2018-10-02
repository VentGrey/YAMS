use std::net::TcpListener;

fn main() {
    let escucha = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in escucha.incoming() {
        let stream = stream.unwrap();

        println!("¡Conexión establecida!");
    }
}

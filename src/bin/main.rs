/*---------LICENSE---------
 * Este trabajo está bajo la
 * licencia GPL-3+ por lo que
 * su copia, distribución y
 * modificación son aceptables
 * y requeridas.
 *
 * Autor: Omar Jair Purata Funes
 * ----------EOF-----------*/


//Crates propias (porque soy gil y tuve que hacer una)
extern crate server;
use hola::ThreadPool;


//Bibliotecas normales, favor de no mover.
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

//Multihilo (Esta cosa es del diablo, no la toque si no sabe lo que hace)
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Apagando el servidor...");
    println!("x.x");
}



fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hola.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hola.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

        let contents = fs::read_to_string(filename).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}

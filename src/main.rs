/*---------LICENSE---------
 * Este trabajo está bajo la
 * licencia GPL-3+ por lo que
 * su copia, distribución y
 * modificación son aceptables
 * y requeridas.
 *
 * Autor: Omar Jair Purata Funes
 * ----------EOF-----------*/

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let escucha = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in escucha.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {

    let contents = fs::read_to_string("hola.html").unwrap();

    let respuesta = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(respuesta.as_bytes()).unwrap();
    stream.flush().unwrap();
    } else {
        //otras cosas por ahora
    }
}

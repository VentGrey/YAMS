/*---------LICENSE---------
 * Este trabajo está bajo la
 * licencia GPL-3+ por lo que
 * su copia, distribución y
 * modificación son aceptables
 * y requeridas.
 *
 * Autor: Omar Jair Purata Funes
 * ----------EOF-----------*/

use std::net::TcpListener;

fn main() {
    let escucha = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in escucha.incoming() {
        let stream = stream.unwrap();

        println!("¡Conexión establecida!");
    }
}

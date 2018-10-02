/*---------LICENSE---------
 * Este trabajo está bajo la
 * licencia GPL-3+ por lo que
 * su copia, distribución y
 * modificación son aceptables
 * y requeridas.
 *
 * Autor: Omar Jair Purata Funes
 * ----------EOF-----------*/

/* Para empezar nuestro servidor web necesitará "escuchar" una conexión TCP
 * Nosotros vamos a trabajar en dicha parte, pues la biblioteca std de Rust
 * ofrece un módulo std::net que nos permite hacer ésto.*/

use std::net::TcpListener;


/* La función main escuchará en la dirección 127.0.0.1:7878 por algún stream
 * de TCP. Cuando encuentre uno ésta enviará a pantalla el mensaje
 * "Conexión establecida" */
fn main() {
    let escucha = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in escucha.incoming() {
        let stream = stream.unwrap();

        println!("¡Conexión establecida!");
    }
}

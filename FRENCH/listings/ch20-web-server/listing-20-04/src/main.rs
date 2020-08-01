use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();

        gestion_connexion(flux);
    }
}

fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0; 1024];

    flux.read(&mut tampon).unwrap();

    let reponse = "HTTP/1.1 200 OK\r\n\r\n";

    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
}

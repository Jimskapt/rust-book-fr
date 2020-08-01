use std::fs;
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

// ANCHOR: here
// -- partie masquée ici--

fn gestion_connexion(mut flux: TcpStream) {
    // -- partie masquée ici--

    // ANCHOR_END: here
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // ANCHOR: here
    let (ligne_statut, nom_fichier) = if tampon.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contenu = fs::read_to_string(nom_fichier).unwrap();

    let reponse = format!("{}{}", ligne_statut, contenu);

    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
}
// ANCHOR_END: here

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// ANCHOR: here
use std::thread;
use std::time::Duration;
// -- partie masquée ici--
// ANCHOR_END: here

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();

        gestion_connexion(flux);
    }
}
// ANCHOR: here

fn gestion_connexion(mut flux: TcpStream) {
    // -- partie masquée ici--

    // ANCHOR_END: here
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

    // ANCHOR: here
    let get = b"GET / HTTP/1.1\r\n";
    let pause = b"GET /pause HTTP/1.1\r\n";

    let (ligne_statut, nom_fichier) = if tampon.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if tampon.starts_with(pause) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // -- partie masquée ici--
    // ANCHOR_END: here

    let contenu = fs::read_to_string(nom_fichier).unwrap();

    let reponse = format!("{}{}", ligne_statut, contenu);

    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
    // ANCHOR: here
}
// ANCHOR_END: here

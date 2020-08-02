use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// ANCHOR: here
fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();

        thread::spawn(|| {
            gestion_connexion(flux);
        });
    }
}
// ANCHOR_END: here

fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

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

    let contenu = fs::read_to_string(nom_fichier).unwrap();

    let reponse = format!("{}{}", ligne_statut, contenu);

    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
}

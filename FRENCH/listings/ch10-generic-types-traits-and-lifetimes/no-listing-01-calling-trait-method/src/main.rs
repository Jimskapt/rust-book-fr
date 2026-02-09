use agregateur::{Resumable, Tweet};

fn main() {
    let publication = PublicationSociale {
        nom_utilisateur: String::from("jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        republication: false,
    };
    
    println!("1 nouvelle publication : {}", publication.resumer());
}

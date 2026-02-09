use agregateur::{self, Resumable, PublicationSociale};

fn main() {
    // ANCHOR: here
    let publication = PublicationSociale {
        nom_utilisateur: String::from("jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        retweet: false,
    };
    
    println!("1 nouvelle publication : {}", publication.resumer());
    // ANCHOR_END: here
}

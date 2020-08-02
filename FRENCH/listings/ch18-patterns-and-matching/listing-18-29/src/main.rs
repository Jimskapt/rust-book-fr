fn main() {
    // ANCHOR: here
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Nous avons trouvé un id dans l'intervalle : {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Nous avons trouvé un id dans un autre intervalle")
        }
        Message::Hello { id } => println!("Nous avons trouvé un autre id : {}", id),
    }
    // ANCHOR_END: here
}

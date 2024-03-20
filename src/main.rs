use std::io;

fn main() {
    println!("Salut boyz");

    println!("Entrez le premier nombre : ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");
    let _number: i32 = input
        .trim()
        .parse()
        .expect("Erreur lors de la conversion du nombre");
}

// println!("Entrez le premier nombre : ");

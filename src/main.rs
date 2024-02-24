use std::io;
fn main() {
    'main: loop {
        let mut name: String = String::new();
        let mut trickster: String = String::new();

        println!("What will your characters name be? :");
        
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line.");

        println!("What will the tricksters name be? :");

        io::stdin()
            .read_line(&mut trickster)
            .expect("Failed to read line.");

        println!("This a story about {}", name);
        println!("{}, went in the forsest looking for a stick", name);
        println!("{} talked to a {} expecting it to be a trick", name, trickster);
        println!("Turns out it was and now {} has got ticks", name);

        'choice: loop {
            println!("Do you want to play again?");
            let mut choice: String = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Error");

            let choice: &str = &choice.trim();

            match choice {
                "exit" => break 'main,
                "replay" => break 'choice,
                _ => continue,
            }
        }
    }
}
mod fish;

use std::io::Write;

use fish::BrailleFish;

fn main() {
    // why [u8; 4]? Because we had a local meme in the rust community about ::<[u8; 4]>() turbofish
    let fish = BrailleFish::new(40, "[u8; 4]".to_string());
    println!("{}", fish.field);
    // now iterate over the fish and print it, but replacing the line in the console with \r
    for item in fish.take(400) {
        print!("\r{}", item);
        std::io::stdout().flush().unwrap();  // to not wait for \n
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

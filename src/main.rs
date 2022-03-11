use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("CompsGen {}",secret_number);
    loop {
        println!("Hey feed me");
        let mut feed = String::new();
        io::stdin()
            .read_line(&mut feed)
            .expect("feed me valid");
        let feed:u32 = feed.trim().parse().expect("ops");
        println!("Your's feed {}",feed);

        match feed.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!");break;},
        }
    }
}
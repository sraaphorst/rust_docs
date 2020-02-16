use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;
use std::process::exit;


fn main() {
    // Generate the secret number.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I just won me a whole mess of loot from liberating Johnny Silver's neck from his head!");
    println!("I'm feeling unnaturally generous today. If ye can guess what the scoundrel had, I'll");
    println!("cut you in and give you a share.");

    loop {
        print!("What do you think that swarthy bandit had: => ");
        io::stdout().flush().expect("Ya just died of scurvy.");

        // Notice that we can just redeclare the type of guess.
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Ya failed like a lubeless landlubber.");

        // Quitters never win.
        if guess.trim().eq( "quit") {
            println!("Ya be a quitter? You're head'll quit yet body as well, then!");
            exit(0);
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You got into the rum early today, I see.");
                continue;
            },
        };

        println!("Yer guess be: {}.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small. You underestimate me, matey. I should send you to be with the fishes."),
            Ordering::Greater => println!("\"Too big!!!\" is what the lassie cried, ya greedy bastard."),
            Ordering::Equal => {
                println!("Ya nasty rapscallion! Ya guessed how many pieces o' eight I had! A deal's a deal.");
                println!("{} for me, one for you...", secret_number - 1);
                break;
            },
        }
    }
}

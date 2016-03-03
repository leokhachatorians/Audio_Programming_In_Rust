use std::io::{self, BufRead};

fn main()
{
	let mut note = 0;
	let scale: [&'static str; 12] = ["C", "Db", "D", "Eb" ,"E", "F",
                             "Gb", "G", "Ab", "A", "Bb", "B"];

    println!("Please enter a key!");
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let key = match lines.next() {
        Some(Ok(a)) => a,
        _ => panic!("Could read input.")
        };

    for i in 0..12 {
        if scale[i] == key{
            note = i;
            println!("== {} major scale == ", key);
            break;
        }
        else {
            note = 13;
        }
    }

    if note <= 12 {
        for i in 0..7 {
            println!("{}", scale[note%12]);
            if i != 2{
                note += 2;
            }
            else{
                note += 1;
            }
        }
    }
    else{
        println!("Invalid Key: {}",key);
    }
}
use std::io;

fn main() {
    let table: [&'static str; 12] = ["C", "C#", "D", "D#",
                                     "E", "F", "F#", "G",
                                     "G#", "A", "A#", "B"];
    let base = get_base_note();
    let interval = get_interval(); 
    let mut p1 = 0;
    let p2 = 11;

    while table[p1].to_string() != base.to_string() {
        p1 += 1;

        if p1 > p2 {
            println!("Couldn't find note: {}", base);
            return
        }
    }

    p1 += mod12(interval);

    if p1 > p2 {
        p1 -= p2;
    }

    println!("{0} transposed by {1} semitones is {2}", base, interval,table[p1]);
}

fn get_interval() -> i32 {
    println!("Enter interval in semitones: ");
    let mut interval = String::new();
    io::stdin().read_line(&mut interval)
        .expect("Error Reading Line");

    let interval: i32 = match interval.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Error, not an integer: {}", interval); get_interval() } 
    };

    interval
}

fn get_base_note() -> String {
    println!("Enter base note (capitals, use # short sharps): ");
    let mut base = String::new();
    io::stdin().read_line(&mut base)
        .expect("Error Reading Line");
    base.trim().into()    
}

fn mod12(interval: i32) -> usize {
    let mut value = interval;
    while value < 0
    {
        value +=  12;
    }
    while value >= 12 
    {
        value -= 12;
    }
    value as usize
}

use std::io;


fn main() {
    let semitone_ratio = 2.0_f64.powf(1.0/12.0);
    
    let c5 = 220.0 * semitone_ratio.powf(3.0);
    let c0 = c5 * 0.5_f64.powf(5.0);

    let midinote = get_midinote();
    if midinote > 127.0 {
        println!("{} is above the MIDI range, try again.", midinote);
    }
    else {
        let frequency = c0 * semitone_ratio.powf(midinote);
        println!("MIDI Note {0} has frequency {1}", midinote, frequency);
    }
}

fn get_midinote() -> f64 {
    println!("Enter a midi note: ");
    let mut midinote = String::new();
    io::stdin().read_line(&mut midinote)
        .expect("Error Reading Line");
    
    let midinote: f64 = match midinote.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Error, not a number: {0}", midinote); get_midinote() }
    };

    midinote
}

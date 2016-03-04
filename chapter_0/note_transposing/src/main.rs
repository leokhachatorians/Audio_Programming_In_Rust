use std::io::{self, BufRead};

fn main() {
    let table: ['&static str; 12] = ["C", "C#", "D", "D#",
                                     "E", "F", "F#", "G",
                                     "G#", "A", "A#", "B"];
    let mut interval = 0;
}

int mod12(interval: i32)
{
    while interval < 0
    {
        interval += 12;
    }
    while interval >= 12 
    {
        interval -= 12;
    }
    interval
}

extern crate getopts;
use getopts::Options;
use std::env;

macro_rules! err_msg  {
    ($err_msg:expr) => {{
        println!("Error: {}", $err_msg);
        return;
    }};
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [option]",program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    let mut intervals: [f32; 25] = [0.0;25];

    opts.optopt("f", "", "set output file name", "NAME");
    opts.optflag("m", "midi", "sets format of start value of MIDI note");
    opts.optflag("i", "interval", "prints the calculated interval was well as abs freq");
    opts.optflag("h", "help", "display the help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(a) => { a } 
        Err(e) => panic!(e.to_string())
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    if matches.free.len() != 2 {
        print_usage(&program, opts);
        err_msg!("Needs two arguments to run.");
    }

    let display_amount = match matches.free[0].parse::<f32>() {
        Ok(num) => num,
        Err(_) => err_msg!("Error: N out of range. Must be between 1 and 24")
    };
    
    let start_value = match matches.free[1].parse::<f32>() {
        Ok(num) => num,
        Err(_) => err_msg!("Error: Start Value must a number")
    };

    let also_intervals = 
        if matches.opt_present("i") {
            true
        } else {
            false
        };

    let is_midi = 
        if matches.opt_present("m") {
            true
        } else {
            false
        };

    if display_amount < 1.0 || display_amount > 24.0 {
        err_msg!("N out of range. Must be between 1 and 24");
    }

    if is_midi {
        if start_value > 127.0 || start_value < 0.0 {
            err_msg!("MIDI must be between 0.0 and 127.0");
        }
    } else {
        if start_value <= 0.0 {
            err_msg!("Frequency value must be positive");
        }
    }

    let mut base_freq = 
            if is_midi {
                let ratio = 2.0_f32.powf(0.083333_f32);
                let c5 = 220.0 * ratio.powf(3.0);
                let c0 = c5 * 0.5_f32.powf(5.0);
                c0 * ratio.powf(start_value as f32)
            } else { 
                start_value as f32
            };

    let ratio = 2.0_f32.powf(1.0/display_amount);
    for i in 0..display_amount as usize {
        intervals[i] = base_freq;
        base_freq *= ratio;
    }

    for i in 0..display_amount as usize {
        if also_intervals {
            println!("{}: {} \t {}", i, ratio.powi(i as i32), intervals[i]);
        } else {
            println!("{}: {:?}", i,  intervals[i]);
        }
    }
}

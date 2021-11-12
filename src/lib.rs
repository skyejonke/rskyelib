use chrono::{Local};
use std::env;
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }


// This will print a log, with the current time, of whatever string you give it. This is only true if it uses the -v or --verbose argument.
pub fn log (inpt :&str) {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) || args.contains(&"--verbose".to_string()) {
        let time = Local::now();
        println!("{}:\t{}", time.format("%X"), inpt);
    }
}

pub fn elog (inpt :&str) {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) || args.contains(&"--verbose".to_string()) {
        let time = Local::now();
        eprintln!("{}:\t{}", time.format("%X"), inpt);
    }
}

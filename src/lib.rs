use structopt::StructOpt;
use chrono::{Local};
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    verbose :bool,
}
// This will print a log, with the current time, of whatever string you give it. This is only true if it uses the -v or --verbose argument.
pub fn log (inpt :&str) {
    let opt = Opt::from_args();
    if opt.verbose {
        let time = Local::now();
        println!("{}:\t{}", time.format("%X"), inpt);
    }
}

pub fn elog (inpt :&str) {
    let opt = Opt::from_args();
    if opt.verbose {
        let time = Local::now();
        eprintln!("{}:\t{}", time.format("%X"), inpt);
    }
}

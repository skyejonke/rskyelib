use structopt::StructOpt;
use chrono::{Local};
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }


#[derive(Debug, StructOpt)] #[structopt(name = "Cleanup", about = "Cleans up a directory.")]
struct Opt {
    /// Print lots of information?
    #[structopt(short, long)]
    verbose: bool,

    /// Input Path
    #[structopt(short = "i", long = "input", default_value = "/home/skye/Downloads/")]
    inpt_path: String,

    /// Output Path
    #[structopt(
        short = "o",
        long = "output",
        default_value = "/home/skye/Documents/DownloadArchives/"
    )]
    out_path: String,

    /// Timeframe (in days)
    #[structopt(short = "t", long = "timeframe", default_value = "7")]
    time_frame: u32,
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

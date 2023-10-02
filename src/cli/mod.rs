pub mod args;

use {
    crate::*,
    args::Args,
    clap::Parser,
    cli_log::*,
    std::path::PathBuf,
};

// static MISSING_DEFAULT_MESSAGE: &str = "\
// No nginx log found at default location, do you have nginx set up?
// If necessary, provide the path to the log file(s) as argument.
// More information with 'rhit --help'.";

fn print_analysis(paths: &[PathBuf], args: &args::Args) -> Result<(), NgStatError> {
    let stat = time!("Calculate stat", wallker::read_path(&paths[0]));
    // let mut log_base = time!("LogBase::new", LogBase::new(paths, args))?;
    // let printer = md::Printer::new(args, &log_base);
    // let base = &mut log_base;
    // let trend_computer = time!("Trend computer initialization", TrendComputer::new(base, args))?;
    // md::summary::print_summary(base, &printer);
    // time!("Analysis & Printing", md::print_analysis(&log_base, &printer, trend_computer.as_ref()));
    println!("Count .ts files: {}", stat.count_ts);
    println!("Count CLASS: {}", stat.count_class);
    println!("Count ATTRS: {}", stat.count_attr);
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

pub fn run() -> Result<(), NgStatError> {
    let args = Args::parse();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    // if args.help {
    //     help::print();
    //     return Ok(());
    // }
    let paths = args.files.clone();
    if paths.is_empty() {
        eprintln!("Target is req arg");
        return Ok(());
    }
    let result = print_analysis(&paths, &args);
    // let result = match args.output {
    //     Output::Raw => print_raw_lines(&paths, &args),
    //     Output::Tables => print_analysis(&paths, &args),
    //     Output::Csv => print_csv_lines(&paths, &args),
    //     Output::Json => print_json_lines(&paths, &args),
    // };
    if let Err(NgStatError::FileNotFound(ref path)) = result {
        // if path == &PathBuf::from(DEFAULT_NGINX_LOCATION) {
        // eprintln!("CATCH: File path is not found");
        // }
    }
    log_mem(Level::Info);
    // result
    Ok(())
}
use clap_verbosity_flag::VerbosityFilter;

pub fn debug_log(verbosity: &VerbosityFilter, message: &str) {
    if *verbosity == VerbosityFilter::Debug {
        println!("{}", message);
    }
}

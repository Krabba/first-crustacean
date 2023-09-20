mod xlog;

fn main() {
    println!("Hello, world!");

    xlog::log_x();

    std::process::exit(0);
}
mod xlog;

fn main() {
    println!("Hello, world!");

    xlog::log_x(55);

    std::process::exit(0);
}
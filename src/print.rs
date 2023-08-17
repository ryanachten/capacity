use colored::Colorize;

pub fn subheading(str: &str) {
    println!("{}", str.italic())
}

pub fn warning(str: &str) {
    println!("{}", str.yellow())
}

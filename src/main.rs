macro_rules! customPrint {
    () => {
        println!("Hello, World!")
    };

    ($dest:expr) => {
        println!("Hello, {}!", $dest);
    };

    ($act:expr => $dest:expr) => {
        println!("{} for {}!", $act, $dest);
    };
}

fn main() {
    customPrint!()
}

// semi-colon at line 7 and 11 will cause an error

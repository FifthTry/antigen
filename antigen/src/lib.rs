mod main;

pub use main::main;


#[cfg(test)]
mod tests {

    #[test]
    fn fbt() {
        if fbt_lib::main().is_some() {
            panic!("test failed")
        }
    }
}

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}

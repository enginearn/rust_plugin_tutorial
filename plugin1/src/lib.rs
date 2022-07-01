#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

extern crate app;

#[no_mangle]
pub fn run() {
    println!("Running plugin1!");
    app::test_app_func("Hello World! from plugin1!");
}
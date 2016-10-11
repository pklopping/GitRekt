// DO NOT RUN THIS IT WILL CRASH YOUR COMPUTER
#[allow(unconditional_recursion)]
fn main() {
    std::thread::spawn(main);
    main();
}
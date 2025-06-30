mod cpu;
mod io;
mod gui;

fn addd(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let a = addd(3, 4);
    println!("{}", a);
}

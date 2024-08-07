use std::io;
fn main() {
    let x=9 as i64;
    let y= 10 as i16;
    
    let z= x+(y as i64) ; 
    println!("{}",z);
    start();
}
fn start() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail");
    let no: i64=input.trim().parse().unwrap();
    println!("{}", no + 2);
}
 
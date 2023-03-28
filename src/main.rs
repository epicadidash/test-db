use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
pub trait Create{
    
} 
enum  Tdata{
    Table(&'static str)
}
enum Int {
    BigInteger(i64),
    NInteger(i32),
    SmallInteger(i16)
}
enum Float {
    BigDecimal(f64),
    NDecimal(f32)
}
enum Tye<T>{
Type(T)
}
enum Type{
Integer(Int),
Decimal(Float),
Boolean(bool),
String(&'static str),
}
fn print_prompt(){
    let mut lock = stdout().lock();
    write!(lock, "db>").unwrap();
    io::stdout().flush();
}
fn inputin(s:&mut String){
    io::stdin().read_line(s).expect("failed to readline");
}

fn chekforcommands(hei:&String,ste:& mut  HashMap<String,i32>){
    let t:Option<char> = Some(hei.chars().nth(0).unwrap());
    let he = hei;
match t{
Some('.')=>{
    println!("Recognized Command");
},
_=>{
commands(he,ste);
}
}
}
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    while true {
       print_prompt();
        let mut input = String::new();
        {
        inputin(&mut input);     
        }
        chekforcommands(& input, & mut scores);
    }
}

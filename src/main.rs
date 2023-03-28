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
fn commands(star:&String, ste:& mut HashMap<String,i32>) {
    let w = star.unicode_words().collect::<Vec<&str>>();
    if w.len() <= 5 {
        if w[0].to_uppercase() == "INSERT" {
            let k:String = w[1].to_string();
            let v:i32 = w[2].to_string().parse().unwrap();
            ste.insert(k, v);
            println!("Inserted")
        }
        else if w[0].to_uppercase() == "FIND" {
            let t:String = w[1].to_string();
           let v =ste.get(&t);
           let mut k =0;
           match v {
              Some(i) => k=*i,
              _ => (),
           }
            println!("Key:{t} , Value:{k}")
        }
        else if w[0].to_uppercase() == "UPDATE" {

        }
        else if w[0].to_uppercase() == "DELETE" {
            let k:String = w[1].to_string();
            ste.remove(&k);
        }
        else if w[0].to_uppercase() == "SHOW"{
            
        }
        else {
            println!("Wrong Command")
        }
    }
   
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

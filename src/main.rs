use std::io;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use std::io::{stdout, Write};

fn print_prompt(){
    let mut lock = stdout().lock();
    write!(lock, "db>").unwrap();
    io::stdout().flush();
}
fn inputin(s:&mut String){
    io::stdin().read_line(s).expect("failed to readline");
}
fn commands(star:&String,file:&mut std::fs::File){
    let w = star.unicode_words().collect::<Vec<&str>>();
    let k:String = w[0].to_string();

        if(k.to_uppercase() == "INSERT"){
            let y = w[1].to_string();
            println!("{y}");
            file.write_all(y.as_bytes()).expect("failed")
    }
}
fn chekforcommands(hei:&String, file:&mut std::fs::File){
    let t:Option<char> = Some(hei.chars().nth(0).unwrap());
    let he = hei;
match t{
Some('.')=>{
    commands(he,file);
},
_=>{
commands(he,file);
}
}
}
fn main() {
    let mut file = std::fs::File::create(".trekup").expect("create failed");
    while true {
       print_prompt();
        let mut input = String::new();
        {
        inputin(&mut input);     
        }
        chekforcommands(& input, &mut file);
    }
}
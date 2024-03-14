use std::io;
use std::fs::OpenOptions;
use std::io::Read;
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
fn error_output(star:String){
    
    println!("Fatal: {} not a command",&star[1..]);
}
fn commands(star:&String){
    let dir_path = "/home/adwait/SDE/test-db/"; // Replace with the path to your directory
    let file_name = "adwait.trekup";
    let full_path = format!("{}/{}", dir_path, file_name);
    let mut file = OpenOptions::new()
    .create(true)
    .read(true)
    .write(true).open(full_path).expect("create failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let w = star.unicode_words().collect::<Vec<&str>>();
    let k:String = w[0].to_string();

        if(k.to_uppercase() == "INSERT"){
            let y = w[1].to_string();
            println!("{}", {&contents});
            file.write_all(y.as_bytes()).expect("failed")
    }
}
fn main_commands(star:&String){
    let w = star.unicode_words().collect::<Vec<&str>>();
    if(w.len()>= 1){
    let k:String = w[0].to_string();
        if (k.to_uppercase() == "EXIT"){

            std::process::exit(0)
        }
        else{
                error_output(star.trim().to_string())
            }
    
}
}
fn chekforcommands(hei:&String){
    let t:Option<char> = Some(hei.chars().nth(0).unwrap());
    let he = hei;
match t{
Some('.')=>{
    main_commands(he);
},
_=>{
commands(he);
}
}
}

fn main() {
    
    while true {
       print_prompt();
        let mut input = String::new();
        {
        inputin(&mut input);     
        }
        chekforcommands(& input) ;
    }
}
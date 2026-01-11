use std::io;

fn main(){
    let mut text =String::new();
    println!("enter a String: ");
    match io::stdin().read_line(&mut text){
        Ok(_)=> {
            let reversed: String =text.trim().chars().rev().collect();
            println!("reversed string is {}",reversed);
        }Err(e)=>println!("error: {}",e),
    }
}

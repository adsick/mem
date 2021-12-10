use mem::{MemApp, Command};
use inquire::{Text, Select};
fn main() {
    let mut app = MemApp::init("data.txt");
    
    let mut c: Command;
    let cl = Command::short_list();
    let ll = Command::long_list();
    let command_descriptions = ll.join("\n");
    loop {
        
        // let ans = Select::new(">", ll.clone()).prompt();
        println!("{}", command_descriptions);
        let ans = Text::new(">").prompt();
        match ans{
            Ok(ans) =>{
                if cl.contains(&ans) || ll.contains(&ans){

                    match ans.as_str(){
                        "x" | "exit" => break,
                        "l" | "list" => {app.list();}
                        _ => println!("not implemented")
                    }



                } else {
                    println!("there is no such command")
                }

            },
            Err(e) => {
                println!("{:?}",  e)
            }
        } 

    }


}
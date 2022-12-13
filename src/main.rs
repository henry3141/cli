use make_command::command;
use ros::{Cli,Command,Api,Arg,ArgPos};
use std::collections::HashMap;
use std::process;
use colored::Colorize;

fn exit(_s:&mut Command,_api:Api,args:HashMap<String,String>) {
    if args.get("exit_code").is_some() {
        process::exit(args.get("exit_code").unwrap().parse().unwrap());
    }
    process::exit(0)
}


#[command("Prints the msg",vec![Arg::new("Echo print message.".to_string(), ArgPos::Int(0), "msg".to_string(), false)])]
fn echo(_s:&mut Command,_api:Api,args:HashMap<String,String>) {
    println!("{}",args.get("msg").unwrap());
}

#[command("Prints help.",vec![Arg::new("Command to give help to".to_string(), ArgPos::Int(0), "comm_name".to_string(), true)])]
fn help(_s:&mut Command,api:Api,args:HashMap<String,String>) {
    let comm_name = args.get("comm_name");
    if comm_name.is_none() {
        for i in api.get_commands() {
            println!("{}",i);
        }
    } else {
        let comm_name = comm_name.unwrap();
        for i in api.get_commands() {
            if i.get_name() == comm_name.clone() {
                println!("{}",i);
                return;
            }
        }
        println!("{}",format!("Command {} not found.",comm_name).red());
    }
}

#[command("Clears the screen.",vec![])]
fn cls(_s:&mut Command,_api:Api,_args:HashMap<String,String>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}



fn main() {
    let mut cli = Cli::create();
    let exit = Command::new(String::from("exit"), String::from("Exit the Cli"),vec![Arg::new("Exit code".to_string(), ArgPos::Int(0), "exit_code".to_string(), true)], exit);
    cli.add_command(echoCommand());
    cli.add_command(helpCommand());
    cli.add_command(clsCommand());
    cli.add_command(exit);
    cli.run();
}

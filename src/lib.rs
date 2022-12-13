use std::fmt::{Debug, Display};
use std::{collections::HashMap, io::Write,};
use colored::Colorize;
use std::env;
use getch::Getch;
use std::io;
use simsearch::SimSearch;
use enable_ansi_support;

#[derive(Debug,Clone)]
pub struct Arg {
    description:String,
    position:ArgPos,
    name:String,
    opt:bool,
}

impl Arg {
    pub fn new(description:String,position:ArgPos,name:String,opt:bool) -> Arg {
        Arg { description, position, name, opt }
    }
}

#[derive(Debug,Clone)]
pub enum ArgPos {
    Prefix(String),
    Int(i32),
    Exist(String),
}



#[derive(Clone)]
pub struct Command {
    name:String,
    description:String,
    args:Vec<Arg>,
    func:fn(&mut Command,Api,HashMap<String,String>),
}

impl Command {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command").field("name", &self.name).field("args", &self.args).finish()
    }
}

impl Display for Command {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{} - {}:",self.name,self.description);
        for i in &self.args {
            match &i.position {
                ArgPos::Exist(j) => {println!("  [{j}] -> {}",i.description)},
                ArgPos::Int(j) => {print!("  {} ",self.name);
                for _i in 0..j.clone() {
                    print!(" []");
                }
                print!(" [{}] -> {}",i.name,i.description);},
                ArgPos::Prefix(j) => {println!("  {}[{}] -> {}",j,i.name,i.description)}
            }
        }
        std::fmt::Result::Ok(())
    }
}

impl Command {
    pub fn new(name:String,desc:String,args:Vec<Arg>,func:fn(&mut Command,Api,HashMap<String,String>)) -> Command {
        Command { name: name, args: args, func: func , description:desc}
    }

    pub fn call(&mut self,api:Api,args:HashMap<String,String>) {
        (self.func)(self,api,args);
    }
}

pub struct Api {
    cli:Cli,
}

impl Api {
    pub fn new(cli:Cli) -> Api {
        Api {cli:cli}
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &&self.cli.commands
    }
}


#[derive(Clone)]
pub struct Cli {
    commands:Vec<Command>,
}

impl Cli {
    pub fn create() -> Cli {
        Cli {commands:vec![]}
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn add_command(&mut self,command:Command) {
        self.commands.push(command);
    }

    fn find_near(&self,with:&str) -> Vec<String> {
        let mut engine: SimSearch<u32> = SimSearch::new();
        let mut x = 0;
        let mut map = HashMap::new();
        for i in &self.commands {
            engine.insert(x,&i.name);
            map.insert(x, &i.name);
            x += 1;
        }
        let results: Vec<u32> = engine.search(with);
        let mut re = vec![];
        x = 0;
        for i in results {
            if x == 5 {
                break;
            }
            let s = map.get(&i).unwrap().clone().clone();
            if s != with {
                re.push(s);
                x += 1;
            } 
        }
        re
    }

    ///add help if correct
    pub fn input(&self) -> String {
        let get = Getch::new();
        let mut current_in = String::new();
        let username = env::var("USERNAME").unwrap();
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.to_str().to_owned().unwrap();
        let computer_name = env::var("COMPUTERNAME").unwrap();
        print!("{} @ {} - {}\n{} {}\n",username.yellow(),computer_name.purple(),cwd.blue(),"$".red(),&current_in);
        let up = (27 as char).to_string() + "[1F";
        let mut last:Vec<String> = vec![];
        loop {
            let char = get.getch().unwrap();
            if char == 8 {
                current_in.pop();
            } else if char == 9 {
                if last.len() > 0 {
                    current_in = last[0].clone();
                }
            }
            else {
                match char as char {
                    '\x0D' => break,
                    _ => current_in.push(char as char),
                }
            }
            for _i in &last {
                println!("{}","                                     ");
            }
            for _i in &last {
                print!("{}",&up);
            }
            let username = env::var("USERNAME").unwrap();
            let cwd = env::current_dir().unwrap();
            let cwd = cwd.to_str().to_owned().unwrap();
            let computer_name = env::var("COMPUTERNAME").unwrap();
            let strr = (&current_in).clone() + "                                    ";
            print!("{}{}{} @ {} - {}\n{} {}\n",&up,&up,username.yellow(),computer_name.purple(),cwd.blue(),"$".red(),&strr);
            for i in self.find_near(&current_in) {
                println!("{}",((&i).clone()+"                          ").bright_black());
            }
            for _i in self.find_near(&current_in) {
                print!("{}",&up);
            }
            io::stdout().flush().unwrap();
            last = self.find_near(&current_in)
        }
        for _i in self.find_near(&current_in){
            println!("{}","                                     ");
        }
        for _i in self.find_near(&current_in) {
            print!("{}",&up);
        }
        current_in
    }

    pub fn run(&mut self) {
        let x = enable_ansi_support::enable_ansi_support();
        match x {
            Ok(()) => {},
            Err(_i) => {panic!("Ansi could not be activated.")}
        }
        loop {
            let clone = self.clone();
            let input = self.input();
            let comm = input.split(" ").collect::<Vec<&str>>();
            let mut args = vec![];
            for i in comm {
                args.push(i.to_owned());
            }
            let mut found = false;
            for i in &mut self.commands {
                if i.name == args[0] {
                    args.remove(0);
                    //may not contain opt prefix arg key
                    let mut pargs:HashMap<String, String> = HashMap::new();
                    let mut ok_args = true;
                    for x in &i.args {
                        match &x.position {
                            ArgPos::Prefix(p) => {
                                if args.iter().any(|e| e == p) {
                                    let mut next_is = false;
                                    args.iter().for_each(|s| {
                                        if s == p {
                                            next_is = true;
                                        } else if next_is {
                                            pargs.insert(x.name.clone(), s.clone());
                                        }
                                    });
                                } else if x.opt == false {
                                    println!("{}",format!("Forced Prefix arg {} was not present!",x.name).red());
                                    ok_args=false
                                }
                            },
                            ArgPos::Int(i) => {
                                if args.len() <= i.clone() as usize && !x.opt {
                                    println!("{}",format!("Posistional Agument {} was not present!",x.name).red());
                                    ok_args = false;
                                } else if args.len() <= i.clone() as usize {
                                } else {
                                    pargs.insert(x.name.clone(), args[i.clone() as usize].to_owned());
                                }
                            },
                            ArgPos::Exist(s) => {if args.iter().any(|e| e == s) {
                                    pargs.insert(x.name.clone(), String::from("true"));
                                }
                                    else if x.opt == true {
                                        pargs.insert(x.name.clone(), String::from("false"));
                                } else {
                                    println!("{}",format!("Forced Exist arg {} was not present!",x.name).red());
                                    ok_args=false
                            }},
                        }
                    }
                    if ok_args {
                        i.call(Api::new(clone),pargs);
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                println!("{}","Command not Found!".red());
            }
        }
    }
}


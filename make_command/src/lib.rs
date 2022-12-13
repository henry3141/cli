use proc_macro::{TokenStream};


#[proc_macro_attribute]
pub fn command(args: TokenStream, mut func: TokenStream) -> TokenStream {
    let arg_str = args.to_string();
    let mut fstr = func.to_string();
    fstr.remove(0);fstr.remove(0);fstr.remove(0);
    let mut name = String::from("");
    for i in fstr.chars() {
        if i == '(' {
            break;
        }
        name.push(i)
    }
    let args = arg_str.split_once(",");
    if args.is_none() {
        panic!("Command needs desc and args argument suplied!");
    }
    let args = args.unwrap();
    let desc = args.0;
    let args2 = args.1;
    let s = format!(
"fn {name}Command() -> Command {{
    Command::new(\"{name}\".to_string(),{desc}.to_string(),{args2}, {name})
}}");
    let stream:TokenStream = s.parse().unwrap();
    func.extend(stream);
    func
}
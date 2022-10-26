use std::{error::Error};
use clap::{Parser, Arg, Command};
use inquire::{Text, Confirm, Select};

pub trait InteractiveParse
where Self: Sized
{
    fn interactive_parse() -> Result<Self, Box<dyn Error>>;
}

impl<T> InteractiveParse for T
where T: Parser {
    fn interactive_parse() -> Result<Self, Box<dyn Error>> {
        let base_command = T::command();
        let mut args = vec![base_command.get_name().to_string()];
        let mut command = &base_command;
        loop {
            args.extend(get_args(command.get_arguments())?);
            let subcommands: Vec<&Command> = command.get_subcommands().collect();
            if subcommands.len() == 0 { break; }
            if !command.is_subcommand_required_set() {
                if !add_optional_command(command)? { break; }
            }
            command = Select::new(command.get_name(), subcommands).prompt()?;
            args.push(command.get_name().to_string());
        }
        Ok(T::parse_from(args))
    }
}

fn parse_required_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    let mut output_args = vec![];
    let id = arg.get_id();
    match arg.is_positional() {
        // Arg is positional
        true => { },
        // Arg uses flag
        false => { output_args.push(format!("--{}", id)); },
    }
    let mut text = Text::new(arg.get_id().as_str());

    // Add a help string
    #[cfg(debug_assertions)]
    let mut help_string = get_type_string(arg);

    #[cfg(not(debug_assertions))]
    let mut help_string = String::default();

    #[cfg(debug_assertions)]
    if let Some(help) = arg.get_help() {
        help_string = format!("{}: {}", help_string, help);
    }

    #[cfg(not(debug_assertions))]
    if let Some(help) = arg.get_help() {
        help_string = format!("{}: {}", help_string, help);
    }

    text = text.with_help_message(help_string.as_str());

    output_args.push(text.prompt()?);
    Ok(output_args)
}

fn add_optional_command(command: &Command) -> Result<bool, Box<dyn Error>> {
    let mut confirm = Confirm::new("Add optional command?");
    if let Some(help) = command.get_subcommand_value_name() {
        confirm = confirm.with_help_message(help);
    }
    Ok(confirm.prompt()?)
}

fn parse_optional_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    match Confirm::new("Add optional value?")
    .with_help_message(arg.get_id().as_str())
    .prompt()? 
    {
        true => { parse_required_arg(arg) },
        false => Ok(vec![]),
    }
}

fn parse_vec_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    let mut new_args = parse_optional_arg(arg)?;
    let mut total_args = vec![];
    while new_args.len() != 0 {
        total_args.extend(new_args);
        new_args = parse_optional_arg(arg)?;
    }
    if let Some(value_delimiter) = arg.get_value_delimiter() {
        total_args.join(value_delimiter.to_string().as_str());
    }
    Ok(total_args)
}

fn parse_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    match arg.get_num_args() {
        // arg is a vec
        Some(_) => {
            parse_vec_arg(arg)
        },
        None => {
            match arg.is_required_set() {
                true => parse_required_arg(arg),
                false => parse_optional_arg(arg),
            }
        },
    }

}

fn get_args<'a>(command: impl Iterator<Item = &'a Arg>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut arg_list = vec![];
    for arg in command {
        arg_list.extend(parse_arg(arg)?);
    }
    Ok(arg_list)
}

#[cfg(debug_assertions)]
fn get_type_string(arg: &Arg) -> String {
    let value_parser = arg.get_value_parser();
    let type_id = value_parser.type_id();
    let mut long_type_name = format!("{:?}", type_id);
    long_type_name = long_type_name.replace("(", "");
    long_type_name = long_type_name.replace(")", "");
    let mut type_list: Vec<&str> = long_type_name.split(", ").collect();
    for type_str in &mut type_list {
        if let Some(split) = type_str.rsplit_once(":") {
            *type_str = split.1
        }
    }
    let combine = type_list.join(",");
    let output = format!("<{}>", combine);
    output
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::fmt::Debug;
    #[cfg(debug_assertions)]
    use clap::CommandFactory;

    use super::*;

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None, subcommand_value_name="my_subcommand")]
    struct Git {
        /// my_subcommand doc
        #[command(subcommand)]
        my_subcommand: Option<SubCommand>,

        /// MyArg help string
        #[arg(required=false, value_parser=tuple_parser::<String, String>)]
        my_arg: Option<Vec<(String, String)>>
    }

    /// Other heading
    #[derive(Parser, Debug)]
    #[clap(rename_all = "snake_case", infer_subcommands=true)]
    enum SubCommand {
        Commit {
            #[arg(required=false)]
            message: Option<String>
        },
        Clone {
            #[arg(value_parser=tuple_parser::<String, String>)]
            address: Vec<(String, String)>
        },
        Merge {
            #[arg(value_delimiter=',')]
            address: Vec<String>
        }
    }

    pub fn tuple_parser<T, U>(s: &str) -> Result<(T, U), String> 
    where T: FromStr, U: FromStr, <T as FromStr>::Err: Debug, <U as FromStr>::Err: Debug,
    {
        let vec: Vec<&str> = s.split(',').collect();
        Ok((T::from_str(vec[0]).unwrap(), U::from_str(vec[1]).unwrap()))
    }

    #[ignore]
    #[test]
    fn test_interactive() {
        let git = Git::interactive_parse().unwrap();
        println!("{:?}", git);   
    }

    #[ignore]
    #[test]
    fn test_static() {
        let args = ["git", "-h"];
        let git = Git::parse_from(args);
        println!("{:?}", git);   
    }

    #[cfg(debug_assertions)]
    #[test]
    fn test_get_type_string() {
        let arg = Git::command().get_arguments().find(|x| x.get_id() == "my_arg").unwrap().clone();
        let type_string = get_type_string(&arg);
        assert_eq!(type_string.as_str(), "<String,String>")
    }
}
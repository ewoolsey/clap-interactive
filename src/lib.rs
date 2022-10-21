use std::{error::Error};
use clap::{Parser, Arg, Command};
use inquire::{Text, Confirm, Select};

pub trait InteractiveParse
where Self: Sized
{
    fn interactive_parse() -> Result<Self, Box<dyn Error>>;
}

fn parse_required_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    let mut output_args = vec![];
    let id = arg.get_id();
    match arg.is_positional() {
        // Arg is positional
        true => {

        },
        // Arg uses flag
        false => {
            output_args.push(format!("--{}", id));
        },
    }
    let string = Text::new(arg.get_id().as_str()).prompt()?;
    output_args.push(string);
    Ok(output_args)
}

fn parse_optional_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    match Confirm::new("Add optional value?")
    .with_help_message(arg.get_id().as_str())
    .prompt()? 
    {
        true => {
            parse_required_arg(arg)
        },
        false => Ok(vec![]),
    }
}

fn parse_vec_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    let value_delimiter = arg.get_value_delimiter();
    match value_delimiter {
        Some(value_delimiter) => {
            let mut new_args = parse_optional_arg(arg)?;
            let mut total_args = vec![];
            while new_args.len() != 0 {
                total_args.extend(new_args);
                new_args = parse_optional_arg(arg)?;
            }
            total_args.join(value_delimiter.to_string().as_str());
            Ok(total_args)
        },
        None => {
            let mut new_args = parse_optional_arg(arg)?;
            let mut total_args = vec![];
            while new_args.len() != 0 {
                total_args.extend(new_args);
                new_args = parse_optional_arg(arg)?;
            }
            Ok(total_args)
        }
    }
}


fn parse_arg(arg: &Arg) -> Result<Vec<String>, Box<dyn Error>> {
    println!("num_args: {:?}", arg.get_num_args());
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
            command = Select::new(command.get_name(), subcommands).prompt()?;
            args.push(command.get_name().to_string());
        }
        Ok(T::parse_from(args))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Git {
        #[command(subcommand)]
        subcommand: SubCommand,

        #[arg(required=false)]
        my_arg: Option<String>
    }

    #[derive(Parser, Debug)]
    #[clap(rename_all = "snake_case", infer_subcommands=true)]
    enum SubCommand {
        Commit {
            #[arg(required=false)]
            message: Option<String>
        },
        Clone {
            address: Vec<String>
        },
        Merge {
            #[arg(value_delimiter=',')]
            address: Vec<String>
        }
    }

    #[test]
    fn test() {
        let git = Git::interactive_parse().unwrap();
        println!("{:?}", git);   
    }
}
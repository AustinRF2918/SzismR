use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;

extern crate regex;
extern crate ansi_term;

mod command_structure;
mod task_structure;
mod rc_parser;
mod arg_parser;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::Blue;

fn main()
{
    //Collect our command line strings.
    let mut arg_parsed = arg_parser::parse::ArgumentObject::new();

    //Preferably we put a parsed rc into this.
    arg_parsed.parse(env::args(),
     vec![
    "run",
    "show",
    "make"],
     vec![
    "fsAutoTemplater",
    "gulpInit",
    "sassInit",
    "bsInit",
    "htmlInit"],
    vec![
    "-d",
    "-dialogue",
    "-web",
     ]);


    //Place them into a command driver which analyizes them and
    //passes them off to other portions.
    command_driver(&mut arg_parsed);
}

fn command_driver(arguments : &mut arg_parser::parse::ArgumentObject)
{
    //Probably should turn these into enumerative types.
    let mut flag_debug = false;
    let mut dialogue_debug = false;

    //Scan for important flags.
    //These may overwrite our original noun verb structures, which is
    //Okay, because then we can offer help.
    for arg in arguments.get_flags().iter()
    {
        //Debug flag.
        if arg == "-d"
        {
            flag_debug = true;
        }

        if arg == "-dialogue"
        {
            dialogue_debug = true;
        }
    }

    if dialogue_debug == true
    {
        println!("{}",Yellow.paint( ".............................................."));
        println!("{}", Blue.paint(".Szism Toolbox | Copyright 2016 | Austin Fell."));
        println!("{}", Green.paint(".............................................."));
        println!("{}", Red.paint(".A file manager,                             ."));
        println!("{}", Green.paint(".     A preset manager,                      ."));
        println!("{}", Blue.paint(".           code generator,                  ."));
        println!("{}", Yellow.paint(".               DRY facilitator.             ."));
        println!("{}", Blue.paint(".............................................."));
    }

    if arguments.get_verb() == "show"
    {
        show_loaded_scripts(flag_debug);
    }

    let target = arguments.get_nouns();
    let verb = arguments.get_verb();
    let flags = arguments.get_flags();

    let mut node = command_structure::comm::node::new(verb, target, flags.clone());


    //Debug display.
    //Only callable via -d flag. Developer tool.
    if flag_debug
    {
        &node.debug_display("Function Driver".to_string());
    }

    //Run with our node perameter: this will run whatever
    //script in whatever language we want.
    println!("{}", arguments.get_verb());
    if arguments.get_verb() == "run"
    {
        execute_script_utility(&mut node, flag_debug);
    }

    if arguments.get_verb() == "make"
    {
        if contains(flags, "-web".to_string())
        {
            execute_script_make("web".to_string(), flag_debug);
            println!("I suggest running 'szism run gulpInit'.");
        }
    }
}

fn contains(collect: Vec<String>, desired: String) -> bool
{
    for i in collect
    {
        if desired == i
        {
            return true;
        }
    }

    return false;
}

//Definately remake this to be more modular.
fn show_loaded_scripts(debug : bool)
{
    match env::current_exe() {
        Ok(mut exe_path) => {
            //Make tokenizer for our parseable RC from here.
            //Parse stuff here.
            let mut rc_stack = Vec::new();

            //Parse a rc_location to get all our loaded plugins.
            rc_stack.push("configuration");
            rc_stack.push("swdl.rc");

            //Create our parser object.
            &exe_path.pop();

            let command_hash = make_rc_dict(&exe_path, rc_stack, debug);

            //Parse the rc and, define our h_parse object as the dictionary of all the objects.

            if debug
            {
                println!("FROM SCRIPT __SHOW__");
            }
            for value in command_hash.keys(){
                println!("Loaded script: {}", Green.paint(value.to_string()));
            }
        }

        Err(_) => {
            //Debug portion to show transference of data,
            println!("{}", Red.paint("Your executable is in a bad path. (Whatever that means...)"));

            //Error code 2.
            std::process::exit(2);
        },
    }
}

//Get the second argument and use it to determine script attributes.
fn execute_script_utility(node : &mut command_structure::comm::node, debug : bool)
{
    //Match the current executable path.
    match env::current_exe() {
        Ok(mut exe_path) => {
            if debug
            {
                //Our executable path was found. I'm not sure when this
                //wouldn't be possible to find? Maybe if we were running
                //from a stream or something? I'm not sure...
                println!("Executing Scripts: {:?}", node.get_targets());
                println!("From executable directory {}", exe_path.display());

                //Debug portion to show transference of data,
                &node.debug_display("Execute Script SUCCESS".to_string());
            }

            //Make tokenizer for our parseable RC from here.
            //Parse stuff here.
            let mut rc_stack = Vec::new();

            //Parse a rc_location to get all our loaded plugins.
            rc_stack.push("configuration");
            rc_stack.push("swdl.rc");

            //Create our parser object.
            let loc = &node.get_targets().clone();
            &exe_path.pop();

            let command_hash = make_rc_dict(&exe_path, rc_stack, debug);

            let mut plugin_stack = vec!["plugins", "std"];

            for i in loc {
                match command_hash.get(i){
                    Some(t) => {
                        plugin_stack.push(t);
                    },

                    None => {
                        println!("{}", Red.paint("That script does not exist :(."));
                        println!("{}", Red.paint("Exiting."));
                        //Error code 1 : Script not found.
                        std::process::exit(1);
                    },
                };
            }
            //Implement this as tokenizer.

            let script_executable = mutate_path(&exe_path, debug, plugin_stack);

            call_script(&script_executable, debug);

        },
        Err(e) => {
            //Executable path wasn't found.
            println! ("{}:{}", Red.paint("Failed to get current executable directory: {}"), e);
            //Debug portion to show transference of data,
            &node.debug_display("Execute Script ERROR".to_string());
        },
    };
}

//Get the second argument and use it to determine script attributes.
fn execute_script_make(mut t: String, debug : bool)
{
    //Match the current executable path.
    match env::current_exe() {
        Ok(mut exe_path) => {
            &exe_path.pop();
            t.push_str(".py");
            let plugin_stack = vec!["plugins", "init", t.as_str()];

            let script_executable = mutate_path(&exe_path, debug, plugin_stack);

            call_script(&script_executable, debug);

        },
        Err(e) => {
            //Executable path wasn't found.
            println! ("{}:{}", Red.paint("Failed to get current executable directory: {}"), e);
            panic!("Failed to get HTML template make.")
        },
    };
}


fn mutate_path(root : &PathBuf, debug : bool, plugin_tokens : Vec<&str>) -> PathBuf
{
    //We're gonna return a new path without destroying our old root.
    let mut ret_path = root.clone();

    //Get all the data from our tokenizer/database.
    for folder in plugin_tokens
    {
        ret_path.push(folder.to_string());
    }

    if debug
    {
        println!("  ____PATH DATA______");
        println!("      Base path name: {:?}", root);
        println!("      Mutated path name: {:?}", ret_path);
        println!(" ");
    }

    return ret_path;
}

fn call_script(script_executable : &PathBuf, debug : bool)
{
    //Only for python right now: add more later.
    //Preferably port all of the old scripts to Rust.
    //Call our python scripts with python3, should be built into bashrc. put pathname as argument.
    let command_status = Command::new("python3")
                                     .arg(script_executable).status().unwrap_or_else(
        |e| {
            panic!("Failed to execute process: {}", e);
        }
    );

    if debug
    {
        println!("Process exited sucessfully with: {}", command_status);
    }
}

fn make_rc_dict(exe_path: &PathBuf, rc_stack: Vec<&str>, debug: bool) -> HashMap<String, String>
{
    let h_parse = rc_parser::parse::hash_parser::new();
    h_parse.parse_rc(&mutate_path(&exe_path, debug, rc_stack), debug)
}

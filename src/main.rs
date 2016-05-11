use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;

extern crate regex;

mod command_structure;
mod task_structure;
mod rc_parser;

fn main()
{
    //Collect our command line strings.
    let mut args: Vec<String> = env::args().collect();

    //Place them into a command driver which analyizes them and
    //passes them off to other portions.
    command_driver(&mut args);
}

fn command_driver(x : &mut Vec<String>)
{
    //Probably should turn these into enumerative types.
    let mut flag_debug = false;
    let mut dialogue_debug = false;

    //Scan for important flags.
    //These may overwrite our original noun verb structures, which is
    //Okay, because then we can offer help.
    for arg in x.iter()
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
        println!("..............................................");
        println!(".Szism Toolbox | Copyright 2016 | Austin Fell.");
        println!("..............................................");
        println!(".A file manager,                             .");
        println!(".     A preset manager,                      .");
        println!(".           Code Generator,                  .");
        println!(".               DRY Facilitator.             .");
        println!("..............................................");
    }

    //Filling up data entry values for display.
    if x.len() > 1
    {
        //First inserted argument is always command. Everything is a sentence in Szism.
        let command = x[1].clone();

        //Checks for show command, which will display all loaded scripts from RC file.
        //We dont put this below because show does not take a perameter (yet).

        if x[1] == "show"
        {
            show_loaded_scripts(flag_debug);
        }


        if x.len() > 2
        {
            let target = x[2].clone();

            //Make a node for our called functions, we can modify this
            //into a callable script and run trees of em later!
            let mut node = command_structure::comm::node::new(command, target);

            //Push onto our node a bunch of our flags
            if x.len() > 3
            {
                //All objects after our noun verb structure are
                //modifiers/adjetives: flags. Thus, I search at
                //the third index for these critical flags.
                for i in 3..x.len()
                {
                    let temp = x[i].clone();
                    &node.push_flag(temp);
                }
            }

            //Debug display.
            //Only callable via -d flag. Developer tool.
            if flag_debug
            {
                &node.debug_display("Function Driver".to_string());
            }

            if x[1] == "run"
            {
                execute_script(&mut node, flag_debug);
            }


        }

    }
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
            rc_stack.push("configuration".to_string());
            rc_stack.push("swdl.rc".to_string());

            //Create our parser object.
            &exe_path.pop();

            let command_hash = make_rc_dict(&exe_path, rc_stack, debug);

            //Parse the rc and, define our h_parse object as the dictionary of all the objects.

            for value in command_hash.keys(){
                println!("Loaded script: {}", value);
            }
        }

        Err(_) => {
            //Debug portion to show transference of data,
            println!("Your executable is in a bad path. (Whatever that means...)");

            //Error code 2.
            std::process::exit(2);
        },
    }
}

//Get the second argument and use it to determine script attributes.
fn execute_script(node : &mut command_structure::comm::node, debug : bool)
{
    //Match the current executable path.
    match env::current_exe() {
        Ok(mut exe_path) => {
            if debug
            {
                //Our executable path was found. I'm not sure when this
                //wouldn't be possible to find? Maybe if we were running
                //from a stream or something? I'm not sure...
                println!("Executing Script: {}", node.get_target());
                println!("From executable directory {}", exe_path.display());

                //Debug portion to show transference of data,
                &node.debug_display("Execute Script SUCCESS".to_string());
            }

            //Make tokenizer for our parseable RC from here.
            //Parse stuff here.
            let mut rc_stack = Vec::new();

            //Parse a rc_location to get all our loaded plugins.
            rc_stack.push("configuration".to_string());
            rc_stack.push("swdl.rc".to_string());

            //Create our parser object.
            let loc = &node.get_target().clone();
            &exe_path.pop();

            let command_hash = make_rc_dict(&exe_path, rc_stack, debug);

            let mut plugin_stack = Vec::new();

            //Right now only works with standard plugins.
            //We gotta parse the RC for the parse pack and
            //append the pack value as well.
            plugin_stack.push("plugins".to_string());
            plugin_stack.push("std".to_string());

            match command_hash.get(loc){
                Some(t) => {
                    plugin_stack.push(t.to_string());
                },

                None => {
                    println!("That script does not exist :(.");
                    println!("Exiting.");
                    //Error code 1 : Script not found.
                    std::process::exit(1);
                },
            };
            //Implement this as tokenizer.

            let script_executable = mutate_path(&exe_path, debug, plugin_stack);

            call_script(&script_executable, debug);

        },
        Err(e) => {
            //Executable path wasn't found.
            println! ("Failed to get current executable directory: {}", e);
            //Debug portion to show transference of data,
            &node.debug_display("Execute Script ERROR".to_string());
        },
    };
}


fn mutate_path(root : &PathBuf, debug : bool, plugin_tokens : Vec<String>) -> PathBuf
{
    //We're gonna return a new path without destroying our old root.
    let mut ret_path = root.clone();

    //Get all the data from our tokenizer/database.
    for folder in plugin_tokens
    {
        ret_path.push(folder);
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

fn make_rc_dict(exe_path: &PathBuf, rc_stack: Vec<String>, debug: bool) -> HashMap<String, String>
{
    let h_parse = rc_parser::parse::hash_parser::new();
    h_parse.parse_rc(&mutate_path(&exe_path, debug, rc_stack), debug)

}

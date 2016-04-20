use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::os;
use std::process::Command;
use std::fs::File;
use std::io::Read;

struct CommandNode
{
    id: String,
    target: String,
    flags:  Vec<String>
}


impl CommandNode{
    pub fn new(arg : String, arg2 : String) -> CommandNode{
        CommandNode{
            id : arg,
            target : arg2,
            flags : Vec::new()
        }
    }

    pub fn push_flag(&mut self, arg: String){
        self.flags.push(arg);
    }

    pub fn get_command(&mut self) -> &String
    {
        return &self.id;
    }

    pub fn get_target(&mut self) -> &String
    {
        return &self.target;
    }

    pub fn get_flags(&mut self) -> &Vec<String>
    {
        return &self.flags;
    }

    pub fn debug_display(&mut self, arg: String)
    {
        println!("");
        println!("_______DEBUG_______");
        println!("       CALLER:");
        println!("          {}", arg);
        println!("Executing Script: {}", &self.get_command());
        println!("With Target: {}", &self.get_target());
        println!("Flag List: ");
        for i in &self.flags
        {
             println!("{:?}", i)
        }
        println!(" ");
    }
}

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
        println!(".                                            .");
        println!(".                                            .");
        println!(".                                            .");
        println!("..............................................");
    }

    //Filling up data entry values for display.
    if x.len() > 1
    {
        let command = x[1].clone();

        if x.len() > 2
        {
            let target = x[2].clone();

            //Make a node for our called functions, we can modify this
            //into a callable script and run trees of em later!
            let mut node = CommandNode::new(command, target);

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

            execute_script(&mut node, flag_debug);
        }
    }
}

//Get the second argument and use it to determine script attributes.
fn execute_script(node : &mut CommandNode, debug : bool)
{
    match env::current_dir() {
        Ok(exe_path) => {
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
            let mut plugin_stack = Vec::new();
            plugin_stack.push("plugins".to_string());
            plugin_stack.push("std".to_string());
            plugin_stack.push("bs-AutoTemplateGenerator".to_string());
            plugin_stack.push("autoTemplater.py".to_string());
            //Implement this as tokenizer.


            //Parse a rc_location to get all our loaded plugins.
            let mut rc_stack = Vec::new();
            rc_stack.push("configuration".to_string());
            rc_stack.push("swdl.rc".to_string());

            //Parse stuff here.
            parse_rc(&exe_path, debug, rc_stack);
            let script_executable = mutate_path(node, &exe_path, debug, plugin_stack);

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

fn parse_rc(root : &PathBuf, debug : bool, rc_tokens : Vec<String>)
{
    //We're gonna return a new path without destroying our old root.
    let mut ret_path = root.clone();

    //Get all our path data.
    for token in rc_tokens
    {
        ret_path.push(token);
    }

    if debug
    {
        println!("  ____PATH DATA______");
        println!("      Base path name: {:?}", root);
        println!("      Mutated path name: {:?}", ret_path);
    }

    let mut rc_data = match File::open(&ret_path){
        Ok(mut f) => {
            let mut data = String::new();
            f.read_to_string(&mut data).unwrap();
            if debug
            {
                println!(" ");
                println!("      _____PASSED RC_________");
                println!(" ");
                println!("{}", data);
            }

        },
        Err(e) => {
             println!("Failed to open RC file: {}", e);
        }
    };
}

fn mutate_path(node : &mut CommandNode, root : &PathBuf, debug : bool, plugin_tokens : Vec<String>) -> PathBuf
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
        node.debug_display("Mutate Path".to_string());
        println!("  ____PATH DATA______");
        println!("      Base path name: {:?}", root);
        println!("      Mutated path name: {:?}", ret_path);
        println!(" ");
    }

    return ret_path;
}

fn call_script(script_executable : &PathBuf, debug : bool)
{
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

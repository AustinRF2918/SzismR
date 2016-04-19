use std::env;

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

        //String is for traceback of functions

    }
}

fn main()
{
    println!("..............................................");
    println!(".Szism Toolbox | Copyright 2016 | Austin Fell.");
    println!("..............................................");

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

    //Scan for important flags.
    //These may overwrite our original noun verb structures, which is
    //Okay, because then we can offer help.
    for arg in 0..x.len()
    {
        //Debug flag.
        if x[arg] == "-d"
        {
            flag_debug = true;
        }
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
    println!("Executing Script: {}", node.get_target());
    //Debug portion to show transference of data,
    if debug
    {
        &node.debug_display("Execute Script".to_string());
    }

}

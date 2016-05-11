/*
 * This is the structure for piping commands
 * together, as well as allowing user inputable
 * commands.
*/

pub mod csw{
    //For parsing file extensions.
    extern crate regex;
    use regex::Regex;

    //We are gonna make this able to interface with other scripting languages after
    //compilation because I don't want this to be unexpandable from the outside.
    pub enum language
    {
        Internal, //For our Rust standard calls.
        Python2,
        Python3,
        Ruby,
        JavaScript,
        Perl,
    }

    //Get's the extension of the specific thing we're calling: for example, if
    //we know what we are calling is python, it will make sure that it has a
    //.py extension before calling the interpreter.
    fn get_lang_extension(lang : language) -> String
    {
        match lang {
            language::Internal => {
                unimplemented!() //Unimplemented: we have to call on a standard
                //method/task container class to wrap this node in what we actually
                //need.
            },
            language::Python2 => {
                ".py".to_string() //We have to somehow determine which python version to use...
            },
            language::Python3 => {
                ".py".to_string() //We have to somehow determine which python versionb to use..
            },
            language::Ruby => {
                ".rb".to_string()
            },
            language::JavaScript => {
                ".js".to_string()
            },
            language::Perl => {
                ".pl".to_string()
            },
        }
    }

    //  This get's whatever programming language interpreter we need. Preferably we would allow
    // the compiler to check for differences, because some people have different bash rcs, so
    // we need to account for that. for now I'll have it like this, but later we will pass this
    // a "script-settings object, which can be enumerated as default or different."
    fn get_cmd_interpreter(lang : language) -> String
    {
        match lang {
            language::Internal => {
                unimplemented!(); //Unimplemented: we have to call on a standard
                //method/task container class to wrap this node in what we actually
                //need.
            },
            language::Python2 => {
                "python2".to_string()
            },
            language::Python3 => {
                "python3".to_string()
            },
            language::Ruby => {
                "ruby".to_string()
            },
            language::JavaScript => {
                "node".to_string()
            },
            language::Perl => {
                "perl".to_string()
            },
        }
    }

    pub struct node
    {
        //Command ID.
        id: String,

        //Absolute location of script on disk.
        script_location: String,

        //Flags being passed into our script
        flags:  Vec<String>,

        //If our node is asynchronized or not.
        is_async : bool,

        //What language this script runs under.
        task_language : language,

        //Is there anything else we might need?
    }

    //A command node is a callable node that can be used in a stack
    //To automate certain stuff: For now, it is only callable via
    //the run command, but it contains a grouping of flags, id
    //and target.

    impl node{
        pub fn new(arg : String, arg2 : String, arg3 : Vec<String>, arg4 : bool, arg5 : language) -> node{
            node{

                id : arg,
                script_location : arg2,
                flags : arg3,
                is_async : arg4,
                task_language : arg5,
                //TODO: Check validity of path and congruence with actual extension.
                //Let's implement this using regex.
            }
        }

        pub fn debug_display(&mut self, arg: String)
        {
            println!("");
            println!("_______DEBUG_______");
            println!("       CALLER:");
            for i in &self.flags
            {
                 println!("{:?}", i)
            }
            println!(" ");
        }
    }
}

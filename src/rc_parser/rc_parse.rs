/*
 * This is the structure for piping commands
 * together, as well as allowing user inputable
 * commands.
*/

pub mod parse{
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;

    pub struct hash_parser
    {
        //We need enumerative state here.
        tokens:  Vec<String>,
    }

    //A command node is a callable node that can be used in a stack
    //To automate certain stuff: For now, it is only callable via
    //the run command, but it contains a grouping of flags, id
    //and target.

    impl hash_parser{
        pub fn new(arg : String, arg2 : String) -> hash_parser{
            hash_parser{
                tokens : Vec::new(),
            }
        }

        fn push_token(&mut self, arg: String){
            //Push an execution flag to our command node.
            self.tokens.push(arg);
        }

        pub fn get_dict()
        {
            let v = HashMap::<String, String>::new();

        }

        pub fn parse_rc(root : &PathBuf, debug : bool, rc_tokens : Vec<String>)
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

            let rc_data = match File::open(&ret_path){
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

    }
}

/*
 * This is the structure for piping commands
 * together, as well as allowing user inputable
 * commands.
*/

pub mod comm{
    pub struct node
    {
        id: String,
        target: String,
        flags:  Vec<String>
    }

    //A command node is a callable node that can be used in a stack
    //To automate certain stuff: For now, it is only callable via
    //the run command, but it contains a grouping of flags, id
    //and target.

    impl node{
        pub fn new(arg : String, arg2 : String) -> node{
            node{
                id : arg,
                target : arg2,
                flags : Vec::new()
            }
        }

        pub fn push_flag(&mut self, arg: String){
            //Push an execution flag to our command node.
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
}

/*
 * This is the structure for piping commands
 * together, as well as allowing user inputable
 * commands.
*/

pub mod comm{
    pub struct node
    {
        command: String,
        targets: Vec<String>,
        flags:  Vec<String>
    }

    //A command node is a callable node that can be used in a stack
    //To automate certain stuff: For now, it is only callable via
    //the run command, but it contains a grouping of flags, id
    //and target.

    impl node{
        pub fn new(commandData : String, targetsData : Vec<String>, flagsData : Vec<String>) -> node{
            node{
                command : commandData,
                targets : targetsData,
                flags : flagsData,
            }
        }

        pub fn get_command(&mut self) -> &String
        {
            return &self.command;
        }

        pub fn get_targets(&mut self) -> &Vec<String>
        {
            return &self.targets;
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
            println!("Executing Command: {}", &self.get_command());

            println!("Target List");
            for i in &self.targets
            {
                 println!("{:?}", i)
            }

            println!("Flag List: ");
            for i in &self.flags
            {
                 println!("{:?}", i)
            }

            println!(" ");
        }
    }
}

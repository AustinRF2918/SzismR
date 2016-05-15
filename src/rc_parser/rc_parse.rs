/*
 * This is a portion for parsing into a hashmap
 * our parsed rc and allowing the user to call
 * pass it back to our main caller for the user
 * to do stuff with.
*/



pub mod parse{

    extern crate regex;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;


    //This is the "Pluginable" Portion of our code.
    pub struct HashParser 
    {
        tokens:  Vec<String>,
    }

    impl HashParser{
        pub fn new() -> HashParser{
                HashParser{
                tokens : Vec::new(),
            }
        }

        pub fn parse_rc(&self, root : &PathBuf, debug : bool) -> HashMap<String, String>
        {


            //We're gonna return a new path without destroying our old root.
            let ret_path = root.clone();
            let mut slice = &mut String::new();
            let mut ret_hash : HashMap<String, String> = HashMap::new();

            let rc_data = match File::open(&ret_path){
                Ok(mut f) => {


                    f.read_to_string(slice).unwrap();
                    {
                        //Parse tokens by new line.
                        let tokens : Vec<&str> = slice.split('\n').collect();

                        let p_pack = regex::Regex::new(r"(.*)\.parsePack").unwrap();
                        let p_ms_entry = regex::Regex::new(r"(.*)\.parseScripts").unwrap();
                        let p_s_entry = regex::Regex::new(r"(?P<script_caller>.*)\^entryPoint").unwrap();
                        let p_descope = regex::Regex::new(r"(.*)End").unwrap();
                        let s_entry = regex::Regex::new(r"(?P<script_entry>.*)").unwrap();
                        let s_dependancy = regex::Regex::new(r"(.*)").unwrap();

                        let mut scope = 0;

                        let mut current_key = String::new();
                        let mut current_path = String::new();


                        //Simple C++ Style enum, no fancy stuff.
                        //
                        //General State Information and possbilities.
                        enum State {
                            Begin,
                            End,
                            ParsingPack,
                            ParsingScriptEntry,
                            ParsingScripts,
                            ParsingDescope,
                            ParsingEntry,
                            ParsingDependancy,
                        }

                        //*************************************************//
                        //------------   These are simply guided ----------//
                        //------------   states, we only need    ----------//
                        //------------   one enum to represent   ----------//
                        //------------   this.                   ----------//
                        //*************************************************//
                        /////////////////////////////////////////////////////
                        //Our being state that can lead to other stuff :)
                        //enum Begin
                        //{
                        //    ParsingPack,
                        //    End,
                        //}

                        //Our parsing a full plugin pack state.
                        //enum ParsingPack
                        //{
                        //    ParsingScriptEntry,
                        //    ParsingScripts,
                        //    ParsingDescope,
                        //}

                        //Script entry state: One way.
                        //enum ParsingScriptEntry
                        //{
                        //    ParsingScript,
                        //}

                        //Parsing script state. For parsing single scripts.
                        //enum ParsingScript
                        //{
                        //    ParsingScript,
                        //    ParsingDescope,
                        //    ParsingScriptEntry,
                        //}

                        //All our ends and descopes, don't confuse End with descoping.
                        //end signifies end of input and that our file may no longer read.
                        //enum ParsingDescope
                        //{
                        //    ParsingDescope,
                        //    ParsingPack,
                        //    ParsingScripts,
                        //    End
                        //}
                        ///////////////////////////////////////////////////////

                        let mut FSA: State = State::Begin;

                        //NOTES:
                        //IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
                        //Preferably we package this into a simplified Finite State Machine
                        //Library for producting simple states and objects to hold these states
                        //using idomatic Rust. But for now I am just gonna use this simple
                        //match/if cases. I Should probably also create more descriptive
                        //error messages.
                        //IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
                        for i in tokens
                        {
                            match FSA{
                                State::Begin => {
                                    if p_pack.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingPack");
                                        }

                                        scope += 1;
                                        FSA = State::ParsingPack;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }

                                },
                                State::ParsingPack => {
                                    if p_ms_entry.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingScripts");
                                        }

                                        scope += 1;
                                         FSA = State::ParsingScripts;
                                    }
                                    else if p_descope.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingDescope");
                                        }

                                        scope -= 1;
                                        FSA = State::ParsingDescope;
                                    }
                                    else if p_s_entry.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingScriptEntry");
                                        }

                                         FSA = State::ParsingScriptEntry;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }

                                },

                                State::ParsingScripts => {
                                    if p_descope.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingDescope");
                                        }

                                        scope -= 1;
                                        FSA = State::ParsingDescope;
                                    }
                                    else if p_s_entry.is_match(i)
                                    {
                                        let key_capture = p_s_entry.captures(i).unwrap();
                                        current_key = key_capture.name("script_caller").unwrap().replace("\n", "").replace("\r", "").trim().to_string();

                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingScriptEntry");
                                        }

                                         FSA = State::ParsingScriptEntry;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }
                                },

                                State::ParsingScriptEntry => {
                                    if s_entry.is_match(i)
                                    {
                                        let path_capture = s_entry.captures(i).unwrap();
                                        current_path = path_capture.name("script_entry").unwrap().replace("\n", "").replace("\r", "").trim().to_string();

                                        ret_hash.insert(current_key.to_string(), current_path.to_string());

                                        //Push key value pair.
                                        if debug
                                        {
                                            println!("value being added: {}", &current_key);
                                            println!("path being added: {}", &current_path);
                                            println!("Parsing RC: Current State: ParsingEntry");
                                        }

                                        FSA = State::ParsingEntry;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }
                                },
                                State::ParsingEntry => {
                                    if p_descope.is_match(i)
                                    {
                                        if debug
                                        {
                                             println!("Parsing RC: Current State: ParsingDescope ");
                                        }

                                        scope -= 1;
                                        FSA = State::ParsingDescope;
                                    }
                                    else if s_dependancy.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingDependancy");
                                        }

                                        FSA = State::ParsingDependancy;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }
                                },
                                State::ParsingDependancy => {

                                    if p_descope.is_match(i)
                                    {
                                        if debug
                                        {
                                             println!("Parsing RC: Current State: ParsingDescope ");
                                        }

                                        scope -= 1;
                                        FSA = State::ParsingDescope;
                                    }
                                    else if p_s_entry.is_match(i)
                                    {
                                        if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingEntry");
                                        }

                                        FSA = State::ParsingEntry;
                                    }
                                    else if s_dependancy.is_match(i)
                                    {
                                         if debug
                                        {
                                            println!("Parsing RC: Current State: ParsingDependancy");
                                        }

                                        FSA = State::ParsingDependancy;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }
                                },
                                State::ParsingDescope => {
                                    if p_descope.is_match(i)
                                    {
                                        if debug
                                        {
                                             println!("Parsing RC: Current State: ParsingDescope");
                                        }

                                        scope -= 1;
                                        FSA = State::ParsingDescope;
                                    }
                                    else if p_pack.is_match(i)
                                    {
                                        if debug
                                        {
                                             println!("Parsing RC: Currennt State: ParsingPack");
                                        }

                                        scope += 1;
                                        FSA = State::ParsingPack;
                                    }
                                    else if p_ms_entry.is_match(i)
                                    {
                                        if debug
                                        {
                                             println!("Parsing RC: Current State: ParsingScripts");
                                        }

                                        scope += 1;
                                         FSA = State::ParsingScripts;
                                    }
                                    else if scope == 0
                                    {
                                        println!("Done");
                                        FSA = State::End;
                                    }
                                    else
                                    {
                                         panic!("Your RC Files syntax is corrupt.");
                                    }
                                },
                                State::End => {
                                         println!("Done");
                                }
                            }
                                //match i
                                //{
                                //    p_pack.is_match(i) => {
                                //        println!("Parsing Pack :{}", i.trim());
                                //    },
                                //    p_s_entry(i) => {
                                //        println!("Parsing Script Entry Point: {}", i.trim());
                                //    },
                                //    p_script(i) => {
                                //        println!("Parsing Scripts: {}", i.trim());
                                //    },
                                //    p_descope(i) => {
                                //        println!("Descope: {}", i.trim());
                                //    },
                                //    _ => {
                                //        println!("Error: {}", i.trim());
                                //    }
                        }
                        if debug
                        {
                            println!(" ");
                            println!("      _____PASSED RC_________");
                            println!(" ");
                        }
                    }
                },
                Err(e) => {
                     println!("Failed to open RC file: {}", e);
                }
            };
            ret_hash
        }
    }
}

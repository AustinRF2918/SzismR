/*
 * This is a portion for parsing into a hashmap
 * our parsed rc and allowing the user to call
 * pass it back to our main caller for the user
 * to do stuff with.
*/



pub mod rc_parser{

    extern crate regex;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;
    use std::process;

    pub enum ParseState{
        SOP,
        ParsePack(String),
        ParseScripts(String),
        ParseEntry(String),
        ParseScript(String),
        ParseEnd(String),
        EOP(String),
    }

    pub struct RCObject{
        tokens:  Vec<String>,
    }

    impl RCObject{
        pub fn new() -> RCObject{
                RCObject{
                tokens : Vec::new(),
            }
        }

        pub fn parse(&self, root : &PathBuf, debug : bool) -> HashMap<String, String>
        {
            //We're gonna return a new path without destroying our old root.
            let mut string_portion = &mut String::new();
            let mut ret_hash : HashMap<String, String> = HashMap::new();

            match File::open(&root){
                Ok(mut f) => {


                    f.read_to_string(string_portion).unwrap();
                    {
                        //Parse tokens by new line.
                        let tokens : Vec<&str> = string_portion.split('\n').collect();

                        let p_pack = regex::Regex::new(r"(.*)\.parsePack").unwrap();
                        let p_ms_entry = regex::Regex::new(r"(.*)\.parseScripts").unwrap();
                        let p_s_entry = regex::Regex::new(r"(?P<script_caller>.*)\^entryPoint").unwrap();
                        let p_descope = regex::Regex::new(r"(.*)End").unwrap();
                        let s_entry = regex::Regex::new(r"(?P<script_entry>.*)").unwrap();
                        let s_dependancy = regex::Regex::new(r"(.*)").unwrap();

                        let mut scope = 0;

                        let mut current_key = String::new();
                        let mut current_path = String::new();

                        let mut state: ParseState = ParseState::SOP;
                        
                        for i in tokens
                        {
                            match state
                            {
                                ParseState::SOP =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                ParseState::ParsePack(data) =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                ParseState::ParseScripts(data) =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        current_key = data;
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                ParseState::ParseEntry(data) =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        ret_hash.insert(data, current_key);
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                ParseState::ParseScript(data) =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        ret_hash.insert(data, current_key);
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                ParseState::ParseEnd(data) =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        ret_hash.insert(data, current_key);
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                ParseState::EOP(data) =>
                                {
                                    if p_pack.is_match(i)
                                    {
                                        ret_hash.insert(data, current_key);
                                        scope += 1;
                                        state = ParseState::ParsePack(i.to_string());
                                    }
                                    else
                                    {
                                        println!("Error parsing RC File");
                                        ::std::process::exit(3);
                                    }
                                },
                                _ =>
                                {
                                }
                            }
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

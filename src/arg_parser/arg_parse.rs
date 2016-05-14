/*
 * This is a portion for parsing into a hashmap
 * our parsed rc and allowing the user to call
 * pass it back to our main caller for the user
 * to do stuff with.
*/



pub mod parse{

    extern crate regex;
    use regex::Regex;
    use std::str;
    use std::slice;
    use std::env;
    use std::collections::HashSet;


    //Verb -> Noun* || Flag* || EOP -> Flag* || EOP
    pub enum ParseState{
        SOP,
        Verb(String),
        Noun(String),
        Adj(String),
        Bad(String),
        EOP,
    }

    pub struct ArgumentObject{
        //Construction automatically modifies this structure to do desired.
        current_parse_state: ParseState,

        pub verb: String,
        pub noun_dict: Vec<String>,
        pub adj_dict: Vec<String>,

        verb_set: HashSet<String>,
        noun_set: HashSet<String>,
        adj_set: HashSet<String>,

        input: Vec<String>
    }

    impl ArgumentObject{
            pub fn new() -> ArgumentObject
            {
                ArgumentObject {
                current_parse_state : ParseState::SOP,
                verb : "".to_string(),
                noun_dict : Vec::new(),
                adj_dict : Vec::new(),

                verb_set : HashSet::new(),
                noun_set : HashSet::new(),
                adj_set : HashSet::new(),

                input : Vec::new(),
                }
            }

            pub fn get_flags(&mut self) -> Vec<String>
            {
                self.adj_dict.clone()
            }

            pub fn get_nouns(&mut self) -> Vec<String>
            {
                self.noun_dict.clone()
            }

            pub fn get_verb(&mut self) -> String
            {
                self.verb.clone()
            }

            pub fn parse(&mut self, args : env::Args, verbs : Vec<&str>, nouns : Vec<&str>, adjs : Vec<&str>) {

            //Get the arguments passed in, collect it to a string vec.
            let input : Vec<String> = args.collect();

            //Set all our lists as our user defined noun/verbs.
            for i in verbs
            {
                self.verb_set.insert(i.to_string());
            }

            for i in nouns
            {
                self.noun_set.insert(i.to_string());
            }

            for i in adjs
            {
                self.adj_set.insert(i.to_string());
            }

            let mut counter = 0;

            for i in input
            {
                counter = counter + 1;
                if counter != 1
                {
                    self.current_parse_state = self.parse_iteration(i);

                    match &self.current_parse_state
                    {
                        &ParseState::Bad(ref tok) =>
                        {
                            panic!("Bad token at position: {}, Word: {}", counter, tok)
                        },
                        &ParseState::Verb(ref tok) =>
                        {
                            self.verb = tok.clone();
                        },
                        &ParseState::Noun(ref tok) =>
                        {
                            self.noun_dict.push(tok.clone());
                        },
                        &ParseState::Adj(ref tok) =>
                        {
                            self.adj_dict.push(tok.clone());
                        },
                        _ =>
                        {},
                    }
            }
        }
        }

        pub fn get_parsed(&mut self) -> (&String, &Vec<String>, &Vec<String>)
        {
            (&self.verb, &self.noun_dict, &self.adj_dict)
        }

        fn parse_iteration(&mut self, token : String) -> ParseState
        {
            match self.current_parse_state
            {
                ParseState::SOP =>
                {
                    if self.verb_set.contains(&token)
                    {
                        ParseState::Verb(token)
                    }
                    else
                    {
                        ParseState::Bad(token)
                    }
                },
                ParseState::Verb(_) =>
                {
                    if self.noun_set.contains(&token)
                    {
                        ParseState::Noun(token)
                    }
                    else if self.adj_set.contains(&token)
                    {
                        ParseState::Adj(token)
                    }
                    else
                    {
                        ParseState::Bad(token)
                    }
                },
                ParseState::Noun(_) =>
                {
                    if self.adj_set.contains(&token)
                    {
                        ParseState::Adj(token)
                    }
                    else if self.noun_set.contains(&token)
                    {
                        ParseState::Noun(token)
                    }
                    else
                    {
                        ParseState::Bad(token)
                    }
                }
                ParseState::Adj(_) =>
                {
                    if self.adj_set.contains(&token)
                    {
                        ParseState::Adj(token)
                    }
                    else
                    {
                        ParseState::Bad(token)
                    }
                }
                ParseState::EOP =>
                {
                    ParseState::EOP
                },
                ParseState::Bad(_) =>
                {
                    ParseState::Bad(token)
                }
            }

        }
    }
}


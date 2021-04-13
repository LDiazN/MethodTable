/*
    Driver program library
*/
// Rust imports
use std::io;
use std::io::Write;

// Internal imports
use crate::method_table::{ClassManager, ClassName, Function};


/// Program manager
pub struct Program {
    manager : ClassManager,
    running : bool
}

enum Action {
    Exit,
    Show(String), // Class to describe
    Create(ClassName, Option<ClassName>, Vec<Function>) // Class, parent class, and possible functions
}

#[derive(Debug)]
enum ProgramError {
    InvalidAction(String),
    NoInput,
    NotEnoughArgs,
    UnexpectedArgument(String),
    InvalidArguments
}

impl Program {

    /// Create a new program ready to run
    pub fn new() -> Program {
        Program{
            manager : ClassManager::new(),
            running : true
        }
    }

    /// Run one iteration of our program 
    pub fn run(&mut self) {
        let mut line = String::new();

		print!(">> "); // print prompt
		// flush so the print! doesn't mess up the execution order with read_line
		io::stdout().flush().expect("Couldn't flush stdout"); 

		// Read a single line
		if let Err(_) = io::stdin().read_line(&mut line) { panic!("Error leyendo input D:") }

        // if error, print error and end
        let act = match self.parse(line) {
            Err(e) => {println!("💥 [PARSE ERROR] {}", e.display()); return},
            Ok(act) => act
        };

        // perform a different operation depending on the action 
        let out = match act {
            // Exit
            Action::Exit => {self.exit(); Ok(())},

            // describe a class
            Action::Show(s) => 
                self.manager
                    .show(s.clone())
                    .and_then(|desc| Ok(println!("Description for {}:\n{}", s, desc))),

            // create a new class
            Action::Create(c, p, fs) => self.manager.add(c, p, fs)
        };
        
        match out {
            Err(e) => println!("💥 [CLASS ERROR] {}", e.display()),
            _      => {}
        };

    }

    /// Parse a string as a as a possible action 
    fn parse(&self, input : String) -> Result<Action, ProgramError> {

        // put additional spaces in : in case we have something like "A:B"
        let tokens = input.replace(":", " : ");
        let mut tokens = tokens.split_whitespace();

        // Get next action
        let action = match tokens.next() {
            None => return Err(ProgramError::NoInput),
            Some(s) => s.to_lowercase()
        };
        
        let input = tokens.collect::<Vec<&str>>();

        // Parse action
        match action.as_str() {
            "describir" => self.parse_show(input),
            "class"     => self.parse_class(input),
            "salir"     => Ok(Action::Exit),
            s           => Err( ProgramError::InvalidAction(s.to_string()) )
        }
    }

    /// Parse an action from a string vector, returning an error if not possible
    fn parse_class(&self, input : Vec<&str>) -> Result<Action, ProgramError> {
        let mut input = input.iter();

        // try to get class 
        let class = match input.next() {
            None    => return Err(ProgramError::NotEnoughArgs),
            Some(s) => s
        };

        // check if need to parse parent 
        let parent = match input.next() {
            None        => None,
            Some(&":")  => match input.next() {
                            None => return Err(ProgramError::NotEnoughArgs),
                            Some(s) => Some(s.to_string())
                        },
            Some(s)     => return Err(ProgramError::UnexpectedArgument(s.to_string()))
        };

        // Check for duplicates in functions 
        let functions = input   
                            .map(|s| s.to_string())
                            .collect::<Vec<Function>>();
        use std::collections::HashSet;
        let mut function_set = HashSet::new();

        for function in &functions {
            if function_set.contains(function) {
                return Err(ProgramError::InvalidArguments)
            }
           function_set.insert(function.clone());
        }

        // return action 
        let act = Action::Create(class.to_string(), parent, functions);
        Ok(act)
    }

    /// Parse a show action from a vector of input strings, or return an error
    /// if not possible
    fn parse_show(&self, input : Vec<&str>) -> Result<Action, ProgramError> {
        let mut input = input.iter();
        // Check if we got a class to describe
        let class = match input.next() {
            None    => return Err(ProgramError::NotEnoughArgs),
            Some(s) => s.to_string()
        };

        // Check if too many arguments
        if input.next().is_some() {
            return Err(ProgramError::InvalidArguments)
        };

        // return class
        Ok(Action::Show(class))
    }

    /// Tells if this program should run a next iteration
    pub fn should_run(&self) -> bool {
        self.running
    }

    /// mark this program to exit
    fn exit(&mut self) {
        self.running = false
    }
}

impl ProgramError {
    fn display(&self) -> String {
        format!("{:?}", self)
    }
}
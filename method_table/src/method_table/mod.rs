/*
    Main library file for our method table implementation
*/

// Rust imports
use std::collections::HashMap;

// Name for a Class
pub type ClassName = String;
// A function is just a string
pub type Function = String;
// Map from functions to class containing such function
type MethodMap = HashMap<Function, ClassName>;
// Map from Class names to Class data
type ClassMap = HashMap<ClassName, ClassData>;

#[derive(Debug, PartialEq)]
struct ClassData {
    methods : MethodMap // methods for this class
}

#[derive(Debug, PartialEq)]
pub struct ClassManager {
    classes : ClassMap // stored classes
}

#[derive(Debug, PartialEq)]
pub enum ClassError {
    NotDefined(ClassName),
    ClassRedefinition(ClassName),
    CircularDependency
}


impl ClassManager {

    /// Create a new empty class manager
    pub fn new() -> ClassManager {
        ClassManager{
            classes : ClassMap::new()
        }
    }

    /// Add a new class, maybe with a parent class, with a list of functions
    /// ## Params 
    /// * `class` - Name of the class to be added
    /// * `parent_class` - Name of parent class in case of inheritance
    /// * `methods` - List of functions defined for this class 
    /// --- 
    /// ## Return 
    /// An error if could not add this class for some reason, or nothing otherwise
    pub fn add( &mut self,
                class : ClassName,
                parent_class : Option<ClassName>, 
                methods : Vec<Function>
            ) -> Result<(), ClassError> 
    {

        let mut class_methods = MethodMap::new();

        // Check if class already exists
        if self.classes.contains_key(&class) {
            return Err(ClassError::ClassRedefinition(class))
        }

        // Check if parent class actually exists
        if parent_class.is_some() {
            let parent_class = parent_class.unwrap();

            // Check if parent class is the same class
            if class == parent_class {
                return Err(ClassError::CircularDependency)
            }

            // if does not exists, return some error
            if !self.classes.contains_key(&parent_class) {
                return Err( ClassError::NotDefined(parent_class) )
            }

            let parent_methods = &self.classes
                                        .get(&parent_class)
                                        .unwrap()
                                        .methods;

            // copy parent's class methods 
            class_methods = parent_methods.clone()
        }


        // Add specific types
        for method in methods {
            class_methods.insert(method, class.clone());
        }

        // add to manager
        let class_data = ClassData{  methods : class_methods };
        self.classes.insert(class, class_data);
        
        Ok(())
    }

    /// Return a string describing this class 
    /// ## Params 
    /// * `class` - Class to be described.
    /// --- 
    /// ## Return 
    /// An undefined error in case of non-existent class, 
    /// a string otherwise
    pub fn show(&self, class : ClassName) -> Result<String, ClassError> {
        
        let class = match self.classes.get(&class) {
            None    => return Err(ClassError::NotDefined(class)),
            Some(c) => c
        };

        let s = class.methods
                .iter()
                .map(|(method, parent_class)| 
                    format!("\t{} -> {} :: {}\n", 
                            method, 
                            parent_class, 
                            method
                        )
                )
                .collect::<Vec<String>>()
                .join("");

        Ok(s)
    }
}

impl ClassError {
    pub fn display(&self) -> String {
        format!("{:?}",  self)
    }
}
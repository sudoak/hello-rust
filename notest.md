* cargo is the dependancy manager (check, run, build, --release, update)
* for Result, the variants are Ok,Err
* a match expression is made up of arms
* use shadowing to overwrite existing values - especially for conversion
* loop {} creates infinite loop
* const cant change or assign values at run time
* mut (can change, but not type) vs shadowing (always reassign using let, can change type) 
* data types can be scalar (int, float ,bool, char) and compound (tuples & arrays)
* character can be unicode too
* tuple fixed length once defined
* statement (it doest return ) vs expression (it returns something , it does not include semiclolons )
* create clock using { }
* if can be used in let statement
* looping (loop, while, for array.iter())
* rev() -> reverse
* memory is managed through system of ownership
* drop() -> this drops off vars out of scope
* double free error (corrupts memory and security vulneribilities)
* move -> moves pointer from one to another -> deep copies
* str1.clone(), str1.len()
* copy happens for scalar types (cos they live in stack)
* 
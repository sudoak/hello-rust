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
* str1.clone(), str1.len(), str1.clear()
* copy happens for scalar types (cos they live in stack)
* we can only have one reference to mutable data apart from owner (&mut) (race condition is avoided)
* one mutatble reference or many immutable refs
* dangle (need lifetime parameter in main)
* .iter.enumerate()
* slices
* struct -> fields
* .. used as spread operator - if already not mentioned
* tuple struct
* String (can grow) vs str (can not and is immutable)
* primitive methods implement Display, Debug by default but not struct
* Debug is a triat and derive is an annotation
* {:?} vs {:#?}
* 
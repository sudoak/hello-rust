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
* str1.clone(), str1.len(), str1.clear(), str1.as_str()
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
* Associate fucntions - which dont take self as parameter - used to return new instance of struct
* struct can have many impementations
* in enum any type of kind of data can be put
* Rust does not have null value - but has Option<T> with Some(T) and None
* <T> it means it holds some type of data 
* Some holds all the values
* But in None u nedd to specify which data type u refering too
* Option<T> { Some(T), None}
* enum can contain struct as well as enum
* combining match and enums is useful
* _ is placeholder and used for exhautive coverage
* if let syntax
* if let Some(3) = some_u8_value { println!("lol")} else { println!("ha ha")}
* can control which is in scope and which is not
* Packages, crates, modules, paths
* A project can have multiple crates but one library
* Absolute path vs relative path
* we can aslo construct relative path of parent module using super super::serve_order()
* if struct has made public, the each invidual fields and impl should be made public
* if enum is made public all of its variants are made public
* we can use  ****use**** to make mods come in scope
* self can cause inconsistecy so dont use in future
* ****as**** can be used for a proper idiomatic way to name a scope
* ****pub**** ****use**** can reimport mods and make it pub for others too
* std is also external but ships with rust. still we need to explicitly mention it
* we can import nested path use std::io::{self, Write}
* vectors can only store values of one type Vec<T> . for can be used to iterate through Vec values
* vec.push(), vec.get(), vec.pop()
* to_string() creates a String from string literal
* push_str()
* + can be sused for string concantenation s1 + &s2
* format!("{}-{}-{}", s1, s2, s3) (it does not take ownership as above)
* Rust strings do not support indexing
* grapheme clusters
* for i in string.chars() {} || string.bytes()
* if we create hashmap from existing vars, it will take away their ownership
* hashmap.insert() will insert also override
* hashmape.entry().or_insert() //checks if exists, if not then updates
* Errors -> Recoverable vs Unrecoverable (Result<T, E>)
* unrecoverable, use panic! macro it unwinds all
* can use panic = 'abort' // without cleaning up, just exiting code
* buffer overread (reading from index which is not present in an array)
* RUST_BACTRACE=1 cargo run // to search a level up
* can match over error.kin() 
* unwrap_or_else(|error|{})
* panic on error -> expect and unwrap 
* ? operator, propogates error (can only be used in fn which returns result)
* *******fs::read_to_string("hello.txt")*******
* if wanna use ****?**** operator in main.rs use Result<(), Box<dyn Error>> //it means any error
* generics are abstract standins for concrete types (Vec<T>, Option etc)
* A trait tells the Rust compiler about functionality a particular type has and can share with other types
* we cant implement external traits of external types
* trait as parameters
* trait bound syntax <T: Display + Copy>
* returning types that implement traits
* heap collections are slow if grew in large amount
* rust moves selected errors from runtime to compile time to improve performence
* preventing dangling references with lifetimes (Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions)
* rust has borrow checkers to compare scopes
* lifetime annotation syntax (explicit lifetime)
  * &i32        // a reference
  * &'a i32     // a reference with an explicit lifetime
  * &'a mut i32 // a mutable reference with an explicit lifetime
* lifetime elision rules (repeated addition of new rules)
* static lifetime -> saves in program binary and always avalilable 
* cargo has two profiles dev and release
* opt-level to control compile time
* cargo doc to generate html, md based documentation in target/doc
* create a public API fro your crate by using reimports pub use self::something
* cargo login token
* cargo publish, 
* cargo yank --vers 1.0.1 --undo
* Box<T> it allows to store data in heap rather in stack Box::new(5)
* in recursive type the memory cant be know at compile time
* cons list (two arguments always)
* concurrency issues - Race, Deadlocks, bugs
* implementation of threads 1:1 (OS API), M:N (green threads)
* thread::spawn to start a new thread & handle.join().unwrap() to wait till its executed completely
* 
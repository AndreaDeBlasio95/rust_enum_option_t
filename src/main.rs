// The Option Enum and Its Advantages over Null Values
// Is another Enum definition in the standard library
// The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.
// Rust doesn't have the null, but have an enum that can encode the concept of a value being present or absent -> Option<T>
/*
enum Option<T>{
    Some(T),
    None,
}
*/

fn main() {
    println!("Hello, world!");

    let some_number = Some(5);
    let some_string = Some("a string");

    // If we use None rather than Some, we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.
    let absent_number: Option<i32> = None;

    // The compiler won’t let us use an Option<T> value as if it were definitely a valid value, this is not allowed:
    /*
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    */
    // Because we are trying to add an i8 with an Option<i8>
    // Rust force us to convert an Option<T> to a T before you can perform T operations with it, this helps catch one of the most com- mon issues with null.
    /* RECAP: 
    In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
    */
    /*
    In order to use an Option<T> value, you want to have code that will handle each variant, the match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
    */
    // The match control Flow Operator
    // The match keyword allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

}

// An enum and a match expression that has the variants of the enum as its patterns
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // match arms have two parts: a pattern and some code
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

// Patterns that Bind to Values -> This is how we can extract values out of enum variants.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// The _ Placeholder ->    The _ pattern will match any value that aren’t specified before it
/*
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // The () is just the unit value, so nothing will happen here.
}
*/

// Concise Control Flow with if let
// The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
/*  
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), 
    }
    We can use this:
    if let Some(3) = some_u8_value {
        println!("three");
    }
*/

/*
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
Or we could use an if let and else expression like this:
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1; 
    }
*/
fn main() {
    
    // Variable bindings are immutable by default
    // but this can be overridden using the mut modifier
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    //_immutable_binding += 1;

    // Variable bindings have a scope and are constrained to live in a block
    // A block is a collection of statements enclosed by braces {}

    // This binding lives in the main function
    let long_lived_binding = 1;

    // this is a block and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of block

    // Error! short_lived_binding does not exist in this scope
    //println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // Also variable shadowing is allowed
    let shadowed_binding = 1;

    {
        println!("Before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("Shadowed in inner block: {}", shadowed_binding);
    }
    println!("Outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("Shadowed in outer block: {}", shadowed_binding);

    // It's possible to declare variable bindings first and initalize them later
    // However, this form is seldom used as it may lead to uninitialized variables
    let a_binding;

    {
        let x = 2;
        
        // Initialize the binding
        a_binding = x * x;
    }

    println!("A binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    //println!("Another binding: {}", another_binding);

    another_binding = 1;

    println!("Another binding: {}", another_binding);
    // The compiler forbids use of uninitialized variables
    // as this would lead to undefined behavior

    // When data is bound by the same name immutable it also freezes
    // Frozen data can't be modified until the immutable binding goes out of scope
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable _mutable_integer
        let _mutable_integer = _mutable_integer;

        // Error! _mutable_integer is frozen in this scope
        //_mutable_integer = 50;

        // _mutable_integer goes out of scope
    }

    // Okay! _mutable_integer is not frozen in this scope
    _mutable_integer = 3;

}
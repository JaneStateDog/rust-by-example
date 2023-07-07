#![allow(unreachable_code)]
// ^ We use this for the funny unreachable sections of loops in this example
#![allow(dead_code)]

use std::str::FromStr;

fn main() {

    // if-else statements don't need the boolean condition to be
    // surrounded by parentheses
    // Each condition is followed by a block
    // if-else conditionals are expressions, and, all branches must return
    // the same type(???)
    // Ohhh it says that because if you do something like this
    let _big_n = 
        if 5 < 2 {
            10 * 3
        } else if 7 > 3 {
            4
        } else {
            9
        };

    // Rust provides a loop keyword to indicate an infinite loop
    // The break statement can be used to exit a loop at anytime,
    // whereas the continue statement can be used to skip the rest of the iteration 
    // and start a new one
    let mut count = 0u32;
    loop {
        count += 1;

        if count == 1 {
            println!("Haha, one!")
        }

        println!("{}", count);

        if count == 5 {
            println!("I think we get the point");
            break;
        }
    }

    // It's possible to break or continue outer loops when dealing with nested loops
    // In these cases, the loops must be annotated with some 'label
    // and the label must be passed to the break/continue statement
    'outer: loop {
        println!("Entered the outer loop");

        '_inner: loop { // I have this label underscored because we never use it
            println!("Entered the inner loop");
            
            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // One of the uses of a loop is to retry an operation until it succeeds
    // If the operation returns a value though, you might need to pass it to the rest of the code
    // So if you put it after the break and it will be returned by the loop expression
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // The while keyword can be used to run a loop while a condition is true
    // Let's write the infamous FizzBuzz (hey I know this!) using a while loop
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    // for and range

    // The for in construct can be used to iterate through an Iterator
    // One of the easiest ways to create an iterator is to use the range notation a..b
    // This yields values from a (inclusive) to b (exclusive) in steps of one

    // Let's write FizzBuzz using for instead of while

    // n will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, a..=b can be used for a range that is inclusive on both ends
    // The above can be written as

    // n will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // for and iterators

    // The for in construct is able to interact with an Iterator in several ways
    // As discussed on the section on the Iterator trait (which we have not gotten to yet,)
    // by default, the for loop will apply the into_iter function to the collection
    // However, this is not the only means of coverting collections into iterators

    // into_iter, iter, and iter_mut all handle the conversion of a collection into
    // an iterator in different ways, by providing different views on the data within

    // iter - This borrows each element of the collection through each iteration
    // Thus leaving the collection untouched and available for reuse after the loop
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter - This consumes the collection so that on each iteration
    // the exact data is provided
    // Once the collection has been consumed it is no longer
    // available for reuse as it has been "moved" within the loop
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    //println!("names: {:?}", names);
    // FIXME ^ Comment out this line

    // iter_mut - This mutably borrows each element of the collection
    // allowing for the collection to be modified in place
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    // In the above snippets note the type of match branch,
    // that is the key difference in the types of iteration
    // The difference in type then of course implies differing
    // actions that are able to be performed

    // Rust provides pattern matching via the match keyword, which can be used like a C switch
    // The first matching arm is evaluated and all possible values must be covered
    
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("Haha, one!"),
        // Match several values
        2 | 3 | 5| 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't nothin' special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);

    // A match block can destructure items in a variety of ways

    // Tuples
    match (0, -2, 3) {
        (0, y, z) => println!("0 then {} then {}", y, z),
        (1, ..) => println!("First is 1 and the rest who knows"),
        (.., 2) => println!("Last is 2 and who cares about the rest"),
        (3, .., 4) => println!("First is 3, last is 4, what's the middle? I don't care"),
        _ => println!("Who cares as usual"),
    }

    // Arrays/slices
    // Most of this is the same as tuples so I won't type it out
    match [1, -2, 6] {
        // You can store them in another array/slice
        // The type depends on that of the value that is being matched against
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example,
        // bind the first and last values and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    // I'm gonna copy paste the one for enums because I don't wanna keep typing all this

    // `allow` required to silence warnings because only
    // one variant is used.
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

    // For pointers, a distinction needs to be made between
    // destructuring and dereferencing as they are different concepts
    // which are used differently from languages like C/C++
    // - Dereferencing uses *
    // - Destructuring uses &, ref, and ref mut

    // Assign a reference of type i32
    // The & signifies there is a reference being assigned
    let reference = &4;

    match reference {
        // If reference is pattern matched against &val,
        // it results in a comparison like:
        // &i32
        // &val
        // ^ We see that if the matching "&"s are dropped, then the i32
        // should be assigned to val
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the &, you dereference before matching
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference?
    // reference was a & because the right side was already a reference
    // This is not a reference because the right side is not one
    let _not_a_reference = 3;

    // Rust provides ref for exactly this purpose
    // It modifies the assignment so that a reference is created for the element;
    // this reference is assigned
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references,
    // references can be retrieved via ref and ref mut
    let value = 5;
    let mut mut_value = 6;

    // Use ref keyword to create a reference
    match value {
        ref r => println!("Got a reference to a value {:?}", r),
    }

    // Use ref mut similarly
    println!("mut_value is: {:?}", mut_value);
    match mut_value {
        ref mut m => {
            // Got a reference
            // Gotta dereference it before we can add anything to it
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
        },
    }
    println!("mut_value is now: {:?}", mut_value);

    // Same as enum, I'm just going to copy paste struct
    // Me too lazy

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    // A match guard can be added to filter the arm

    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    // Note that the compiler won't take guard conditions into account
    // when checking if all patterns are covered by the match expression
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }

    // Ahh why is flow of control so longggg

    // Binding
    // Indirectly accessing a variable makes it impossible to branch
    // and use that variable without re-binding
    // match provides the @ sigil for binding values to names

    fn age() -> u32{
        15
    }

    println!("Tell me what type of person you are!");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could match 1..=12 directly but then what age
        // would the child be? Instead, bind to n for the
        // sequence of 1..=12
        // Now the age can be reported
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound, so just return the result
        n => println!("I'm an old person of age {:?}", n),
    }

    // You can also use binding to "destructure" enum variants, such as Option
    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        // Got "Some" variant, match if its value, bound to n, is equal to 42
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (None variant)
        _ => (),
    }

    // FINALLY we're past match lmao
    // if let

    // For some use cases, when matching enums, match is awkward
    // For example (oh god more matching:)
    /* 
    // Make `optional` of type `Option<i32>`
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just so we could destructure
            // `i` from the option.
        },
        _ => {},
        // ^ Required because `match` is exhaustive. Doesn't it seem
        // like wasted space?
    };
    */

    // if let is cleaner for this use case and in addition allows various
    // failure options to be specified
    
    // All have type Option<i32>
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The if let construct reads:
    // if let destructures number into Some(i), evaluate the block
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case
        println!("Didn't match a number. Let's go with a letter!");
    }

    // If you need to specify a failure, use an else
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an else if condition to see if the
    // alternate failure branch should be taken
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default
        println!("I don't lile letters. Let's go with an emoticon :)!");
    }

    // In the same way, if let can be used to match an enum value
    enum Foo2 {
        Bar,
        Baz,
        Qux(u32),
    }

    // Create example variables
    let a = Foo2::Bar;
    let b = Foo2::Baz;
    let c = Foo2::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo2::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo2::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo2::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with if let
    if let Foo2::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }

    // Another benefit is that if let allows us to match non-parameterized enum variants
    // This is true even in cases where the enum doesn't implement or derive PartialEq
    // In such cases if Foo::Bar == a would fail to compile, because instances of the enum cannot
    // be equated, however if let will continue to work

    // With let-else a refutable pattern can match and bind variables in the surrounding
    // scope like a normal let, or else diverge (e.g. break, return, panic!) when the pattern doesn't match

    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split("");
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    // I have no hecking idea what any of this does

    // The scope of name bindings is the main thing that makes this different from match or if let-else expressions
    // You could previously approximate these patterns with an unfortunate bit of repetition and an outer let

    // THIS IS THE LAST BIT OF THIS SECTION, FINALLY

    // Similar to if let, while let can make awkward match sequences more tolerable
    // Consider the following sequence that increments i:
    /*
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // Repeatedly try this test.
    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            },
            // Quit the loop when the destructure fails:
            _ => { break; }
            // ^ Why should this be required? There must be a better way!
        }
    }
    */
    // Using while let makes this sequence much nicer

    // Make optional of type Option<i32>
    let mut optional = Some(0);

    // This reads: "while let destructures optional into Some(i), evaluate the block, elsle break"
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {:?}. Try again", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require explicitly handling the failing case
    } 
    // ^ if let had additional options else/else if clauses
    // while let does not have these

    // We just finished section 8 of 24 in Rust By Example
    // I just. I just want to code. What the hell do I do now?
    // Do I just go try and make something and skim through Rust By Example as I do so?
    // I guess I'll just do that

}
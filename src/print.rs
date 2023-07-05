fn main() {
    let x = 5 + /*90 + */ 5;
    let y = 21;
    
    println!("{0}, test, {1}, test2, {0}", x, y);
    println!("{first_thing}, testing, {second_thing}", 
        first_thing=x, 
        second_thing=y
    );
    
    // Base 10, then binary, then hexadecimal
    println!("{}", x + y);
    println!("{:b}", x + y);
    println!("{:x}", x + y);
    
    // Right-justify text by 4 white spaces (4 + 1 for the number = 5)
    println!("{number:>5}", number=y);
    
    // Pad numbers with extra 0s
    println!("{number:0<5}", number=y);
    
    // You can even do this if you use $
    println!("{number:0>width$}", number=y, width=x);
    
    // Only types that implement fmt::Display can be formatted wiht '{}'
    // User-defined types do not imeplent this by default
    
    // Also apparently you can do this
    // Where you capture a surrounding variable
    let number: f64 = 1.0;
    println!("{number}"); // Crazy
    
    // Thingy it is telling me to do
    let pi = 3.141592;
    println!("{0:.2}", pi); // Prints "3.14"
}
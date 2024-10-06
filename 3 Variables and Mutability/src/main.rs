fn main() {
    // CONST are declared at compiletime
    // Can be set to a "constant" expression, which can guaranteed be evaluated ar compile time.
    // By that is meant that values can be used, which will not first be evaluated at runtime and therefore "may" not be present.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const value is: {THREE_HOURS_IN_SECONDS}.");
    // LET or immutable | mutable values are declared at runtime
    let mut x = 5;
    println!("The value of x is: {x}.");
    x = 0;
    println!("The value of x is: {x}.");

    shadowing_variables();
}

/// # Summary
/// you can declare a new variable with the same name as a previous variable.
/// Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.
/// In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
fn shadowing_variables() {
    let x = 5;

    let x = x + 1; // x is now 6 (i32)

    {
        let x = String::from("Hello"); // x is now a String in this inner scope
        println!("The value of x in the inner scope is: {x}");
    }

    // After the inner scope ends, we are back to the previous x (which is 6)
    println!("The value of x is: {x}");
}

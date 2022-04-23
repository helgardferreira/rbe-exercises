// `allow` required to silence warnings because only one variant is used.
#[allow(dead_code)]
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

struct StructEx {
    x: (u32, u32),
    y: u32,
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    /*
    Rust provides pattern matching via the match keyword, which can be used like
    a C switch. The first matching arm is evaluated and all possible values must
    be covered.
    */
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest fo cases
        _ => println!("Ain't special"),
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

    /*
    A match block can destructure items in a variety of ways.

    Tuples can be destructured in a match as follows:
    */
    let triple = (10, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) =>
            println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
        // (x, y, z) =>
        //     println!("Default case: ( {}, {}, {} )", x, y, z),
    }

    /*
    Like tuples, arrays and slices can be destructured this way:
    */
    // Try changing the values in the array, or make it a slice!
    let array = [4, -2, 6];

    match array {
// Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // You can match against any part in the array
        [_, -2, ..] =>
            println!("all that matters is that the second element is -2"),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    /*
    An enum is destructured similarly:
    */
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
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

    /*
    For pointers, a distinction needs to be made between destructuring and
    dereferencing as they are different concepts which are used differently
    from languages like C/C++.

    Dereferencing uses *
    Destructuring uses &, ref, and ref mut
    */
    // Assign a reference of type `i32`. The `&` signifies there is a reference
    // being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results in a
        // comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not a reference
    // because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the assignment
    // so that a reference is created for the element; this reference is
    // assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references can be
    // retrieved via `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can add anything
            // to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    /*
    Similarly, a struct can be destructured as shown:
    */
    // Try changing the values in the struct to see what happens
    let foo = StructEx { x: (1, 2), y: 3 };

    match foo {
        StructEx { x: (1, b), y } =>
            println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        StructEx { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        StructEx { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //StructEx { y } => println!("y = {}", y),
    }

    /*
    A match guard can be added to filter the arm.
    */
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    /*
    Note that the compiler does not check arbitrary expressions for whether all
    possible conditions have been checked. Therefore, you must use the _ pattern
    at the end.
    */
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // This should not be possible to reach
    }

    /*
    Indirectly accessing a variable makes it impossible to branch and use that
    variable without re-binding. match provides the @ sigil for binding values
    to names:
    */

    let age = || -> u32 {
        15
    };

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    /*
    You can also use binding to "destructure" enum variants, such as Option:
    */
    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        // Got `Some` Variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n) if n == 42 => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => ()
    }

    /*
    For some use cases, when matching enums, match is awkward. For example:
    */
    // Make `optional` of type `Option<i32>`
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just so we could destructure
            // `i` from the option
        }
        _ => {}
        // ^ Required because `match` is exhaustive.
        // Doesn't it seem like wasted space?
    }

    /*
    if let is cleaner for this use case and in addition allows various failure
    options to be specified:
    */
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated to false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    /*
    In the same way, if let can be used to match any enum value:
    */
    // Create some variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred, c = {}", value);
    }

    /*
    Another benefit is that if let allows us to match non-parameterized enum
    variants. This is true even in cases where the enum doesn't implement or
    derive PartialEq. In such cases if Foo::Bar == a would fail to compile,
    because instances of the enum cannot be equated, however if let will
    continue to work.

    Would you like a challenge? Fix the following example to use if let:
    */
    // Variable a matches Foo::Bar
    // if Foo::Bar == a {
    if let Foo::Bar = a {
        // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }

    /*
    Similar to if let, while let can make awkward match sequences more
    tolerable. Consider the following sequence that increments i:
    */
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // Repeatedly try this test.
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            }
            // Quit the loop when the destructure fails:
            _ => { break; }
            // ^ Why should this be required? There must be a better way!
        }
    }

    /*
    Using while let makes this sequence much nicer:
    */
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}

fn main() {
    /*
    Branching with if-else is similar to other languages. Unlike many of them,
    the boolean condition doesn't need to be surrounded by parentheses, and
    each condition is followed by a block. if-else conditionals are expressions,
    and, all branches must return the same type.
    */
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n)
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, incrase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    // Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    /*
    Rust provides a loop keyword to indicate an infinite loop.

    The break statement can be used to exit a loop at anytime, whereas the
    continue statement can be used to skip the rest of the iteration and start
    a new one.
    */
    let mut count = 0u32;

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");

            // Exit this loop
            break;
        }
    }

    /*
    It's possible to break or continue outer loops when dealing with nested
    loops. In these cases, the loops must be annotated with some 'label, and
    the label must be passed to the break/continue statement.
    */
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    /*
    One of the uses of a loop is to retry an operation until it succeeds. If
    the operation returns a value though, you might need to pass it to the rest
    of the code: put it after the break, and it will be returned by the loop
    expression.
    */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    /*
    The while keyword can be used to run a loop while a condition is true.

    Let's write the infamous FizzBuzz using a while loop.
    */
    let fizz_buzz_it = |n: i32| {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    };

    let mut n = 1;

    while n < 101 {
        fizz_buzz_it(n);

        // Increment counter
        n += 1;
    }

    /*
    The for in construct can be used to iterate through an Iterator. One of the
    easiest ways to create an iterator is to use the range notation a..b. This
    yields values from a (inclusive) to b (exclusive) in steps of one.

    Let's write FizzBuzz using for instead of while.
    */
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        fizz_buzz_it(n);
    }

    /*
    Alternatively, a..=b can be used for a range that is inclusive on both ends.
    The above can be written as:
    */
    for n in 1..=100 {
        fizz_buzz_it(n);
    }

    /*
    The for in construct is able to interact with an Iterator in several ways.
    As discussed in the section on the Iterator trait, by default the for loop
    will apply the into_iter function to the collection. However, this is not
    the only means of converting collections into iterators.

    into_iter, iter and iter_mut all handle the conversion of a collection into
    an iterator in different ways, by providing different views on the data
    within.

    `iter` - This borrows each element of the collection through each
    iteration. Thus leaving the collection untouched and available for reuse
    after the loop.
    */
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    /*
    into_iter - This consumes the collection so that on each iteration the exact
    data is provided. Once the collection has been consumed it is no longer
    available for reuse as it has been 'moved' within the loop.
    */
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {:?}", name),
        }
    }

    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line
    // error[E0382]: borrow of moved value: `names`

    /*
    iter_mut - This mutably borrows each element of the collection, allowing for
    the collection to be modified in place.
    */
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello"
        }
    }

    println!("names: {:?}", names);
    /*
    In the above snippets note the type of match branch, that is the key
    difference in the types of iteration. The difference in type then of course
    implies differing actions that are able to be performed.
    */
}

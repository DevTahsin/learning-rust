# Common programming concepts in Rust

## Variables and mutability
- mutability is can change value
- immutability is can't change value

By default, variables are immutable(can't change the value).
```
let x = 5; // immutable 
```
if the immutable value has a new assign after first assign compiler catch that and throw error at compile time.
```
let mut x = 5; // immutable
```

## Constants
Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

- `mut` keyword does not allowed to use with constants
- can be declared in any scope, including the global scope.
- may be set only to a constant expression, not the result of a value that could only be computed at runtime. 
```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
rust's naming convention for constants is to use all Uppercase and underscore between words.

this expressions result will assigned in compile time. if its variable it will be in run time.

## Shadowing
Shadowing means you can declare a new variable with the same name as a previous variable.
```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```
this code will compiled and does not throw error at runtime.

it's like mut right? but actually its not.
if we will marking variable with `mut` we'll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.

the other difference between `mut` and shadowing is that because we're effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. 

For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

shadowing style
```
   let spaces = "   ";
   let spaces = spaces.len();
```
mutable style
```
    let mut spaces = "   ";
    spaces = spaces.len();
```
if we try to use `mut` for this, as shown here, we'll get a compile-time error.
Because variable's types are does not match.

## Data Types
we'll look at two data type subsets; scalar and compound.

### Scalar types
A scalar type represents a single value.

Rust has four primary scalar types: integers, floating-point numbers, booleans and characters.

#### integer types
- unsigned (must be positive)
- signed (can be negative or positive)

signed integer types starts with `i` unsigned integer types starts with `u`


| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

the `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as "arch": 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.

You can write integer literals in any of the forms show in below table.

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

INTEGER OVERFLOW!

Let's say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you're compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error.

When you're compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two's complement wrapping. In short, values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won't panic, but the variable will have a value that probably isn't what you were expecting it to have. Relying on integer overflow's wrapping behavior is considered an error.

#### floating-point types
Rust's floating-point types are `f32` and `f64`.
f32 is 32 bit f64 is 64 bit. by default its `f64`.
```
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

#### Numeric operations
- `+` addition
- `-` substraction
- `*` multiplication
- `/` division
- `%` remainder

#### The boolean type
it can be `true` or `false`. keyword is `bool`
```
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

#### The Character type
```
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```
we specify `char` literals with single quotes(`'`). char types are 4 bytes.


### Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 

Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

We can also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:
```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
The tuple without any values has a special name, `unit`. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don't return any other value.

### The Array Type
- arrays in Rust have a fixed length. 
- every element of an array must have the same type.
- if you want dynamic length array you must use vectors.
- syntax of definition is `let variable_name: [datatype; length_of_array] = [elements_of_array ]`

```
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
Arrays are useful when you want your data allocated on the stack rather than the heap.

An array isn't as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

You write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
```
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```
let a = [3; 5];
```
means 5 elements that will al be set to the value 3 initially. This is the same as writing `let a = [3, 3, 3, 3, 3];`

Accessing array elements
```
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime, especially in this case, because the compiler can't possibly know what value a user will enter when they run the code later.

## Functions
- declared with `fn` keyword.
- you must declare the type of each parameter.
- function is an expression.

We define a function in Rust by entering fn followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.

```
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### Parameters
```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```
### Statements and Expressions
Rust is an expression-based language. 
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let's look at some examples.

We've actually already used statements and expressions. Creating a variable and assigning a value to it with the let keyword is a statement. `let y = 6;` is a statement.
```
fn main() {
    let y = 6;
}
```
Function definitions are also statements; the entire preceding example is a statement in itself.

Statements do not return values. Therefore, you can't assign a let statement to another variable, as the following code tries to do; you'll get an error.
```
fn main() {
    let x = (let y = 6);
}
```
Expressions evaluate to a value and make up most of the rest of the code that you'll write in Rust. Consider a math operation, such as `5 + 6`, which is an expression that evaluates to the value `11`.

- Calling a function is an expression.
- Calling a macro is an expression.
- A new scope block created with curly brackets is an expression, for example:
```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Expressions do not include ending semicolons.

### functions with return value
- there is no return keyword
- because all functions are expression
- if you return a value do not put semicolon(`;`)

We don't name return values, but we must declare their type after an arrow (->). 

```
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. 

With parameters:
```
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## Comments
Comments starts with two slash `//`

If multiple line you must put `//` to each line

## Control Flow

### `if` expressions

- Conditions must be bool
- 

Common mean of if statement branching.

```
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

### Using `if` in a `let` Statement

```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

### Repetition with Loops
- there is three kind of loop
- `loop`
- `while`
- `for`


- `break` for exit from loop
- `continue` for continue to next step of loop

- `break` can return value
- `break` or `continue` also breaks or continues specific loops with loop labels like `'counting_up`

#### `loop`
The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```
fn main() {
    loop {
        println!("again!");
    }
}
```

##### Returning Values from Loops

with break keyword you can return value like

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

#### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 

```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first break that doesn't specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop.

#### `while`

its conditional loop. While a condition evaluates to true, the code runs; otherwise, it exits the loop.

```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

#### `for`

You can choose to use the while construct to loop over the elements of a collection, such as an array.

```
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

However, this approach is error prone; we could cause the program to panic if the index value or test condition is incorrect. For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It's also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection.

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

we can use with range expression for definition `for` loop as well

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
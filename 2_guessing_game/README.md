# Programming a guessing game
You'll learn about let, match, methods, associated functions, external crates, and more! In the following chapters, we'll explore these ideas in more detail. In this chapter, you'll just practice the fundamentals.

## syntax keywords

- `use`     importing the dependency
- `fn`      declares new function
- `let`     creates new variable. when directly use of let like `let apple = 5` the apple variable won't change because default behavior of definition of variable with let is `immutable`.
- `mut`     means mutable. `let mut apple = 5` the apple variable can be change.
- `//` comment sign
- `match` is like switch case statement handler.
- `loop` is infinite loop scope. in c# while(true)


## about program.

``let mut guess = String::new();`` code means create a mutable variable guess which is a new instance of String.

```
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
```

means io dependency's stdin function called with read_line function for user input handling form standart input. the input was directly assigned to guess variable with &mut.

`&` operator is indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

variables and references are immutable by default.

`read_line` method is also return `Result` value. Result is an `enumaration`, `Result's` variants are `Ok` and `Err`. Ok is the successfully generated value. The `Err` variant means the operation failed and `Err` contains information about how or why the operation failed.

`Result` is has an `expect` method that you can call. if this instance `Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`.

If you don't call `expect`, the program will compile, but you'll get a warning.

`.expect("Failed to read line");` that means if read_line is completed with Err program prompts Failed to read line.

`println!("You guessed {guess})` the `{}` brackets is a placeholder: hold a value in a place. 
```let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```
For an example. {x} stands for x variable placeholder and {} empty brackets stands for second argument.


## Project need random
### adding dependency
So we need to add rand dependency. 
When adding new dependency to project we must tell cargo the dependency name and version.
- Open Cargo.toml file
- Under the dependencies add `rand = "0.8.5"`
- build project with cargo build

### implementation of random
- `use rand::Rng` rand is dependency and Rng `trait` defines methods that random number generators implement,  and this trait must be in scope for us to use those methods.
- trait means interface
- crate means compilable unit (in c# it means assembly)
- `let secret_number = rand::thread_rng().gen_range(1..=100);`
thread_rng function that gives us the particular random number generator we're going to use: one that is local to the currernt thread of execution and is seeded by the operating system. Then we call the `gen_range` method on the random number generator. `gen_range` method takes a `range expression` as an argument and generates a random number in the range. `start..=end` is `1..=100` means between 1 and 100.


## is guessed number and secret number match?

### we use cmp trait for comparing values
Ordering is a enum that has 
- Less
- Greater
- Equal 

```
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

guess is String secret_number is a int32. we must convert guess variable to int32.

```
let guess: u32 = guess.trim().parse().expect("Please type a number");
```
tricky part is we create a variable named guess but we already have a variable named guess right? 
Rust allows us to shadow the previous value of guess with a new one.
Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variable, such as `guess_str` and `guess`.

guess.trim is eliminate whitespace from begging and end. the parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. we need to tell Rust the exact number type we want by using `let guess: u32` the colon (`:`) . 

### we use loop keyword for creating infinite loop
```
loop{
    // infinite scope
}
```

loop will create infinite running scope till matching error or break;

we moved input and comparision code to this scope. and changed input handling are with
```
 let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };
```

if parse has an error it will continue the loop not breaking program.

and also if user guess correct answer loop will break and program ended with success
```
match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
```

fn main() {
    let array_of_lines = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five goldenen rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping"
    ];
    let array_of_numerations = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth"
    ];

    for index in 0..(array_of_numerations.len()) {
        let numeration = array_of_numerations[index];
        println!("On the {numeration} day of Christmas");
        println!("My true love brought to me");

        for line_index in (0..(index)).rev() {
            let line = array_of_lines[line_index];
            println!("{line}");
        }

        if index == 0{
            println!("A partridge in a pear tree");
        } else {
            println!("And a partridge in a pear tree");
        }
        println!("");
    }
    
}


// On the first day of Christmas
// My true love brought to me
// A partridge in a pear tree

// On the second day of Christmas
// My true love brought to me
// Two turtle doves
// And a partridge in a pear tree

// On the third day of Christmas
// My true love brought to me
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the fourth day of Christmas
// My true love brought to me
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the fifth day of Christmas
// My true love brought to me
// Five goldenen rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the sixth day of Christmas
// My true love brought to me
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the seventh day of Christmas
// My true love brought to me
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the eighth day of Christmas
// My true love brought to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree

// On the ninth day of Christmas
// My true love brought to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three French hens
// Two turtle doves
// And a partridge in a pear tree
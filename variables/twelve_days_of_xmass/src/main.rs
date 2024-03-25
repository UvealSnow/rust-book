fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut previous_gifts = String::new();

    // Time: O(n), Space O(n)
    for day in 0..12 {
        let gifts = match day {
            0 => format!("{}.\n", gifts[day]),
            1 => format!("{},\nAnd {}.\n", gifts[day], gifts[0].to_lowercase()),
            _ => format!("{},\n{}", gifts[day], previous_gifts).to_string(),
        };

        println!("On the {} day of Christmas,\nmy true love gave to me:\n{}", days[day], gifts);
        
        if day > 0 {
            previous_gifts = gifts;
        }
    }
}

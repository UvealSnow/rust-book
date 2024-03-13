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

    // Don't use 0..=12 or it will go oob
    for day in 0..12 {
        let mut lyrics = format!("On the {} day of Christmas,\nmy true love gave to me:\n", days[day]);
        match day {
            0 => lyrics.push_str(&format!("{}.", gifts[0])),
            _ => {
                for n in (0..=day).rev() {
                    match n == 0 {
                        true => lyrics.push_str(&format!("And {}.", gifts[n].to_lowercase())),
                        false => lyrics.push_str(&format!("{},\n", gifts[n])),
                    }
                }
            }
        }
        println!("{lyrics}\n");
    }
}

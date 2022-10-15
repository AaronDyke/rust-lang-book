const TWELVE_DAYS_OF_CHRISTMAS: [(&str, &str); 12] = [
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtledoves"),
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five gold rings (five golden rings)"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eighth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Eleven pipers piping"),
    ("twelfth", "Twelve drummers drumming"),
];

fn main() {
    for (i, day_of_christmas) in TWELVE_DAYS_OF_CHRISTMAS.iter().enumerate() {
        println!(
            "On the {} day of Christmas, my true love sent to me\n{}",
            day_of_christmas.0,
            day_of_christmas.1
        );
        if i != 0 {
            let mut past_index_of_christmas = i - 1;
            loop {
                if past_index_of_christmas == 0 {
                    println!("And {}", TWELVE_DAYS_OF_CHRISTMAS[past_index_of_christmas].1.to_lowercase());
                    break;
                } else {
                    println!("{}", TWELVE_DAYS_OF_CHRISTMAS[past_index_of_christmas].1);
                    past_index_of_christmas -= 1;
                } 
            }
        }
    }
}

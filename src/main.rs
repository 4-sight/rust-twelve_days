fn main() {
    let gifts: [&str; 11] = [
        "    Two turtle doves,",
        "    Three French hens,",
        "    Four calling birds,",
        "    Five gold rings,",
        "    Six geese a laying,",
        "    Seven swans a swimming,",
        "    Eight maids a milking,",
        "    Nine ladies dancing,",
        "    Ten lords a leaping,",
        "    Eleven pipers piping,",
        "    Twelve drummers drumming,",
    ];

    let days: [&str; 12] = [
        "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for d in 1u8..13u8 {
        println!(
            "\nOn the {} day of Christmas\nMy true love gave to me",
            days[d as usize]
        );

        get_gifts((d) as usize, &gifts);
        if d > 1 {
            println!("    And a partridge in a pear tree.")
        } else {
            println!("    A partridge in a pear tree.")
        }
    }
}

fn get_gifts(n: usize, gifts: &[&str; 11]) {
    if n > 1 {
        for i in (0..n - 1).rev() {
            println!("{}", &gifts[i]);
        }
    }
}

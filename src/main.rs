fn main() {
    let gifts: [String; 11] = [
        String::from("    Two turtle doves,"),
        String::from("    Three French hens,"),
        String::from("    Four calling birds,"),
        String::from("    Five gold rings,"),
        String::from("    Six geese a laying,"),
        String::from("    Seven swans a swimming,"),
        String::from("    Eight maids a milking,"),
        String::from("    Nine ladies dancing,"),
        String::from("    Ten lords a leaping,"),
        String::from("    Eleven pipers piping,"),
        String::from("    Twelve drummers drumming,"),
    ];

    for d in 1u8..13u8 {
        let day = match long_form(d) {
            Some(s) => s,
            None => String::from("error"),
        };

        if day == "error" {
            println!("Day out of range");
            return;
        }

        println!("\nOn the {} day of Christmas\nMy true love gave to me", day);

        get_gifts((d) as usize, &gifts);
        if d > 1 {
            println!("    And a partridge in a pear tree.")
        } else {
            println!("    A partridge in a pear tree.")
        }
    }
}

fn get_gifts(n: usize, gifts: &[String; 11]) {
    if n > 1 {
        for i in (0..n - 1).rev() {
            println!("{}", &gifts[i]);
        }
    }
}

fn long_form(n: u8) -> Option<String> {
    match n {
        1 => Some(String::from("first")),
        2 => Some(String::from("second")),
        3 => Some(String::from("third")),
        4 => Some(String::from("forth")),
        5 => Some(String::from("fifth")),
        6 => Some(String::from("sixth")),
        7 => Some(String::from("seventh")),
        8 => Some(String::from("eight")),
        9 => Some(String::from("ninth")),
        10 => Some(String::from("tenth")),
        11 => Some(String::from("eleventh")),
        12 => Some(String::from("twelfth")),
        _ => None,
    }
}

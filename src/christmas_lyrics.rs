// printing lyrics to the Christmas carol “The Twelve Days of Christmas,”
// taking advantage of the repetition in the song.
pub fn lyrics() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            println!("{}", gifts[j]);
        }
        println!();
    }
}

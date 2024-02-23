fn main() {
    // Storing all the lyrics
    const LYRICS: [&str; 12] = ["A partridge in a pear tree.", "Two turtle doves, and", "Three French hens,", "Four calling birds,", "Five gold rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];
    
    // Storing the days
    const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    // Looping over the lyrics
    println!("\n\n--- Twelwe days of Christmas: A Christmas Carol ---\n");
    for i in 1..13 {
        // Printing the common line once every iteration
        println!("On the {} day of Christmas,\nMy true love sent to me", DAYS[i-1]);
        for j in (0..i).rev() {
            println!("{}", LYRICS[j]);
        }
        println!("\n");
    }
}

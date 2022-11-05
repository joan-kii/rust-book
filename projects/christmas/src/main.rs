fn main() {

    let lyrics = [
        "Two turtle doves, and",
        "Three french hens\n",
        "Four calling birds\n",
        "Five golden rings\n",
        "Six geese a-laying\n",
        "Seven swans a-swimming\n",
        "Eight maids a-milking\n",
        "Nine ladies dancing\n",
        "Ten lords a-leaping\n",
        "Eleven pipers piping\n",
        "Twelve drummers drumming\n"
    ];

    let mut count: usize = 0;
    let mut content = String::new();

    for verse in 0..=11 {
        println!("On the first day of Christmas, my true love sent to me");

        while count < verse {

            content = lyrics[count].to_owned() + &content;

            println!("{content}");

            count += 1;
        }

        println!("A partridge in a pear tree\n");
    }
}

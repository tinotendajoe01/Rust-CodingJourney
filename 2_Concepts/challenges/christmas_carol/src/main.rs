// fn main() {
//     let days = [
//         "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
//         "tenth", "eleventh", "twelfth",
//     ];

//     let gifts = [
//         "a Partridge in a Pear Tree.\n",
//         "2 Turtle Doves\nand ",
//         "3 French Hens, ",
//         "4 Calling Birds, ",
//         "5 Golden Rings, ",
//         "6 Geese a Laying, ",
//         "7 Swans a Swimming, ",
//         "8 Maids a Milking, ",
//         "9 Ladies Dancing, ",
//         "10 Lords a Leaping, ",
//         "11 Pipers Piping, ",
//         "12 Drummers Drumming, ",
//     ];

//     for i in 0..12 {
//         println!(
//             "\nOn the {} day of Christmas\nmy true love sent to me:",
//             days[i]
//         );
//         for j in (0..=i).rev() {
//             print!("{}", gifts[j]);
//         }
//     }
// }
fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree.\n",
        "2 Turtle Doves\nand ",
        "3 French Hens, ",
        "4 Calling Birds, ",
        "5 Golden Rings, ",
        "6 Geese a Laying, ",
        "7 Swans a Swimming, ",
        "8 Maids a Milking, ",
        "9 Ladies Dancing, ",
        "10 Lords a Leaping, ",
        "11 Pipers Piping, ",
        "12 Drummers Drumming, ",
    ];

    'outer: for (i, day) in days.iter().enumerate() {
        println!(
            "\nOn the {} day of Christmas\nmy true love sent to me:",
            day
        );
        for gift in gifts.iter().take(i + 1).rev() {
            print!("{}", gift);
        }

        if i == days.len() - 1 {
            break 'outer;
        }
    }
}

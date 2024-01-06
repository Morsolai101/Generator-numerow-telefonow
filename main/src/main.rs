/*
    phoneNumberGenerator
*/

/*
connectingBiblical:
biblicalBibliotOfOutput,BiblicalFormat (creation,Greeting)
*/
use std::io;
use std::io::Write;
//LList of forbidden combinations of numbers (constant (not formatted)) List of prohibited combinations of dials (constant (not format))
const BLOK: [&str; 10] = [
    "0000", "1111", "2222", "3333", "4444", "5555", "6666", "7777", "8888", "9999",
];

// The main cycle main
fn main() {
    // Outputting words to the terminal
    println!("Enter the operator's number in 2 digits:");

    /*
        Creating a variable with a water of values from the keyboard
    */

    // Creating a blank sheet
    let mut kib_kod: String = String::new();

    // From keyboard values to mutable change kib_kod(CHANGE NOT DEFINED)
    io::stdin().read_line(&mut kib_kod).expect("Error");
    /*
    We set a new non-mutable variable kod with a well-defined class String(syllable)
    and remove spaces from the beginning and end of the email using trim()
    */
    let kod: &str = kib_kod.trim();

    // Set the length of the number
    // let mut kib_length: String = String::new();
    // io::stdin().read_line(&mut kib_length).expect("Помилка");

    // let s: &str = kib_length.as_str().trim();
    // let kil: f32 = s.parse::<f32>().unwrap(); //IN STR, OUT I32

    let mut length: f32 = 0.0000001;
    // Create a counter for the number of generated numbers
    let mut quantity: i32 = 0;

    // Create a file (nomer.txt)
    println!("Creating a file (nomer.txt) with numbers");
    let mut file: std::fs::File = std::fs::File::create("nomer.txt").expect("Error");

    /*
    Cycle with number creation
    */

    while length < 1.0 {
        // Create numbers by adding 1
        length += 0.0000001;

        // We format this format in a string using (format("{:.x}" - how much to round the value)
        let format: String = format!("{:.7}", length);
        // Delete the first two 0 characters.
        let arr: &str = &format[2..];

        // Create the final number
        let z: String = format!("{}{}", &kod, &arr);
        // Check if the values do not fall within the block
        if !z.contains(&BLOK[0])
            && !z.contains(&BLOK[1])
            && !z.contains(&BLOK[2])
            && !z.contains(&BLOK[3])
            && !z.contains(&BLOK[4])
            && !z.contains(&BLOK[5])
            && !z.contains(&BLOK[6])
            && !z.contains(&BLOK[7])
            && !z.contains(&BLOK[8])
            && !z.contains(&BLOK[9])
        {
            /*
            We drive the values that have been pushed to the file (nomer.txt)
            */
            file.write_all(z.as_bytes()).expect("Error");

            // To go to the second page
            file.write_all("\n".as_bytes()).expect("Error");

            quantity += 1;
        }
    }

    // Print how many numbers we have created
    println!("The following were created {}", &quantity);

    // Print information that the program has finished working
    println!("The file is created to finish the work, press any key");

    // A gag so that you don't immediately interrupt the program
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Error");
}

mod duplicate_characters;
mod rotate_string;
mod is_palindrome;
use text_io::read;

fn main() {
    println!("Assignment-3, Anand");
    println!("enter decision for apply which you want \n
    1:- Find all duplicate characters in given string\n
    2:- Check if two strings are rotations of each other or not\n
    3:- check if a given String is Palindrome or not\n ");
    let choice: i32 = read!();
    if choice == 1{
        duplicate_characters::duplicate_char();
    }
    if choice == 2{
        println!("{}", rotate_string::rotate_check("abcd","dcba",0));
    }
    if choice == 3{
        let string = "ANAND";
        if is_palindrome::is_palindrome(string) {
            println!("Palindrome");
        } else {
            println!("Not Palindrome");
        }
    }
}

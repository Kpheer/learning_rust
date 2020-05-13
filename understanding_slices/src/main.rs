use std::io;

fn main() {
    println!("Understanding Slices!");
    println!("Enter a sentence:");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("What???");

    let word = first_word(&s);

    println!("The first word in that sentence is: {}", word);


}

fn first_word(s: &String) -> &str{

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]

}


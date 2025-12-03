fn main() {
    let text = "1188511885";

    text.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .for_each(|characters| {
            println!("Text: {text}, Window: {:?}", characters);
        });
}

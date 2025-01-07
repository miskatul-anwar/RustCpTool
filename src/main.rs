use cliclack::input;

fn main() {
    let number: String = input("What is the meaning of life?")
        .placeholder("Not sure")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();
    println!("{number:?}");
}

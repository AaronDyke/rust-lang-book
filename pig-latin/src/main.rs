fn translate(english: String) -> String {
    match &english[0..1] {
        "a" | "e" | "i" | "o" | "u" => english + "-hay",
        _ => format!("{}-{}ay", &english[1..], &english[0..1])
        
    }
}

fn main() {
    let test = String::from("aaron");

    let translated = translate(test);
    println!("{}", translated);
}

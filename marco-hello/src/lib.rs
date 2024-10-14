/* A Marco polo game
Takes input as string Marco and outputs Polo, otherwise the program with respond what is my name?
*/
pub fn marco_polo(input: &str) -> String {
    match input {
        "Marco" => String::from("Polo"),
        _ => String::from("What is my name?"),
    }
}

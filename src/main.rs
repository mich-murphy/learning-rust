pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let _student = student;
    let mut plants: Vec<&'static str> = vec![];
    for (_iter, char) in diagram.chars().enumerate() {
        if char.to_string() == "G" {
            plants.push("grass");
        } else if char.to_string() == "C" {
            plants.push("clover");
        } else if char.to_string() == "R" {
            plants.push("radish");
        } else if char.to_string() == "V" {
            plants.push("violet");
        }
    } 
    plants
}

fn main() {

}

#[test]
fn garden_with_single_student() {
    let diagram = "RCGG";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

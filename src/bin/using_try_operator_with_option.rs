fn main() {
    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last(&mut animals));
    println!("{:?}", length_of_last(&mut animals));
    println!("{:?}", length_of_last(&mut animals));
    println!("{:?}", length_of_last(&mut animals));
}

fn length_of_last(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}
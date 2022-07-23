pub fn find_unique_substr_in_vec(vec: &Vec<String>, substr: &str) -> usize {
    let mut found: Option<usize> = None;
    for (index, i) in vec.into_iter().enumerate() {
        if i.contains(&substr) {
            if found.is_some() {
                println!("Error: Found more than one match for {}", substr);
                std::process::exit(0);
            }
            found = Some(index);
        }
    }
    match found {
        Some(index) => index,
        None => panic!("Could not find {}", substr),
    }
}

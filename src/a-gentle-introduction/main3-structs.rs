struct IntroduceYourselfStruct {
    name: String,
    age: i32
}

fn main() {
    let my_introduction = 
        IntroduceYourselfStruct { 
            name: String::from("your_name_here"),
            age: 3
        };

    dbg!(my_introduction.name.clone());
    dbg!(my_introduction.age.clone());

    // TODO change these into tests below instead
    // PRs welcome ^^
    assert_ne!(my_introduction.name, "your_name_here");
    assert_ne!(my_introduction.age, 3);
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_main3() {
//         assert_neq!(
            
//         );
//     }

// }

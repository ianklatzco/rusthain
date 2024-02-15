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

    dbg!(my_introduction.name);
    dbg!(my_introduction.age);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main3() {
        assert_neq!(

        );
    }

}

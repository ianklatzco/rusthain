




// welcome!
// change the string below to hallo, rusthain! to begin!
// your rust journey awaits!
fn hello() -> String {
    String::from("hallo, rusthain!")
} // hallo, rusthain!

fn add1(a: i32) -> i32 {
    // return a + 1;  // statement
    a
}

// rust differentiates between "expression" and "statements"

fn default() {
}



// the stuff below is your usual "boilerplate"
// you'll almost always see a main() function
// and you'll often see a test,
//   either in the same file
//   or a directory nearby named "tests"


// ==========================================================================
// ==========================================================================

fn main() {
    let s = hello();
    dbg!(s);
}

// #[derive(Debug,Copy)]
// fn foo() {
//     dbg!("hello");
// }

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn test_main1() {

        assert_eq!(
            hello(), 
            String::from("hallo, rusthain!")
        );
    }
}





// welcome!
// change the string below to hallo, rusthain! to begin!
// your rust journey awaits!
fn hello() -> String {
    String::from("TODO")
} // hallo, rusthain!




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
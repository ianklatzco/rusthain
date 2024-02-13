




// welcome!
// change the string below to hallo, xhain! to begin!
// your rust journey awaits!
fn string_to_print() -> String {
    String::from("TODO")
} // hallo, xhain!




// the stuff below is your usual "boilerplate"
// you'll almost always see a main() function
// and you'll often see a test,
//   either in the same file
//   or a directory nearby named "tests"


// ==========================================================================
// ==========================================================================

fn main() {
    let s = string_to_print();
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use crate::string_to_print;

    #[test]
    fn it_works() {

        assert_eq!(
            string_to_print(), 
            String::from("hallo, xhain!")
        );
    }
}
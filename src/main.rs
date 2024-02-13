// welcome!
// the println!() "function" is your friend!
// write println!() and pass in "Hallo, xhain!" 
fn string_to_print() -> String {
    return String::from("TODO");
} 

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
            String::from("Hallo, xhain!")
        );
    }
}
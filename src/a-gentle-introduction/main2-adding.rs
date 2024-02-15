// main2.rs: addition
// make the numbers sum up to a specific value!

fn arithmetic() -> i32 {
    // pick four numbers, any four numbers....
    let a = 2;
    let b = 4;
    let c = 0;
    let d = 8;
    return a + b + c + d;
}

// all of these exercises are just guidelines for learning
// you probably know some other arithmetic operations: + - / * pow
// can you google/chatgpt to figure out how to use them in rust,
// then change 

// ==========================================================================
// ==========================================================================

fn main() {
    let result = arithmetic();
    dbg!(result);
}

// #[] is an attribute that applies to the thing that follows
// #![] is anattribute that applies to the thing that is within
#[cfg(test)]
mod tests {
    use crate::arithmetic;

    #[test]
    fn test_main2() {
        assert_eq!(
            arithmetic(), 
            14
        );
    }

    #[ignore]
    #[test]
    fn main2_extra_credit() {
        // write your own test!
        // for example, define your own fn add(a,b)
        // and use it here inside an assert_eq!(add(2,3),5)
        // (don't forget to remove #[ignore]!)
    }
}
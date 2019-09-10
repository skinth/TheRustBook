fn main() {
    let s = String::from("hello");  // s comes into scope

    let s_clone = s.clone();
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    //println!("s: {}",s); //here s is not valid because it has been moved in takes_ownership()
    println!("s_clone: {}", s_clone);

    let mut s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    println!("len of s1: {}", get_length(&s1)); //we pass s1 by reference, so it not move!

    change(&mut s1); //it is mutable reference so we can change the value referenced by s
    println!("new s1 value: {}", s1);

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    //SLICE
    let s_slice = String::from("lost in translation");
    let five = &s_slice[0..5];

    //cannot clear string now! Because it is used after (so the compiler think it is immutable)
    //s_slice.clear();

    println!("{}", five);

    //slice not take ownership!
    println!("First Word: {}", first_word(&s_slice));

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


//WE CAN USE REFERENCE!!
fn get_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
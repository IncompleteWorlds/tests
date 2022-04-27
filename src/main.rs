use std::collections::HashMap;

fn print_vec(in_vec: &Vec<u8>) {
    for v in in_vec {
        print!("{} ", v);
    }
    println!();
}

fn test_vectors() {
    let mut myvec: Vec<u8> = Vec::new();

    myvec.push(56);
    myvec.push(0);
    myvec.push(0xf7);
    myvec.push(59);
    myvec.push(0xaa);
    print_vec(&myvec);

    let mut i: &u8;

    i = &myvec[0];
    println!("i = {}", i);

    i = myvec.get(4).unwrap();
    println!("i = {}", i);

    i = &myvec[0];
    println!("i = {}", i);
}

fn test_maps() {
    let mut map: HashMap<&str, i32>;

    map = HashMap::new();

    map.insert("pepe", 1);
    map.insert("pepe", 8);

    for (k,v) in &map {
        println!("{} = {}", k,v);
    }

    if map.contains_key("pepe") {
        println!("Key found");
    }

    if map.contains_key("pepe1") {
        println!("Key found");
    } else {
        println!("NO Key");
    }

}

fn test_maps1() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }
}

fn main() {
    //test_vectors();

    test_maps();
}

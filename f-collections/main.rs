fn main() {
	/*
	An array is data inside square brackets: []. Arrays:

    - must not change their size,
    - must only contain the same type.
	*/
    let array1 = ["One", "Two"]; // This one is type [&str; 2]
    let array2 = ["One", "Two", "Five"]; // But this one is type [&str; 3]. Different type!
	
	println!("Array one {:?} and array two {:?}",array1,array2);

	//Creating an array of size 10 with same initialized values

	let my_array = ["a"; 10];
    println!("{:?}", my_array);
	
	let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

	//These are slices

    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}", three_to_five, start_at_two, end_at_five, everything);

	//Slices also work with strings
	
	let s = String::from("hello");

	let len = s.len();

	let slice = &s[3..len];
	let slice2 = &s[1..];
	
	println!("First slice {0} and second slice {1}",slice,slice2);

	//Vectors are to arrays like String is to &str
	
	let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();
    // If we run the program now, the compiler will give an error.
    // It doesn't know the type of vec.

    my_vec.push(name1); // Now it knows: it's Vec<String>
    my_vec.push(name2);
	
	println!("{:?}",my_vec);

	//Vectors can also be created with vec! macro
	
	let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Everything is the same as above except we added vec!.
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("Three to five: {:?},start at two: {:?},end at five: {:?},everything: {:?}", three_to_five, start_at_two, end_at_five, everything);

	//Tuples are collections that can house multiple different types. Empty functions are actually tuples
	
	let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}, Second item: {:?}, Third item: {:?}, Fourth item: {:?}, Fifth item: {:?},Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    )
}
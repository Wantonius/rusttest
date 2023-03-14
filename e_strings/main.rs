fn main() {
	
	//There are two String types in Rust. First one is a str reference &str which is a light-weight and fast. 
	//The other is String type which is heavier but has more functionality 
    let name = "Bingo"; 
    let other_name = String::from("Adrian Fahrenheit Țepeș"); 

	println!("My name is {0} and this is {1}",name,other_name);
	
	//The &str is dynamically sized, it is a reference, and String is what is known as an owned type and it has a size.
	
	println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. 'Bingo' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Bingo")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));

	//There are many ways to create strings and one more is format! macro
	
	let my_name = "Jim Bob";
    let my_country = "USA";
    let my_home = "Alabama";

    let together = format!(
        "I am {} and I come from {}. I live in {}.",
        my_name, my_country, my_home
    );
	
	println!("{}",together)
}
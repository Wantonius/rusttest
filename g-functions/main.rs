fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn print_country_returns_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name // return it here. Note that return value is just a line WITHOUT a semicolon.
}

fn print_country_with_reference(country_name: &String) {
    println!("{}", country_name);
}

fn add_hungary(country_name: &mut String) { // first we say that the function takes a mutable reference
    country_name.push_str("-Hungary"); // push_str() adds a &str to a String
    println!("Now it says: {}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(country); // We print "Austria"
    //print_country(country); 

	/*
	If we remove the comment from that other print_country line it will not work. Rust has very specific ownership
	rules for variables and transferring the variable to a function will also transfer the ownership. Also when
	ever a block of code ends ALL variables and their associated memory will be released. So the country variable no longer exists on the second line
	*/
	
    let country = String::from("Austria");
    let country = print_country_returns_country(country); // we have to use let here now to get the String back
    print_country_returns_country(country);	
	
	//This works but is unfeasible. Returning the original value back seems bothersome. 

    let country = String::from("Austria");
    print_country_with_reference(&country); // We print "Austria"
    print_country_with_reference(&country); // That was fun, let's do it again!

	//What we actually want to do is pass a reference to the function. This is called borrowing in Rust language.
	
	//Passing a mutable reference
	
	let mut country = String::from("Austria");
	add_hungary(&mut country); 
	
}
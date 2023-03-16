fn main() {

	//Base for loop
    for number in 0..3 {
        println!("The number is: {}", number);
    }
	
	//Base for loop to equal
    for number in 0..=3 {
        println!("The next number is: {}", number);
    }
	
	//Loop until you break. No condition on the loop
	
	let mut counter = 0; // set a counter to 0
	
	loop {
        counter +=1; // increase the counter by 1
        println!("The counter is now: {}", counter);
        if counter == 5 { // stop when counter == 5
            break;
        }
    }
	
	//Named loops. We break from the first in the second. Use tick "'" and a name to name loops
	
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop {
        // Give the first loop a name
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 9 {
            // Starts a second loop inside this loop
            println!("Now entering the second loop.");

            'second_loop: loop {
                // now we are inside 'second_loop
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop; // Break out of 'first_loop so we can exit the program
                }
            }
        }
    }
	println!("After second loop");
	//And finally...while
	
	let mut counter = 0;

    while counter < 5 {
        counter +=1;
        println!("The counter is now: {}", counter);
    }
}
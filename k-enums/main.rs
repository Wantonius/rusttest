// We create enum with two options
enum ThingsInTheSky {
    Sun,
    Stars,
}

// Use the clock to create things in the sky 
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun, // Between 6 and 18 hours we can see the sun
        _ => ThingsInTheSky::Stars, // Otherwise, we can see stars
    }
}

// With this function we can match against the two choices in ThingsInTheSky.
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}

fn main() {
    let time = 8; 
    let skystate = create_skystate(time); 
    check_skystate(&skystate); // Give it a reference so it can read the variable skystate. Remember the ownership rules.

	let time2 = 22; 
    let skystate2 = create_skystate(time2); 
    check_skystate(&skystate2);
}
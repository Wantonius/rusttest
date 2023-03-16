//Structs can be so-called tuple structs
#[derive(Debug)] //This is a built-in trait. More on traits later. This helps us to debug print since tuple structs do not normally carry this trait.
struct Color(u8, u8, u8); 

//Or more usual named structs
struct SizeAndColor {
    size: u32,
    color: Color, // And we put it in our new named struct
}

fn main() {
    let my_color = Color(50, 0, 50);

    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color
    };
	
	println!("Size is {} and color is {:?}",size_and_color.size,size_and_color.color)
}
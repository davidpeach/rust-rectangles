struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The are of this rectangle is {} square pixels.", 
        area(&rectangle)
    );
}

// & is an Immutable Borrow (main keeps ownership of rectangle)
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
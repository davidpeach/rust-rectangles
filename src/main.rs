fn main() {
    let width = 30;
    let height = 50;
    let rectangle = (width, height);

    println!(
        "The area of this rectangle is {} square pixels.", 
        area(rectangle)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
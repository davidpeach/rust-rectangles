fn main() {
    let width = 31;
    let height = 50;

    println!(
        "The are of this rectangle is {} square pixels.", 
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of this rectangle is {} square pixels.", 
        rectangle.area()
    );

    let rectangle2 = Rectangle {
        width: 10,
        height: 30,
    };

    println!("Can rectangle hold rectangle2 ? {}", rectangle.can_hold(&rectangle2));


    let new_square = Rectangle::square(30);

    println!("The is my new square height: {} and width: {}", new_square.height, new_square.width);
}

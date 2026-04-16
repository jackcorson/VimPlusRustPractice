
pub fn setupStructPractice() {
    calculateAreaAndPerimeter();
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    height: u32,
    width: u32
}

struct Color(u32, u32, u32);

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
    
    fn perimeter(&self) -> u32 {
        return self.height*2 + self.width*2;
    }

    fn cmp(&self, otherRec: &Rectangle) -> bool {
        return self == otherRec;
    }
}

fn calculateAreaAndPerimeter() {
    let rec = Rectangle {
        height: 30,
        width: 20,
    };

    let area = rec.area();
    let perimeter = rec.perimeter();
    println!("The area of the rectangle: {rec:?} is {area} and it's perimeter is {perimeter}");

    structUpdateAndCmp(&rec);
    println!("{rec:?}: I can still be used");

}

fn structUpdateAndCmp(rec: &Rectangle) {
    let newRec = Rectangle { 
        height: 10,
        ..*rec
    };

    let area = newRec.area();
    let perimeter = newRec.perimeter();
    println!("New rec: {newRec:?} has an area of {area} and a perimeter of {perimeter}");
    let equal = newRec.cmp(&rec);
    if equal { println!("The two structs are equal") } else { println!("The two structs are not equal") }              
}
use super::traits::HasArea;

pub fn print_area<T: HasArea>(shape: T) {
    println!("area: {}", shape.area());
}

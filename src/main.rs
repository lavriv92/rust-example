mod modules;

use modules::structs;
use modules::utils::print_area;

fn main() {
    let circle = structs::CircleBuilder::new()
                .x(5.0)
                .y(3.0)
                .radius(5.0)
                .finalize();

    let square = structs::Square::new(1.0, 2.0, 3.0);

    print_area(square);
    print_area(circle);
}

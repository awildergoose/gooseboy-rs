use crate::test;
use gooseboy::color::Color;

pub fn test_color() {
    let c = Color::new(1, 2, 3, 4);
    test!("color:new", c.r == 1 && c.g == 2 && c.b == 3 && c.a == 4);

    let co = Color::new_opaque(10, 20, 30);
    test!(
        "color:new_opaque",
        co.r == 10 && co.g == 20 && co.b == 30 && co.a == 255
    );
}

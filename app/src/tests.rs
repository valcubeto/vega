#![allow(dead_code, unused)]

#[test]
fn debugging() {
    use terminal::debug;
    debug!("A nice message");

    #[derive(Debug)]
    struct Point {
        pub x: i32,
        pub y: i32
    }

    let point = Point { x: -21, y: 34 };
    debug!(point);
    debug!(point.x);
    debug!("x is {}", point.x)
}

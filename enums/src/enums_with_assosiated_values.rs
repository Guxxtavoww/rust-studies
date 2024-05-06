pub enum Coords {
    TwoD(i32, i32),
    TreeD(i32, i32, i32),
}

pub fn enum_with_values() {
    let point_2d = Coords::TwoD(5, 10);
    let point_3d = Coords::TreeD(3, 8, 15);

    match point_2d {
        Coords::TwoD(x, y) => println!("x: {}, y: {}", x, y),
        Coords::TreeD(_x, _y, _z) => println!("3d coords"),
    }

    match point_3d {
        Coords::TwoD(_x, _y) => println!("2d coords"),
        Coords::TreeD(x, y, z) => println!("x: {}, y: {}, z: {}", x, y, z),
    }
}

pub fn enum_with_if() {
    let point_2d = Coords::TwoD(5, 10);
    let point_3d = Coords::TreeD(3, 8, 15);

    if let Coords::TwoD(x, y) = point_2d {
        println!("x: {}, y: {}", x, y);
    } else {
        println!("3d coords");
    }

    if let Coords::TreeD(x, y, z) = point_3d {
        println!("x: {}, y: {}, z: {}", x, y, z);
    } else {
        println!("2d coords");
    }
}

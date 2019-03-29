pub type Size = Size32;
pub type Pos = xyz::Pos32;
pub type Rect = xyz::Rect32;

pub type Pos2 = xy::Pos32;
pub type Rect2 = xy::Rect32;
pub type Pos2d = xy::Pos32;
pub type Rect2d = xy::Rect32;

pub type Pos3 = xyz::Pos32;
pub type Rect3 = xyz::Rect32;
pub type Pos3d = xyz::Pos32;
pub type Rect3d = xyz::Rect32;

pub struct Size32 {
    pub width: i32,
    pub height: i32
}

pub mod xy {
    pub struct Pos32 {
        pub x: i32,
        pub y: i32
    }

    pub struct Rect32 {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32
    }
}

pub mod xyz {
    pub struct Pos32 {
        pub x: i32,
        pub y: i32,
        pub z: i32
    }

    pub struct Rect32 {
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub width: i32,
        pub height: i32
    }
}

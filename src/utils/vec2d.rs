pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl ToString for Vec2d {
    fn to_string(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }
}


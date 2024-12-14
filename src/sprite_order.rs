pub enum SpriteOrder {
    GRASS,
    Player,
    Gun,
    Bullet,
    Enemy,
}

impl SpriteOrder {
    pub fn z_index(&self) -> f32 {
        match self {
            SpriteOrder::GRASS => 0.0,
            SpriteOrder::Player => 2.0,
            SpriteOrder::Gun => 0.1,
            SpriteOrder::Bullet => 1.0,
            SpriteOrder::Enemy => 1.0,
        }
    }
}

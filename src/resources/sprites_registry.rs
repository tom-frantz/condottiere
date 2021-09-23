use amethyst::assets::Handle;
use amethyst::renderer::{SpriteRender, SpriteSheet};

pub struct SpriteRegistry {
    default_sprite: SpriteRender,
}

impl SpriteRegistry {
    pub fn new(default_sprite: SpriteRender) -> Self {
        SpriteRegistry { default_sprite }
    }

    pub fn get_default_sprite(&self) -> SpriteRender {
        self.default_sprite.clone()
    }
}

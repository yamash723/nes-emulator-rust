use crate::nes::ram::Ram;
use super::sprite::Sprite;

pub type SpritesWithCtx = Vec<SpriteWithCtx>;

#[derive(Debug)]
pub struct SpriteWithCtx {
    pub sprite: Sprite,
}

impl SpriteWithCtx {
}

#[cfg(test)]
mod sprite_with_ctx_test {
    use super::*;

}
use anyhow::Result;
use specs_blit::{
    blit::{BlitBuffer, Color},
    SpriteRef,
};
use sprite_gen::{gen_sprite, MaskValue, Options};

/// Generate a random sprite from a mask and return it as a blit buffer.
pub fn generate(width: usize, options: Options, mask: &[MaskValue]) -> Result<SpriteRef> {
    let buffer_width = if options.mirror_x { width * 2 } else { width };

    let buf = BlitBuffer::from_buffer(
        &gen_sprite(&mask, width, options)
            .into_iter()
            // Invert the colors
            .map(|p| p ^ 0xFF_FF_FF_FF)
            .collect::<Vec<_>>(),
        buffer_width as i32,
        Color::from_u32(0),
    );

    specs_blit::load(buf)
}

use ggez::graphics::Rect;

pub struct Tile {
    position: [f32; 2],
    size: [f32; 2],
    size_raw: [f32; 2],
}

impl Tile {
    pub fn new(position: [f32; 2], size: [f32; 2], sprite_sheet_size: [f32; 2]) -> Self {
        let [sprite_sheet_width, sprite_sheet_height] = sprite_sheet_size;
        let [position_x, position_y] = position;
        let [width, height] = size;
        Tile {
            position: [
                position_x / sprite_sheet_width,
                position_y / sprite_sheet_height,
            ],
            size: [width / sprite_sheet_width, height / sprite_sheet_height],
            size_raw: size,
        }
    }

    pub fn position(&self) -> &[f32; 2] {
        &self.position
    }

    pub fn size(&self) -> &[f32; 2] {
        &self.size
    }

    pub fn size_raw(&self) -> &[f32; 2] {
        &self.size_raw
    }
}

impl From<&Tile> for Rect {
    fn from(tile: &Tile) -> Rect {
        let [x, y] = tile.position;
        let [width, height] = tile.size;
        Rect {
            x: x as f32,
            y: y as f32,
            w: width as f32,
            h: height as f32,
        }
    }
}

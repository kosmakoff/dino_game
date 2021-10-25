use ggez::graphics::Rect;

pub struct Tile {
    pub position: [f32; 2],
    pub size: [f32; 2],
}

impl Tile {
    pub fn new(position: [f32; 2], size: [f32; 2]) -> Self {
        Tile { position, size }
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

use crate::modules::{
    textured::Sprite,
};
use math::Vec3;
use super::vertex::Vertex2;

pub fn build_block_mesh(sprites: &Vec<Sprite>, pos: Vec3, block_index: usize) -> Option<(Vec<Vertex2>, Vec<i32>)> {
    let mut new_data = Vec::<Vertex2>::new();
    let mut new_index = Vec::<i32>::new();
    let (width, height) = (626.0, 1782.0);

    let value = if let Some(value) = sprites.get(block_index % 330) {
        value
    } else {
        return None;
    };

    let index = 0;
    let x = &value.frame.x;
    let y = &value.frame.y;
    let xw = x + &value.frame.w;
    let yh = y + &value.frame.h;
    let (xx, yy, xw, yh) = (x / width, y / height, xw / width, yh / height);
    let pbs = 1.0;
    let nbs = -1.0;

    // Top
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + pbs,
        pos.z + nbs,
        xx,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + pbs,
        pos.z + pbs,
        xx,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + pbs,
        pos.z + pbs,
        xw,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + pbs,
        pos.z + nbs,
        xw,
        yy,
    ));

    // Top
    new_index.push(((index * 24) + 0) as i32);
    new_index.push(((index * 24) + 1) as i32);
    new_index.push(((index * 24) + 2) as i32);
    new_index.push(((index * 24) + 0) as i32);
    new_index.push(((index * 24) + 2) as i32);
    new_index.push(((index * 24) + 3) as i32);

    // Left
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + pbs,
        pos.z + pbs,
        xx,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + nbs,
        pos.z + pbs,
        xw,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + nbs,
        pos.z + nbs,
        xw,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + pbs,
        pos.z + nbs,
        xx,
        yh,
    ));

    // Left
    new_index.push(((index * 24) + 5) as i32);
    new_index.push(((index * 24) + 4) as i32);
    new_index.push(((index * 24) + 6) as i32);
    new_index.push(((index * 24) + 6) as i32);
    new_index.push(((index * 24) + 4) as i32);
    new_index.push(((index * 24) + 7) as i32);

    // Right
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + pbs,
        pos.z + pbs,
        xw,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + nbs,
        pos.z + pbs,
        xx,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + nbs,
        pos.z + nbs,
        xx,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + pbs,
        pos.z + nbs,
        xw,
        yy,
    ));

    // Right
    new_index.push(((index * 24) + 8) as i32);
    new_index.push(((index * 24) + 9) as i32);
    new_index.push(((index * 24) + 10) as i32);
    new_index.push(((index * 24) + 8) as i32);
    new_index.push(((index * 24) + 10) as i32);
    new_index.push(((index * 24) + 11) as i32);

    // Front
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + pbs,
        pos.z + pbs,
        xw,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + nbs,
        pos.z + pbs,
        xw,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + nbs,
        pos.z + pbs,
        xx,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + pbs,
        pos.z + pbs,
        xx,
        yh,
    ));

    // Front
    new_index.push(((index * 24) + 13) as i32);
    new_index.push(((index * 24) + 12) as i32);
    new_index.push(((index * 24) + 14) as i32);
    new_index.push(((index * 24) + 15) as i32);
    new_index.push(((index * 24) + 14) as i32);
    new_index.push(((index * 24) + 12) as i32);

    // Back
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + pbs,
        pos.z + nbs,
        xx,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + nbs,
        pos.z + nbs,
        xx,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + nbs,
        pos.z + nbs,
        xw,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + pbs,
        pos.z + nbs,
        xw,
        yy,
    ));

    new_index.push(((index * 24) + 16) as i32);
    new_index.push(((index * 24) + 17) as i32);
    new_index.push(((index * 24) + 18) as i32);
    new_index.push(((index * 24) + 16) as i32);
    new_index.push(((index * 24) + 18) as i32);
    new_index.push(((index * 24) + 19) as i32);

    // Bottom
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + nbs,
        pos.z + nbs,
        xw,
        yh,
    ));
    new_data.push(Vertex2::new(
        pos.x + nbs,
        pos.y + nbs,
        pos.z + pbs,
        xw,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + nbs,
        pos.z + pbs,
        xx,
        yy,
    ));
    new_data.push(Vertex2::new(
        pos.x + pbs,
        pos.y + nbs,
        pos.z + nbs,
        xx,
        yh,
    ));

    new_index.push(((index * 24) + 21) as i32);
    new_index.push(((index * 24) + 20) as i32);
    new_index.push(((index * 24) + 22) as i32);
    new_index.push(((index * 24) + 22) as i32);
    new_index.push(((index * 24) + 20) as i32);
    new_index.push(((index * 24) + 23) as i32);

    Some((new_data, new_index))
}

use crate::Octant;

pub fn find_center(octant: Octant) -> (i32, i32, i32) {
    let (origin_x, origin_y, origin_z) = octant.origin;
    let midpoint_x = origin_x + octant.length/2;
    let midpoint_y = origin_y + octant.height/2;
    let midpoint_z = origin_z + octant.width/2;

    (midpoint_x, midpoint_y, midpoint_z)
}
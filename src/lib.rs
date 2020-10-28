mod utils;

type Boxtant = Option<Box<Octant>>;
type Point = (i32, i32, i32);
pub struct Octant{
    front_top_left: Boxtant,
    front_top_right: Boxtant,
    front_bottom_right: Boxtant,
    front_bottom_left: Boxtant,
    back_top_left: Boxtant,
    back_top_right: Boxtant,
    back_bottom_right: Boxtant,
    back_bottom_left: Boxtant,
    length: i32,
    width: i32,
    height: i32,
    origin: Point,
    points: Vec<Point>,
    split: bool
}

impl Octant {
    pub fn new(length: i32, width: i32, height: i32, origin: Point) -> Octant {
        Octant{
            front_top_left: None,
            front_top_right: None,
            front_bottom_left: None,
            front_bottom_right: None,
            back_top_left: None,
            back_top_right: None,
            back_bottom_left: None,
            back_bottom_right: None,
            length,
            width,
            height,
            origin,
            points: Vec::new(),
            split: false
        }
    }

    fn split(&mut self){
        let center = self.find_center();
        let length = self.length/2;
        let width = self.width/2;
        let height = self.height/2;

        //(x,y,z)
        let ftl = (center.0-length, center.1+height, center.2-width);
        let ftr = (center.0, center.1+height, center.2-width);

        let fbl = (center.0-length, center.1, center.2-width);
        let fbr = (center.0, center.1, center.2-width);

        let btl = (center.0-length, center.1+height, center.2);
        let btr = (center.0, center.1+height, center.2);

        let bbl = (center.0-length, center.1, center.2);
        let bbr = (center.0, center.1, center.2);

        self.front_top_left = Some(Box::new(Octant::new(length, width, height, ftl)));
        self.front_top_right = Some(Box::new(Octant::new(length, width, height, ftr)));
        self.front_bottom_left = Some(Box::new(Octant::new(length, width, height, fbl)));
        self.front_bottom_right = Some(Box::new(Octant::new(length, width, height, fbr)));
        self.back_top_left = Some(Box::new(Octant::new(length, width, height, btl)));
        self.back_top_right = Some(Box::new(Octant::new(length, width, height, btr)));
        self.back_bottom_left = Some(Box::new(Octant::new(length, width, height, bbl)));
        self.back_bottom_right = Some(Box::new(Octant::new(length, width, height, bbr)));
        self.split = true
    }

    fn find_center(&self) -> (i32, i32, i32) {
        let (origin_x, origin_y, origin_z) = self.origin;
        let midpoint_x = origin_x + self.length/2;
        let midpoint_y = origin_y + self.height/2;
        let midpoint_z = origin_z + self.width/2;
    
        (midpoint_x, midpoint_y, midpoint_z)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

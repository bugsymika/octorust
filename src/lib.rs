mod utils;

type Boxtant = Option<Box<Octant>>;
type Point = (i32, i32, i32);

enum OctantLocations{
    FTL,
    FTR,
    FBR,
    FBL,
    BTL,
    BTR,
    BBR,
    BBL
}
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

    fn insert_point(&mut self, point: Point) {
        let max_x = (self.length as f32/2.0).round() as i32 + self.origin.0;
        let max_y = (self.height as f32/2.0).round() as i32 + self.origin.1;
        let max_z = (self.width as f32/2.0).round() as i32 + self.origin.2;

        self.points.push(point);

        if point.0 > max_x || point.1 > max_y || point.2 > max_z || self.points.len() > 4 {
            self.split()
        }
    }

    fn reallocate_points(&mut self) {
        while let Some(point) = self.points.pop(){
            let location = self.get_new_octant(point);

            match location {
                OctantLocations::FTL => self.front_top_left.as_mut().unwrap().insert_point(point), 
                OctantLocations::FTR => self.front_top_right.as_mut().unwrap().insert_point(point), 
                OctantLocations::FBR => self.front_bottom_right.as_mut().unwrap().insert_point(point), 
                OctantLocations::FBL => self.front_bottom_left.as_mut().unwrap().insert_point(point), 
                OctantLocations::BTL => self.back_top_left.as_mut().unwrap().insert_point(point), 
                OctantLocations::BTR => self.back_top_right.as_mut().unwrap().insert_point(point), 
                OctantLocations::BBR => self.back_bottom_right.as_mut().unwrap().insert_point(point), 
                OctantLocations::BBL => self.back_bottom_left.as_mut().unwrap().insert_point(point),    
            }
        }
    }

    fn get_new_octant(&self, point: Point) -> OctantLocations {
        let mid_x = (self.length as f32/2.0).round() as i32 + self.origin.0;
        let mid_y = (self.height as f32/2.0).round() as i32 + self.origin.1;
        let mid_z = (self.width as f32/2.0).round() as i32 + self.origin.2;
        
        let mut location = String::new();

        if point.2 > mid_z {
            location.push_str("B");
        } else {
            location.push_str("F");
        };

        if point.1 > mid_y {
            location.push_str("T");
        } else {
            location.push_str("B");
        };

        if point.0 > mid_x {
            location.push_str("R");
        } else {
            location.push_str("L");
        };

        match location.as_str() {
            "FTL" => return OctantLocations::FTL, 
            "FTR" => return OctantLocations::FTR, 
            "FBR" => return OctantLocations::FBR, 
            "FBL" => return OctantLocations::FBL, 
            "BTL" => return OctantLocations::BTL, 
            "BTR" => return OctantLocations::BTR, 
            "BBR" => return OctantLocations::BBR, 
            "BBL" => return OctantLocations::BBL,    
            _ => unreachable!(),
        }

    }

    fn split(&mut self){
        let center = self.find_center();
        let length = self.length/2;
        let width = self.width/2;
        let height = self.height/2;

        //(x,y,z)
        let ftl = (center.0-length, center.1, center.2-width);
        let ftr = (center.0, center.1, center.2-width);

        let fbl = (center.0-length, center.1-height, center.2-width);
        let fbr = (center.0, center.1-height, center.2-width);

        let btl = (center.0-length, center.1, center.2);
        let btr = (center.0, center.1, center.2);

        let bbl = (center.0-length, center.1-height, center.2);
        let bbr = (center.0, center.1-height, center.2);

        self.front_top_left = Some(Box::new(Octant::new(length, width, height, ftl)));
        self.front_top_right = Some(Box::new(Octant::new(length, width, height, ftr)));
        self.front_bottom_left = Some(Box::new(Octant::new(length, width, height, fbl)));
        self.front_bottom_right = Some(Box::new(Octant::new(length, width, height, fbr)));
        self.back_top_left = Some(Box::new(Octant::new(length, width, height, btl)));
        self.back_top_right = Some(Box::new(Octant::new(length, width, height, btr)));
        self.back_bottom_left = Some(Box::new(Octant::new(length, width, height, bbl)));
        self.back_bottom_right = Some(Box::new(Octant::new(length, width, height, bbr)));
        self.split = true;
        self.reallocate_points();
    }

    fn find_center(&self) -> (i32, i32, i32) {
        let (origin_x, origin_y, origin_z) = self.origin;
        let midpoint_x = origin_x + (self.length/2);
        let midpoint_y = origin_y + (self.height/2);
        let midpoint_z = origin_z + (self.width/2);
    
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

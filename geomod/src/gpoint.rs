//  GPoint.rs   created by sebhall on 20/06/2023
//
//  GPoint is the basic Point structure for GEOMOD

//  the point represents the most fundamental entity in geometry - merely a single location in space


pub mod gpoint {
    
    use crate::gobject::GObject;
    pub struct GPoint {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    
    impl GPoint {
        pub fn init(x: f64, y: f64, z: f64) -> Self {
            GPoint { x, y, z }
        }
    }
    
    impl GObject for GPoint {
        fn dump(&self) -> String {
            format!("GPoint {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
        }

        fn format(&self) -> String {
            format!("GPoint {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
        } 

        fn string_init(string: String) -> Self {
            GPoint { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
} 


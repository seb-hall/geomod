//  GLine.rs   created by sebhall on 20/06/2023
//
//  GLine is the basic Line structure for GEOMOD

//  the line is a basic entity representing a connection between two points in space

use crate::gpoint;

pub mod gline {

    pub struct GLine {
        pub start: GPoint, 
        pub end: GPoint
    }
    
    impl GLine {
        pub fn init(start: GPoint, end: GPoint) -> Self {
            GLine { start: start, end: end }
        }
    
        pub fn dump() {
            
        }
    
    }
}

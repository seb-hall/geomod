use geomod;
use geomod::gobject::GObject;

fn main() {
    let my_point = geomod::gpoint::gpoint::GPoint::init(10.0, 0.0, 0.0);
    println!("{}", my_point.dump());
}
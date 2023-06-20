//  GObject.rs   created by sebhall on 20/06/2023
//
//  GObject defines a set of common traits for all objects


pub trait GObject {
    fn dump(&self) -> String;
}

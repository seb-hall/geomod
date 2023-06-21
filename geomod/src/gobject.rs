//  GObject.rs   created by sebhall on 20/06/2023
//
//  GObject defines a set of common traits for all objects

//  common methods for managment and debugging
pub trait GObject {
    fn dump(&self) -> String;
    fn format(&self) -> String;
    fn string_init(string: String) -> Self;
}

//!
//! Properties associated with the object.
//!
//! Note that the object does not necessarily have to implement all of them.
///
/// Object, which can have a normal.
pub trait Normal<T, Dir> {
    ///
    /// Returns a normal at given point.
    fn normal_at(&self, point: &T) -> Dir;
}
///
/// Object with a length.
#[allow(clippy::len_without_is_empty)]
pub trait Length {
    ///
    /// Returns the length of the object.
    fn len(&self) -> f64;
}
///
/// Object with an area.
pub trait Area {
    ///
    /// Returns the area of the object.
    fn area(&self) -> f64;
}
///
/// Object with a volume.
pub trait Volume {
    ///
    /// Retruns the volume of the object.
    fn volume(&self) -> f64;
}
///
/// Object that has the center.
pub trait Center {
    type Output;
    ///
    /// Returns the center of the object.
    fn center(&self) -> Self::Output;
}
///
/// Algorithm for calculating the distance.
pub trait Dist<T> {
    ///
    /// Returns the distance between `&self` and `&other`.
    fn dist(&self, ohter: &T) -> f64;
}
///
/// Manages user-defined metadata.
pub trait Metadata<T> {
    ///
    /// Returns a shared reference to the object attributes.
    fn attrs(&self) -> Option<&Attributes<T>>;
    ///
    /// Returns the exlusive reference to the object attributes.
    fn attrs_mut(&mut self) -> Option<&mut Attributes<T>>;
}
///
/// Data associated with the object.
pub struct Attributes<T> {
    ///
    /// Object name.
    name: String,
    ///
    /// User-defined metadata.
    custom: T,
}
//
//
impl<T> Attributes<T> {
    ///
    /// Creates a new instance.
    pub fn new(name: String, custom: T) -> Self {
        Self { name, custom }
    }
    ///
    /// Returns a shared reference to the object name.
    pub fn name(&self) -> &str {
        &self.name
    }
    ///
    /// Returns a shared reference to the custom data of the object.
    pub fn custom(&self) -> &T {
        &self.custom
    }
    ///
    /// Returns the exclusive reference to the custom data of the object.
    pub fn custom_mut(&mut self) -> &mut T {
        &mut self.custom
    }
}
//
//
impl<T: Clone> Clone for Attributes<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            custom: self.custom.clone(),
        }
    }
}

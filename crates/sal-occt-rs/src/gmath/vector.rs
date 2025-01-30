use glam::{f64::DQuat, DVec3};
use sal_3dlib_core::gmath;
///
/// Displacment in 3-dimensional space.
///
/// # Examples
/// ```no_run
/// use core::f64::consts::FRAC_PI_2;
/// use sal_occt_rs::gmath::vector::Vector;
/// //
/// // create a new oX vector
/// let unit_x = Vector::new(1.0, 0.0, 0.0);
/// // transform it to oY by rotating 90 degrees around oZ
/// let unit_y = unit_x.rotate(Vector::unit_z(), FRAC_PI_2);
/// # //
/// # use sal_3dlib_core::gmath as core_gmath;
/// # //
/// # let target = core_gmath::Vector::from(Vector::unit_y());
/// # let result = core_gmath::Vector::from(unit_y);
/// # let tolerance = 10.0;
/// # for i in 0..=2 {
/// #     let t_val = (target[i] * tolerance).trunc();
/// #     let r_val = (result[i] * tolerance).trunc();
/// #     assert_eq!(t_val, r_val);
/// # }
/// ```
pub struct Vector(pub(crate) gmath::Vector<3>);
//
//
impl Vector {
    ///
    /// Returns the normalized vector oX.
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    ///
    /// Returns the normalized vector oY.
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    ///
    /// Returns the normalized vector oZ.
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
    ///
    /// Creates a vector from coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z].into())
    }
    ///
    /// Rotates vector `rad` radians around `axis`.
    ///
    /// # Examples
    /// Create oZ by rotating oX 90 degrees around oY:
    /// ```no_run
    /// use core::f64::consts::FRAC_PI_2;
    /// use sal_occt_rs::gmath::vector::Vector;
    /// //
    /// let unit_x = Vector::unit_x();
    /// let unit_z = unit_x.rotate(Vector::unit_y(), -FRAC_PI_2);
    /// # //
    /// # use sal_3dlib_core::gmath as core_gmath;
    /// # //
    /// # let target = core_gmath::Vector::from(Vector::unit_z());
    /// # let result = core_gmath::Vector::from(unit_z);
    /// # let tolerance = 10.0;
    /// # for i in 0..=2 {
    /// #     let t_val = (target[i] * tolerance).trunc();
    /// #     let r_val = (result[i] * tolerance).trunc();
    /// #     assert_eq!(t_val, r_val);
    /// # }
    /// ```
    pub fn rotate(self, axis: Self, rad: f64) -> Self {
        let axis = DVec3::from_array(*axis.0).normalize();
        let this = DVec3::from_array(*self.0);
        Self(
            DQuat::from_axis_angle(axis, rad)
                .mul_vec3(this)
                .to_array()
                .into(),
        )
    }
}
//
//
impl From<Vector> for gmath::Vector<3> {
    fn from(val: Vector) -> Self {
        val.0
    }
}

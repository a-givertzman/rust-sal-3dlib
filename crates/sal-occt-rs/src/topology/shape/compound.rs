//!
//! Compound implementation in terms of OCCT.
//!
//! It provides the final object - [Compound] - and its related trait implementations.
//
use super::{
    face::{Face, OcctFace},
    shell::{OcctShell, Shell},
    solid::{OcctSolid, Solid},
    vertex::OcctVertex,
};
use opencascade::primitives::{self, IntoShape};
pub use sal_3dlib_core::topology::shape::compound::Solids;
use sal_3dlib_core::{ops::boolean::volume, props::Center, topology::shape::compound};
use sal_sync::services::entity::error::str_err::StrErr;
///
/// Group of any type of topological objects.
///
/// For internal use only. It provides low-level implementation for [Compound].
#[derive(Clone)]
pub struct OcctCompound(pub(crate) primitives::Compound);
//
//
impl Center for OcctCompound {
    type Output = OcctVertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        OcctVertex(primitives::Vertex::new(point))
    }
}
//
//
impl Solids<OcctSolid> for OcctCompound {
    fn solids(&self) -> impl IntoIterator<Item = OcctSolid> {
        self.0.as_ref().into_shape().solids().map(OcctSolid)
    }
}
//
//
impl volume::AlgoMakerVolume<OcctFace, OcctShell, OcctSolid> for OcctCompound {
    type Error = StrErr;
    //
    //
    fn build<'a>(
        fs: impl IntoIterator<Item = &'a OcctFace>,
        ls: impl IntoIterator<Item = &'a OcctShell>,
        ds: impl IntoIterator<Item = &'a OcctSolid>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        OcctFace: 'a,
        OcctShell: 'a,
        OcctSolid: 'a,
    {
        primitives::Compound::volume(
            fs.into_iter().map(|face| face.0.as_ref()),
            ls.into_iter().map(|shell| shell.0.as_ref()),
            ds.into_iter().map(|solid| solid.0.as_ref()),
        )
        .map_or_else(
            |err| {
                Err(StrErr(format!(
                    "Compound.solidify | Failed to volume args: {}",
                    err
                )))
            },
            |compound| Ok(Self(compound)),
        )
    }
}
///
/// Group of any type of topological objects.
pub type Compound<T> = compound::Compound<3, OcctCompound, T>;
///
/// Algorithm to volume together three sets of the objects in 3-dementional space.
///
/// The Volume Maker algorithm has been designed for building the elementary volumes (solids)
/// from a set of connected, intersecting, or nested shapes (faces, shells, and solids).  
///
/// The algorithm can also be useful for splitting objects into parts,
/// or constructing new solid(s) from set of intersecting or connected faces or shells.  
///
/// Note that the algorithm creates only closed solids.
pub trait AlgoMakerVolume<T>: volume::AlgoMakerVolume<Face<T>, Shell<T>, Solid<T>> {
    ///
    /// Performs the algorithm and wraps the result solid(s) in a compound.
    ///
    /// See [`Volume Maker Algorithm`] for details.
    ///
    /// # Examples
    /// Consider the method that creates a cube from the six planes:
    /// ```no_run
    /// use sal_occt_rs::topology::shape::{
    ///     face::Face,
    ///     compound::{Compound, AlgoMakerVolume}
    /// };
    /// use sal_sync::services::entity::error::str_err::StrErr;
    /// //
    /// fn create_cube_from_six_planes<T>(planes: [&Face<T>; 6]) -> Result<Compound<T>, StrErr> {
    ///     // shells and solids are not involved,
    ///     // so the algorithm takes them as empty arrays
    ///     Compound::build(planes, [], [])
    /// }
    /// ```
    ///
    /// For given [Solid], this algorithm allows to get elements splitted by a [Face]
    /// (or devided into top and bottom parts) relative to the splitter face:
    /// ```no_run
    /// use sal_occt_rs::topology::shape::{
    ///     face::Face,
    ///     compound::{Compound, AlgoMakerVolume},
    ///     solid::Solid,
    /// };
    /// use sal_sync::services::entity::error::str_err::StrErr;
    /// //
    /// fn top_bottom_parts<T>(model: Solid<T>, splitter: Face<T>) -> Result<Compound<T>, StrErr> {
    ///     // again, no shells are involved - 2nd argument is empty
    ///     Compound::build([&splitter], [], [&model])
    /// }
    /// ```
    ///
    /// But if [Face] is too simple, a more complex object like [Shell] may need to be used for splitting:
    /// ```no_run
    /// use sal_occt_rs::topology::shape::{
    ///     compound::{Compound, AlgoMakerVolume},
    ///     shell::Shell,
    ///     solid::Solid,
    /// };
    /// use sal_sync::services::entity::error::str_err::StrErr;
    /// //
    /// fn split_by_waved_surface<T>(model: Solid<T>, splitter: Shell<T>) -> Result<Compound<T>, StrErr> {
    ///     // note that in this case there are no faces involved
    ///     Compound::build([], [&splitter], [&model])
    /// }
    /// ```
    ///
    /// [`Volume Maker Algorithm`]: https://dev.opencascade.org/doc/overview/html/specification__boolean_operations.html#specification__boolean_10b
    fn build<'a>(
        fs: impl IntoIterator<Item = &'a Face<T>>,
        ls: impl IntoIterator<Item = &'a Shell<T>>,
        ds: impl IntoIterator<Item = &'a Solid<T>>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Face<T>: 'a,
        Shell<T>: 'a,
        Solid<T>: 'a,
    {
        <Self as volume::AlgoMakerVolume<Face<T>, Shell<T>, Solid<T>>>::build(fs, ls, ds)
    }
}
//
//
impl<T> AlgoMakerVolume<T> for Compound<T> {}

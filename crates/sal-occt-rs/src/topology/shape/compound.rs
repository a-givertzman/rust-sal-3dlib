use super::*;
use primitives::IntoShape;
use sal_3dlib_core::ops::AlgoMakerVolume;
use sal_3dlib_core::props::Center;
use sal_3dlib_core::topology::compound::Solids;
use sal_sync::services::entity::error::str_err::StrErr;
//
//
impl Center for Compound {
    type Output = Vertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        Vertex(primitives::Vertex::new(point))
    }
}
//
//
impl AlgoMakerVolume<Face, Shell, Solid> for Compound {
    type Error = StrErr;
    //
    //
    fn build<'a>(
        fs: impl IntoIterator<Item = &'a Face>,
        ls: impl IntoIterator<Item = &'a Shell>,
        ds: impl IntoIterator<Item = &'a Solid>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Face: 'a,
        Shell: 'a,
        Solid: 'a,
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
//
//
impl Solids<Solid> for Compound {
    fn solids(&self) -> impl IntoIterator<Item = Solid> {
        self.0.as_ref().into_shape().solids().map(Solid)
    }
}

crates/opencascade-sys/include/wrapper.hxx
```cpp
inline void shape_list_append_shape(TopTools_ListOfShape &list, const TopoDS_Shape &shape) { list.Append(shape); }

inline void SetRunParallel_BRepAlgoAPI_Common(BRepAlgoAPI_Common &theBOP, bool theFlag) {
  theBOP.SetRunParallel(theFlag);
}
inline void SetUseOBB_BRepAlgoAPI_Common(BRepAlgoAPI_Common &theBOP, bool theFlag) { theBOP.SetUseOBB(theFlag); }
inline void SetFuzzyValue_BRepAlgoAPI_Common(BRepAlgoAPI_Common &theBOP, double theFuzz) {
  theBOP.SetFuzzyValue(theFuzz);
}

inline bool HasErrors_BRepAlgoAPI_Common(const BRepAlgoAPI_Common &theBOP) { return theBOP.HasErrors(); }

inline const TopoDS_Shape &BOPAlgo_MakerVolume_Shape(const BOPAlgo_MakerVolume &aMV) { return aMV.Shape(); }
```

crates/opencascade-sys/src/lib.rs
```rust
pub mod ffi {
    #[derive(Debug)]
    #[repr(u32)]
    pub enum BOPAlgo_Operation {
        BOPAlgo_COMMON,
        BOPAlgo_FUSE,
        BOPAlgo_CUT,
        BOPAlgo_CUT21,
        BOPAlgo_SECTION,
        BOPAlgo_UNKNOWN,
    }
/* ... */
        pub fn shape_list_append_shape(list: Pin<&mut TopTools_ListOfShape>, face: &TopoDS_Shape);
/* ... */
        type BOPAlgo_MakerVolume;

        #[cxx_name = "construct_unique"]
        pub fn BOPAlgo_MakerVolume_ctor() -> UniquePtr<BOPAlgo_MakerVolume>;
        pub fn SetArguments(self: Pin<&mut BOPAlgo_MakerVolume>, the_ls: &TopTools_ListOfShape);
        pub fn Perform(self: Pin<&mut BOPAlgo_MakerVolume>, the_range: &Message_ProgressRange);
        pub fn BOPAlgo_MakerVolume_Shape(theMV: &BOPAlgo_MakerVolume) -> &TopoDS_Shape;
/* ... */
        type BOPAlgo_Operation;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Common_ctor() -> UniquePtr<BRepAlgoAPI_Common>;
/* ... */
        /// Obsolete.
        #[rust_name = "BRepAlgoAPI_Common_ctor2"]
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Common>, the_range: &Message_ProgressRange);
        pub fn SetTools(self: Pin<&mut BRepAlgoAPI_Common>, the_ls: &TopTools_ListOfShape);
        pub fn SetArguments(self: Pin<&mut BRepAlgoAPI_Common>, the_ls: &TopTools_ListOfShape);
        pub fn HasErrors_BRepAlgoAPI_Common(the_bop: &BRepAlgoAPI_Common) -> bool;
        pub fn SetFuzzyValue_BRepAlgoAPI_Common(
            the_bop: Pin<&mut BRepAlgoAPI_Common>,
            the_fuzz: f64,
        );
        pub fn SetRunParallel_BRepAlgoAPI_Common(
            the_bop: Pin<&mut BRepAlgoAPI_Common>,
            the_flag: bool,
        );
        pub fn SetUseOBB_BRepAlgoAPI_Common(
            the_bop: Pin<&mut BRepAlgoAPI_Common>,
            the_use_obb: bool,
        );
        pub fn SetGlue(self: Pin<&mut BRepAlgoAPI_Common>, glue: BOPAlgo_GlueEnum);
/* ... */
}
```

crates/opencascade/src/primitives/shape.rs
```rust
use crate::angle::Angle;
/* ... before/after */
        // let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor(&self.inner, &other.inner);
        let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor2(&self.inner, &other.inner);
/* ... */
impl Shape {
  pub fn intersect_with_params(
          &self,
          other: &Shape,
          parallel: bool,
          fuzzy: f64,
          obb: bool,
          glue: u8,
    ) -> BooleanShape {
        let mut common_operation = ffi::BRepAlgoAPI_Common_ctor();

        // set tools
        let mut tools = ffi::new_list_of_shape();
        ffi::shape_list_append_shape(tools.pin_mut(), &self.inner);
        common_operation.pin_mut().SetTools(&tools);

        // set arguments
        let mut arguments = ffi::new_list_of_shape();
        ffi::shape_list_append_shape(arguments.pin_mut(), &other.inner);
        common_operation.pin_mut().SetArguments(&arguments);

        // set additional options
        ffi::SetFuzzyValue_BRepAlgoAPI_Common(common_operation.pin_mut(), fuzzy);
        ffi::SetRunParallel_BRepAlgoAPI_Common(common_operation.pin_mut(), parallel);
        ffi::SetUseOBB_BRepAlgoAPI_Common(common_operation.pin_mut(), obb);
        match glue {
            2 => common_operation.pin_mut().SetGlue(ffi::BOPAlgo_GlueEnum::BOPAlgo_GlueFull),
            1 => common_operation.pin_mut().SetGlue(ffi::BOPAlgo_GlueEnum::BOPAlgo_GlueShift),
            _ => common_operation.pin_mut().SetGlue(ffi::BOPAlgo_GlueEnum::BOPAlgo_GlueOff),
        }

        // perform operation
        common_operation.pin_mut().Build(&ffi::Message_ProgressRange_ctor());

        // if ffi::HasErrors_BRepAlgoAPI_Common(&common_operation) {
        //     panic!("something went wrong");
        // }

        // get result edges
        let edge_list = common_operation.pin_mut().SectionEdges();
        let vec = ffi::shape_list_to_vector(edge_list);
        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::TopoDS_cast_to_edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        // get result shape
        let shape = Self::from_shape(common_operation.pin_mut().Shape());

        BooleanShape { shape, new_edges }
    }

    pub fn rotate(mut self, rotation_axis: DVec3, angle: Angle) -> Self {
        // create general transformation object
        let mut transform = ffi::new_transform();

        // apply rotation to transformation
        let rotation_axis_vec =
            ffi::gp_Ax1_ctor(&make_point(DVec3::ZERO), &make_dir(rotation_axis));
        transform.pin_mut().SetRotation(&rotation_axis_vec, angle.radians());

        // get result location
        let location = ffi::TopLoc_Location_from_transform(&transform);

        // apply transformation to shape
        self.inner.pin_mut().translate(&location, false);
        self
    }
/* ... */
}
/* ... */
```

crates/opencascade/src/primitives/solid.rs
```rust
/* ... before/after */
        // let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor(inner_shape, other_inner_shape);
        let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor2(inner_shape, other_inner_shape);
/* ... */
impl Solid {
    pub fn volume<'a, T>(
        shells_as_shapes: impl IntoIterator<Item = &'a T>,
        _avoid_internal_shapes: bool,
    ) -> Self
    where
        T: AsRef<Shape> + 'a,
    {
        // create Volume maker
        let mut maker = ffi::BOPAlgo_MakerVolume_ctor();

        // set shells to make solid from
        let mut arguments = ffi::new_list_of_shape();
        for shape in shells_as_shapes {
            ffi::shape_list_append_shape(arguments.pin_mut(), &shape.as_ref().inner);
        }
        maker.pin_mut().SetArguments(&arguments);

        // perform the opearation
        maker.pin_mut().Perform(&ffi::Message_ProgressRange_ctor());
        // cast result to solid according to doc
        let genaral_shape = ffi::BOPAlgo_MakerVolume_Shape(&maker);
        let solid = ffi::TopoDS_cast_to_solid(genaral_shape);
        Solid::from_solid(solid)
    }
/* ... */
}
```
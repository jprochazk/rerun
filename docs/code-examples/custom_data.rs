//! Demonstrates how to implement custom archetypes and components, and extend existing ones.

use rerun::{
    archetypes::Points3D,
    datatypes::Float32,
    demo_util::grid,
    external::{arrow2, glam, re_types},
    AsComponents, ComponentBatch, ComponentName, Loggable, MaybeOwnedComponentBatch,
    NamedIndicatorComponent, RecordingStreamBuilder,
};

// ---

/// A custom [component bundle] that extends Rerun's builtin [`Points3D`] archetype with extra
/// [`rerun::Component`]s.
///
/// [component bundle]: [`AsComponents`]
struct CustomPoints3D {
    points3d: Points3D,
    confidences: Option<Vec<Confidence>>,
}

impl AsComponents for CustomPoints3D {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        let indicator = NamedIndicatorComponent("user.CustomPoints3DIndicator".into());
        self.points3d
            .as_component_batches()
            .into_iter()
            .chain(
                [
                    Some(indicator.to_batch()),
                    self.confidences
                        .as_ref()
                        .map(|v| (v as &dyn ComponentBatch).into()),
                ]
                .into_iter()
                .flatten(),
            )
            .collect()
    }
}

// ---

/// A custom [`rerun::Component`] that is backed by a builtin [`Float32`] scalar [`rerun::Datatype`].
#[derive(Debug, Clone, Copy)]
struct Confidence(Float32);

impl From<f32> for Confidence {
    fn from(v: f32) -> Self {
        Self(Float32(v))
    }
}

impl Loggable for Confidence {
    type Name = ComponentName;

    fn name() -> Self::Name {
        "user.Confidence".into()
    }

    fn arrow_datatype() -> arrow2::datatypes::DataType {
        Float32::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types::SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: 'a,
    {
        Float32::to_arrow_opt(data.into_iter().map(|opt| opt.map(Into::into).map(|c| c.0)))
    }
}

// ---

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rec, storage) = RecordingStreamBuilder::new("rerun_example_custom_data").memory()?;

    rec.log(
        "left/my_confident_point_cloud",
        &CustomPoints3D {
            points3d: Points3D::new(grid(glam::Vec3::splat(-5.0), glam::Vec3::splat(5.0), 3)),
            confidences: Some(vec![42f32.into()]),
        },
    )?;

    rec.log(
        "right/my_polarized_point_cloud",
        &CustomPoints3D {
            points3d: Points3D::new(grid(glam::Vec3::splat(-5.0), glam::Vec3::splat(5.0), 3)),
            confidences: Some((0..27).map(|i| i as f32).map(Into::into).collect()),
        },
    )?;

    rerun::native_viewer::show(storage.take())?;
    Ok(())
}

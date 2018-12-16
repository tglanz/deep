use math::{Shape};

type TensorId = u16;

///
/// A reference to some arbitrary tensor.
///
/// Open questions
///   - Can the shape really be optional? I think it does.. Why? Some examples
///     1. Batch size shouldn't be fixed
///     2. Some operations don't really need the specifc shape. See gemm broadcasts and such..
#[derive(Debug)]
pub struct TensorDescriptor {
    tensor_id: TensorId,
    shape: Option<Shape>,
}

impl TensorDescriptor {
    pub fn create(tensor_id: TensorId) -> Self {
        TensorDescriptor {
            tensor_id,
            shape: None,
        }
    }

    pub fn with_shape<TIntoShape: Into<Shape>>(mut self, into_shape: TIntoShape) -> Self {
        self.shape = Some(into_shape.into());
        self
    }
}
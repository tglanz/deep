use {Shape};

pub type TensorId = u16;

///
/// A reference to some arbitrary tensor.
///
/// Open questions
///   - Can the shape really be optional? I think it does.. Why? Some examples
///     1. Batch size shouldn't be fixed
///     2. Some operations don't really need the specifc shape. See gemm broadcasts and such..
///     3. Fuck it, now its mandatory
#[derive(Serialize, Deserialize, Debug)]
pub struct TensorDescriptor {
    tensor_id: TensorId,
    shape: Shape,
}

impl TensorDescriptor {
    pub fn get_tensor_id(&self) -> TensorId {
        self.tensor_id
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn create<I: Into<Shape>>(tensor_id: TensorId, shape: I) -> Self {
        TensorDescriptor {
            tensor_id,
            shape: shape.into(),
        }
    }
}

impl Clone for TensorDescriptor {
    fn clone(&self) -> Self {
        TensorDescriptor::create(self.tensor_id, self.shape.clone())
    }
}
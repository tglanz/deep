use math::{Tensor};
use operations::{Operation};

pub type NodeId = u16;
pub type Nodes = Vec<Node>;

#[derive(Debug)]
pub enum Node {
    InputNode(NodeId, Tensor<u16>),
    ParameterNode(NodeId, Tensor<u16>),
    OperationNode(NodeId, Operation),
}

/* serde custom serialization for node...
impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
        {
            match self {
                Node::InputNode(id, tensor) => {
                    let mut state = serializer.serialize_struct("node", 3)?;
                    state.serialize_field("type", "input")?;
                    state.serialize_field("node_id", &id)?;
                    state.serialize_field("tensor", &tensor.get_shape())?;
                    state.end()
                }
                Node::ParameterNode(id, tensor) => {
                    let mut state = serializer.serialize_struct("node", 3)?;
                    state.serialize_field("type", "parameter")?;
                    state.serialize_field("id", &id)?;
                    state.serialize_field("tensor", &tensor.get_shape())?;
                    state.end()
                }
                Node::OperationNode(id, operation) => {
                    let mut state = serializer.serialize_struct("node", 3)?;
                    state.serialize_field("type", "operation")?;
                    state.serialize_field("id", &id)?;
                    state.serialize_field("operation", &operation)?;
                    state.end()
                }
            }
        }
}
*/
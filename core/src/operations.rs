pub type Operand = u8;

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    Add(add::Attributes),
    Dot(dot::Attributes),
    LeakyRelu(leaky_relu::Attributes),
}

pub mod add {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Attributes;

    pub fn operation() -> Operation {
        Operation::Add(Attributes {

        })
    }

    pub mod operands {
        pub const INPUT: super::Operand = 0;
    }
}

pub mod dot {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Attributes;

    pub fn operation() -> Operation {
        Operation::Dot(Attributes {

        })
    }
    
    pub mod operands {
        pub const INPUT: super::Operand = 0;
        pub const WEIGHTS: super::Operand = 1;
    }
}

pub mod leaky_relu {

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Attributes {
        scalar: f32
    }

    pub fn operation(scalar: f32) -> Operation {
        Operation::LeakyRelu(Attributes {
            scalar
        })
    }

    pub mod operands {
        pub const INPUT: super::Operand = 0;
    }
}

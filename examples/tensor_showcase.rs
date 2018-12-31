extern crate deep;

use deep::core::Tensor;

fn iterate_tensor(tensor: &Tensor<u16>) {
    println!("iterating tensor");
    for item in tensor {
        println!(" - {:?}", item);
    }
}

fn access_tensor(tensor: &mut Tensor<u16>) {
    println!("mutation at 0");
    println!(" - initially {:?}", tensor[0]);
    tensor[0] = 322;
    println!(" - now       {:?}, expected 322", tensor[0]);

    println!("mutation at (1,)");
    println!(" - initially {:?}", tensor[(1,)]);
    tensor[(1,)] = 41;
    println!(" - now       {:?}, expected 41", tensor[(1,)]);

    println!("mutation at (1, 1)");
    println!(" - initially {:?}", tensor[(1, 1)]);
    tensor[(1, 1)] = 623;
    println!(" - now       {:?}, expected 623", tensor[(1, 1)]);

    println!("mutation at vec![1, 0, 2]");
    println!(" - initially {:?}", tensor[&vec![1, 0, 2]]);
    tensor[&vec![1, 0, 2]] = 8787;
    println!(" - now       {:?}, expected 8787", tensor[&vec![1, 0, 2]]);
}

fn main() {
    let mut tensor: Tensor<u16> = Tensor::default((2, 2, 3));
    access_tensor(&mut tensor);
    iterate_tensor(&tensor);
}
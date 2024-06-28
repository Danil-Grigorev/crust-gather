use kube::CustomResourceExt;

use crate::gather::crd::Gather;

fn main() {
    print!("{}", serde_yaml::to_string(&Gather::crd()).unwrap())
}

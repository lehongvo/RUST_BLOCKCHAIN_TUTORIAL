pub mod graden;

use crate::graden::management::Tree;
use crate::graden::vegetables::Asparagus;

fn main() {
    let plant: Asparagus = Asparagus::create_tree();
    plant.get_tree();

    let tree: Tree = Tree::store(plant);
    tree.get_info();
}

mod graden;

use graden::vegetables::Asparagus;

fn main() {
    let plant: Asparagus = Asparagus::create_tree();
    plant.get_tree();
}
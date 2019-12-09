// calculate fuel required for a module - take its mass, divide by three, round down, and subtract 2
fn fuel(mass: u32) -> u32 {
  mass / 3 - 2
}

// given a list of masses, sum the fuel required
fn sum_fuel(masses: &[u32]) -> u32 {
  let mut fuel_sum = 0;

  // sum the total fuel for all of the modules
  for mass in masses.iter () {
    fuel_sum += fuel(*mass);
  }

  fuel_sum
}

fn main() {
  let masses = [136583, 77036, 109200, 142168, 74357, 146941, 129306, 98180, 105195, 129127, 135956, 116070, 89198, 51306, 144552, 109900, 56658, 52478, 115147, 63882, 70342, 98678, 107384, 135359, 87237, 84469, 106477, 104645, 77066, 74143, 76537, 140547, 68128, 116624, 148407, 78276, 117623, 96019, 75825, 75010, 98644, 119641, 139736, 104452, 72599, 63017, 59648, 126163, 69629, 79080, 92195, 58221, 134276, 80301, 89870, 146799, 145702, 138367, 131977, 56781, 85326, 138115, 70241, 60454, 76934, 119321, 93493, 123047, 149941, 141729, 70141, 134525, 93312, 92043, 79582, 115959, 51058, 94686, 70749, 99408, 118560, 95821, 58995, 94906, 98421, 118673, 83575, 83434, 63884, 70575, 134177, 116233, 113954, 52829, 123860, 128020, 126718, 144463, 140192, 143461];

  // print out sum of fuel for all modules
  println!("Total fuel required: {}", sum_fuel(&masses));
}
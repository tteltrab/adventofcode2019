// part 1 - calculate fuel required for a module - take its mass, divide by three, round down, and subtract 2
fn fuel_1(mass: i32) -> i32 {
  mass / 3 - 2
}

// part 2 - same as part 1, but including mass of fuel as additional mass until ~0 is reached
fn fuel_2(mass: i32) -> i32 {
  let current_fuel = fuel_1(mass);

  if current_fuel < 0 {
    return 0;
  }

  return current_fuel + fuel_2(current_fuel);
}

// given a list of masses, sum the total fuel required using a given fuel calculator function
fn sum_fuel(masses: &[i32], fuel_calculator: fn(i32) -> i32) -> i32 {
  let mut fuel_sum = 0;

  // sum the total fuel for all of the modules
  for mass in masses.iter () {
    // use the given fuel_calculator function to determine fuel used for this specific module mass
    fuel_sum += fuel_calculator(*mass);
  }

  fuel_sum
}

fn main() {
  let masses = [136583, 77036, 109200, 142168, 74357, 146941, 129306, 98180, 105195, 129127, 135956, 116070, 89198, 51306, 144552, 109900, 56658, 52478, 115147, 63882, 70342, 98678, 107384, 135359, 87237, 84469, 106477, 104645, 77066, 74143, 76537, 140547, 68128, 116624, 148407, 78276, 117623, 96019, 75825, 75010, 98644, 119641, 139736, 104452, 72599, 63017, 59648, 126163, 69629, 79080, 92195, 58221, 134276, 80301, 89870, 146799, 145702, 138367, 131977, 56781, 85326, 138115, 70241, 60454, 76934, 119321, 93493, 123047, 149941, 141729, 70141, 134525, 93312, 92043, 79582, 115959, 51058, 94686, 70749, 99408, 118560, 95821, 58995, 94906, 98421, 118673, 83575, 83434, 63884, 70575, 134177, 116233, 113954, 52829, 123860, 128020, 126718, 144463, 140192, 143461];

  // part 1 - print out sum of fuel for all modules 
  println!("Total fuel required: {}", sum_fuel(&masses, fuel_1));

  // part 2 - print out sum of fuel required for all modules + fuel mass
  println!("Total fuel required: {}", sum_fuel(&masses, fuel_2));
}

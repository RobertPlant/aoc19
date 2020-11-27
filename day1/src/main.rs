mod input;

fn get_fuel_requirement(mass: u32) -> u32 {
    return (mass / 3) - 2;
}

fn main() {
    let input_data = input::get_input();
    let mut cumulative_total :u32 = 0;

    for datum in input_data {
        cumulative_total += get_fuel_requirement(datum);
    }

    println!("Total: {}", cumulative_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_12() {
        assert_eq!(get_fuel_requirement(12), 2)
    }

    #[test]
    fn test_mass_14() {
        assert_eq!(get_fuel_requirement(14), 2)
    }

    #[test]
    fn test_mass_1969() {
        assert_eq!(get_fuel_requirement(1969), 654)
    }

    #[test]
    fn test_mass_100756() {
        assert_eq!(get_fuel_requirement(100756), 33583)
    }
}

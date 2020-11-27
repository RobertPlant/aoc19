mod input;

// Since its unsigned we cant go under zero, therefore we have to check before
// performing the minus two calculation. If it were to go under zero it should
// be set to zero.
fn get_fuel_requirement(mass: u32) -> u32 {
    let mut calculation = mass / 3;

    if calculation >= 2 {
        calculation -= 2
    } else {
        calculation = 0;
    }

    calculation
}

// Recursively loop over the fuel requirements, determine in the additional fuel
// required to launch the fuel also requires more fuel.
fn get_fuel_requirement_with_recursion(mass: u32) -> u32 {
    let mut requirement = get_fuel_requirement(mass);

    if requirement > 0 {
        requirement += get_fuel_requirement_with_recursion(requirement);
    }

    requirement
}

fn main() {
    let input_data = input::get_input();
    let mut cumulative_total: u32 = 0;
    let mut cumulative_total_recursion: u32 = 0;

    for datum in input_data {
        cumulative_total += get_fuel_requirement(datum);
        cumulative_total_recursion += get_fuel_requirement_with_recursion(datum);
    }

    println!("Total: {}", cumulative_total);
    println!("Total with fuel recalculated: {}", cumulative_total_recursion);
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

    #[test]
    fn test_mass_100756_with_recursion() {
        assert_eq!(get_fuel_requirement_with_recursion(100756), 50346)
    }
}

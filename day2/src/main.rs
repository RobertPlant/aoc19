mod input;

const ADD: usize = 1;
const MULTIPLY: usize = 2;
const HALT: usize = 99;

fn compute(input: Vec<usize>, opcode_offset: usize) -> Vec<usize> {
    let mut updated_input = input.clone();

    let operation = input[opcode_offset];

    if operation == HALT {
        return updated_input;
    }

    let a_pointer = input[opcode_offset + 1];
    let b_pointer = input[opcode_offset + 2];
    let output_pointer = input[opcode_offset + 3];

    if operation == ADD {
        updated_input[output_pointer] = input[a_pointer] + input[b_pointer];
    } else if operation == MULTIPLY {
        updated_input[output_pointer] = input[a_pointer] * input[b_pointer];
    }

    updated_input = compute(updated_input, opcode_offset + 4);

    updated_input
}

fn main() {
    let input_data = input::get_input();

    println!("Output: {:?}", compute(input_data, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(compute(vec![1, 0, 0, 0, 99], 0), vec![2, 0, 0, 0, 99])
    }

    #[test]
    fn test_multiply() {
        assert_eq!(compute(vec![2, 3, 0, 3, 99], 0), vec![2, 3, 0, 6, 99])
    }

    #[test]
    fn test_multiply_with_halt() {
        assert_eq!(
            compute(vec![2, 4, 4, 5, 99, 0], 0),
            vec![2, 4, 4, 5, 99, 9801]
        )
    }

    #[test]
    fn test_multi_level() {
        assert_eq!(
            compute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 0),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        )
    }
}

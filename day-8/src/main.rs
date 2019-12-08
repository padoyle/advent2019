use std::collections::HashMap;

static INPUT_STR: &str = include_str!("../input.txt");
const BLACK: usize = 0;
const WHITE: usize = 1;

fn main() {
    println!("P1 Result: {}", p1(INPUT_STR, (25, 6)));
    println!("P2 Result:");
    p2(INPUT_STR, (25, 6));
}

fn p1(input: &str, dims: (usize, usize)) -> usize {
    let layer_value_map = get_layer_values(input, dims);

    let mut min_zeroes = std::usize::MAX;
    let mut min_index = 0;
    for (index, layer_values) in layer_value_map.iter() {
        let zero_count = layer_values[0];
        min_zeroes = std::cmp::min(min_zeroes, zero_count);
        if zero_count == min_zeroes {
            min_index = *index;
        }
    }

    let ones = layer_value_map[&min_index][1];
    let twos = layer_value_map[&min_index][2];

    ones * twos
}

fn p2(input: &str, dims: (usize, usize)) {
    let layer_size = dims.0 * dims.1;
    let mut layers = split_into(input, layer_size);
    layers.reverse();

    let mut final_image = vec![2; layer_size];
    for pixel_idx in 0..layer_size {
        let mut value = 2;
        for layer_idx in 0..layers.len() {
            let pixel_value = layers[layer_idx]
                .chars()
                .nth(pixel_idx)
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            if pixel_value == BLACK {
                value = BLACK;
            } else if pixel_value == WHITE {
                value = WHITE;
            }
        }
        final_image[pixel_idx] = value;
    }

    for y in 0..dims.1 {
        for x in 0..dims.0 {
            let print_value = match final_image[y * dims.0 + x] {
                0 => " ",
                1 => "|",
                _ => " ",
            };
            print!("{}", print_value);
        }
        println!();
    }
}

type LayerValues = HashMap<usize, Vec<usize>>;

fn get_layer_values(input: &str, dims: (usize, usize)) -> LayerValues {
    let mut values: LayerValues = HashMap::new();

    let layers = split_into(input, dims.0 * dims.1);
    for layer_idx in 0..layers.len() {
        let layer = layers[layer_idx];
        let mut digit_counts = vec![0; 3];

        for digit in layer.chars() {
            let digit = digit.to_digit(10).expect("Not a valid digit") as usize;
            digit_counts[digit] += 1;
        }

        values.insert(layer_idx, digit_counts);
    }

    values
}

fn split_into<'a>(input: &'a str, count: usize) -> Vec<&'a str> {
    let mut layers = Vec::new();
    let mut ptr = 0;

    while ptr + count <= input.len() {
        layers.push(&input[ptr..ptr + count]);
        ptr += count;
    }

    layers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_helper() {
        assert_eq!(split_into("12345678", 2), vec!["12", "34", "56", "78"]);
    }

    #[test]
    fn example() {
        assert_eq!(p1("121121010021000122010020", (3, 2)), 8);
    }
}

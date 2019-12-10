fn main() {
    let mut data: Vec<u32> = include_str!("input")
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();

    let (width, height) = (25, 6);
    let layer_size = width * height;
    let mut start_idx = 0;
    let mut end_idx = layer_size;

    let mut rendered = vec![2; layer_size];
    while start_idx < data.len() {
        let slice = &data[start_idx..end_idx];
        for (i, val) in slice.iter().enumerate() {
            if rendered[i] == 2 {
                rendered[i] = *val;
            }
        }

        start_idx += layer_size;
        end_idx += layer_size;
    }

    let mut row_start_idx = 0;
    let mut row_end_idx = width;

    while row_start_idx < rendered.len() {
        let slice = &rendered[row_start_idx..row_end_idx];
        for i in slice {
            print!("{}", i);
            //if *i == 0 { print!("X") } else { print!(" ") }
        }
        println!("");

        row_start_idx += width;
        row_end_idx += width;
    }

    //println!("{:?}", rendered);

    /* Part 1
    let mut fewest_ones = std::u32::MAX;
    let mut product = 0;

    while start_idx < data.len() {
        let slice = &data[start_idx..end_idx];
        let mut num0 = 0;
        let mut num1 = 0;
        let mut num2 = 0;
        for val in slice {
            match val {
                0 => num0 += 1,
                1 => num1 += 1,
                2 => num2 += 1,
                _ => { /* noop */ },
            }
        }

        if num0 < fewest_ones {
            fewest_ones = num0;
            product = num1 * num2;
        }

        start_idx += layer_size;
        end_idx += layer_size;
    }

    println!("{}", product);
    */
}

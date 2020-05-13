pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_pts = vec![];
    for (row_num, row) in input.iter().enumerate() {
        for (col_num, value) in row.iter().enumerate() {
            // safe for unwrap() because inside for loop
            if value == row.iter().max().unwrap() {
                if !input.iter().any(|row| { row[col_num] < *value }) {
                    saddle_pts.push((row_num, col_num));
                }               
            }
        }
    }
    saddle_pts
}
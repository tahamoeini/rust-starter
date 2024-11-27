pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut row_result = String::new();
        for (j, cell) in row.chars().enumerate() {
            if cell == '*' {
                row_result.push('*');
            } else {
                let mut count = 0;
                let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
                for (dx, dy) in offsets.iter() {
                    let new_i = i as i32 + dx;
                    let new_j = j as i32 + dy;
                    if new_i >= 0
                        && new_i < minefield.len() as i32
                        && new_j >= 0
                        && (new_j as usize) < minefield[new_i as usize].len()
                    {
                        if minefield[new_i as usize].chars().nth(new_j as usize).unwrap() == '*' {
                            count += 1;
                        }
                    }
                }
                row_result.push_str(&count.to_string());
            }
        }
        result.push(row_result);
    }
    result
}

pub fn main() {
    let minefield = vec![
        " *   **",
        " **  * ",
        "*   *  ",
        " **  * ",
        "*   *  ",
        " *   **",
        " **  * ",
    ];
    let result = annotate(&minefield);
    for row in result {
        println!("{}", row);
    }
}
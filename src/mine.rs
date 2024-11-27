pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    minefield.iter().enumerate().map(|(i, row)| {
        row.chars().enumerate().map(|(j, cell)| {
            if cell == '*' {
                '*'.to_string()
            } else {
                let count = offsets.iter().filter(|&&(dx, dy)| {
                    let new_i = i as i32 + dx;
                    let new_j = j as i32 + dy;
                    new_i >= 0 && new_i < minefield.len() as i32 && new_j >= 0 && (new_j as usize) < minefield[new_i as usize].len() && minefield[new_i as usize].chars().nth(new_j as usize) == Some('*')
                }).count();
                if count > 0 { count.to_string() } else { " ".to_string() }
            }
        }).collect()
    }).collect()
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
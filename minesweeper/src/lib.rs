pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<String> = Vec::new();
    for (row_index, row) in minefield.iter().enumerate() {
        let mut row_string = String::new();
        for (col_index, _) in row.as_bytes().iter().enumerate() {
            let point = Point::new(row_index, col_index);
            if is_mine(minefield, &point) {
                row_string.push('*');
            } else {
                let surrounding_mines = count_surrounding_mines(minefield, &point);
                if surrounding_mines > 0 {
                    row_string.push_str(&surrounding_mines.to_string());
                } else {
                    row_string.push(' ');
                }
            }
        }
        result.push(row_string);
    }
    result
}
#[derive(Debug, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn is_mine(matrix: &[&str], point: &Point) -> bool {
    // get the element at the given row and column, if the index is out of bounds, return 0

    // if the element is a mine, return 1, otherwise return 0

    match matrix.get(point.row) {
        Some(row_string) => match row_string.as_bytes().get(point.col) {
            Some(b'*') => true,
            _ => false,
        },
        None => false,
    }
}

fn count_surrounding_mines(matrix: &[&str], point: &Point) -> u8 {
    let mut counter: u8 = 0;

    for rows in point.row as i32 - 1..=point.row as i32 + 1 {
        for cols in point.col as i32 - 1..=point.col as i32 + 1 {
            let surrounding_point = Point::new(rows as usize, cols as usize);
            if surrounding_point == *point {
                continue;
            }
            if is_mine(matrix, &surrounding_point) {
                counter += 1;
            }
        }
    }
    counter
}

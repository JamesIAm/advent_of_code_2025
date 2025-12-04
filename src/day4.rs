use crate::file_utils::fetch_input;

const DAY: i32 = 4;
const TEST_ANSWER: usize = 43;

pub fn main() {
    let test_contents = fetch_input(DAY, true);
    let test_answer = solve_day(test_contents);
    assert_eq!(test_answer, TEST_ANSWER);
    let real_contents = fetch_input(DAY, false);
    let real_answer = solve_day(real_contents);
    println!("{}", real_answer)
}

const XY_ADJUSTMENTS: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];
fn solve_day(contents: String) -> usize {
    let lines: Vec<Vec<String>> = contents
        .split("\r\n")
        .map(|x| {
            x.chars()
                .map(|char| char.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let width = lines[0].len();
    let height = lines.len();
    println!("Width {} height {}", width, height);
    let mut grid: Vec<Vec<Square>> = Vec::new();
    for y in 0..height {
        let mut line: Vec<Square> = Vec::new();
        for x in 0..width {
            let sq = Square {
                adjacent_paper_count: 0,
                is_paper: lines[y][x] == "@",
            };
            line.push(sq)
        }
        grid.push(line)
    }
    let mut count: usize = 0;
    let mut removed = 1;
    while removed > 0 {
        removed = 0;
        for y in 0..height {
            for x in 0..width {
                increase_adjacent_paper_count(width, height, &mut grid, y, x);
            }
        }
        for y in 0..height {
            for x in 0..width {
                let sq = &mut grid[y][x];
                if is_removable(sq) {
                    count += 1;
                    removed += 1;
                    sq.is_paper = false
                }
            }
        }
        print_state(&mut grid);
        println!("Removing {}", removed);
        reset_adjacent_count(width, height, &mut grid);
    }
    println!("{}", count);
    count
}

fn reset_adjacent_count(width: usize, height: usize, grid: &mut Vec<Vec<Square>>) {
    iterate_over_grid(
        |sq, _y, _x| sq.adjacent_paper_count = 0,
        width,
        height,
        grid,
    );
}

fn iterate_over_grid(
    function: fn(&mut Square, usize, usize),
    width: usize,
    height: usize,
    grid: &mut Vec<Vec<Square>>,
) {
    for y in 0..height {
        for x in 0..width {
            function(&mut grid[y][x], y, x);
        }
    }
}

fn increase_adjacent_paper_count(
    width: usize,
    height: usize,
    grid: &mut Vec<Vec<Square>>,
    y: usize,
    x: usize,
) {
    if grid[y][x].is_paper {
        for xy_adjustment in XY_ADJUSTMENTS {
            let target_y = (y as isize + xy_adjustment.0) as usize;
            let target_x = (x as isize + xy_adjustment.1) as usize;
            if within_bounds(width, height, target_y, target_x) {
                let other_sq = &mut grid[target_y][target_x];
                other_sq.adjacent_paper_count += 1;
            }
        }
    }
}

fn within_bounds(width: usize, height: usize, target_y: usize, target_x: usize) -> bool {
    target_x < width && target_y < height
}

fn print_state(grid: &mut Vec<Vec<Square>>) {
    for line in grid {
        for x in line {
            if is_removable(x) {
                print!("x")
            } else if x.is_paper {
                print!("@")
            } else {
                print!(".")
            }
            // print!("{}", x.adjacent_paper_count)
        }
        println!();
    }
}

fn is_removable(x: &mut Square) -> bool {
    x.adjacent_paper_count < 4 && x.is_paper
}

struct Square {
    adjacent_paper_count: usize,
    is_paper: bool,
}

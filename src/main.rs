//QUESTION 2

//Initialize the Board 

fn main() {
    let mut board = vec![vec![0; 20]; 20];
    let initial_live_cells = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

    for &(x, y) in &initial_live_cells {
        board[x][y] = 1;
    }

    for _ in 0..20 {
        board = next_generation(&board);
        print_board(&board);
    }
}

fn next_generation(board: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_board = vec![vec![0; 20]; 20];

    for x in 0..20 {
        for y in 0..20 {
            let live_neighbors = count_live_neighbors(board, x, y); 
            new_board[x][y] = match live_neighbors {
                2 => board[x][y], // stays the same
                3 => 1, // become alive
                _ => 0, // dead
            };
        }
    }

    new_board
}

fn count_live_neighbors(board: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for &(dx, dy) in &directions {
        let nx = (x as isize + dx + 20) % 20;
        let ny = (y as isize + dy + 20) % 20;
        count += board[nx as usize][ny as usize];
    }

    count
}

fn print_board(board: &Vec<Vec<i32>>) {
    for row in board {
        for &cell in row {
            print!("{}", if cell == 1 { "1" } else { "0" });
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_live_neighbors() {
        let board = vec![
            vec![0, 1, 0],
            vec![1, 1, 1],
            vec![0, 1, 0],
        ];
        assert_eq!(count_live_neighbors(&board, 1, 1), 4);
        assert_eq!(count_live_neighbors(&board, 0, 0), 3);
        assert_eq!(count_live_neighbors(&board, 2, 2), 3);
    }
}

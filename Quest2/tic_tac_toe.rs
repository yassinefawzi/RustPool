pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string();
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string();

    }
    return "tie".to_string();
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if player == table[0][0] && player == table[1][1] && player == table[2][2] {
        return true;
    }
    if player == table[0][2] && player == table[1][1] && player == table[2][0] {
        return true;
    }
    return false;
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
        if player == row[0] && player == row[1] && player == row[2] {
            return true;
        }
    }
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if player == table[0][col] && player == table[1][col] && player == table[2][col] {
            return true;
        }
    }
    return false;
}

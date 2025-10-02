

use std::{io};
use rand::Rng;

// Структура игры 2048
struct Game2048 {
    board: Vec<Vec<u32>>,
    score:u32,
    moves:u32,
    won:Option<u32>
}

// Реализация методов для структуры Game2048
impl Game2048 {
    // Метод создания новой игры
    fn new() -> Self {
        Self { board: (vec![vec![0; 4]; 4]), score: (0), moves: (0), won: (None) }
    }
    // Метод вывода игрового поля в консоль
    fn print(&self) {
        // Выводим верхнюю границу таблицы
        println!("+------+------+------+------+");

        // Проходим по каждой строке поля
        for row in &self.board {
            // Начинаем строку с разделителя
            print!("|");
            
            // Проходим по каждой ячейке в строке
            for &cell in row {
                // Если ячейка пустая (0)
                if cell == 0 {
                    // Выводим пустую клетку
                    print!("      |");
                } else {
                    // Иначе выводим значение с выравниванием
                    print!(" {:4} |", cell);
                }
            }
            // Переход на новую строку
            println!("");
            // Выводим нижнюю границу строки
            println!("+------+------+------+------+");
        }
    }
    
    // Метод добавления новой плитки на случайное пустое место
    fn spawn_tile(&mut self) {
        // Создаём вектор для хранения координат пустых ячеек
        let mut empty_position: Vec<(usize, usize)> = Vec::new();

        // Двойной цикл для прохода по всем ячейкам поля
        for row_index in 0..4 {        // индекс строки (0, 1, 2, 3)
            for col_index in 0..4 {    // индекс столбца (0, 1, 2, 3)
                // Проверяем, пуста ли ячейка (значение 0)
                if self.board[row_index][col_index] == 0 {
                    // Если пуста - добавляем её координаты в вектор
                    empty_position.push((row_index, col_index));
                }
            }
        }
        
        // Проверяем, есть ли вообще пустые ячейки
        if !empty_position.is_empty() {
            let mut rng = rand::rng();
            let random_index = rng.random_range(0..empty_position.len());
            let (row, col) = empty_position[random_index];

            let random_choices = if rng.random_range(0..100) < 90 {2} else {4};

            self.board[row][col] = random_choices;
        }
    }
    fn process_row(&mut self, temp_row:Vec<u32>) -> Vec<u32>{
        let mut result:Vec<u32> = Vec::new();
        let mut i = 0;
        
        while temp_row.len() > i {
                if i + 1 < temp_row.len() && temp_row[i] == temp_row[i + 1] {
                    let new_value = temp_row[i] * 2;
                    result.push(new_value);
                    self.score += new_value;
                    i += 2;
                } else {
                    result.push(temp_row[i]);
                    i += 1
                }   
            }
        while result.len() < 4 {
            result.push(0);
        }
        result
    }
    fn move_left(&mut self) {
        for row_index in 0..4 {                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  
            let mut temp_row: Vec<u32> = Vec::new();
                for col_index in 0..4 {
                    if self.board[row_index][col_index] == 0 {
                    continue;
                    } else {
                    temp_row.push(self.board[row_index][col_index]);
                }
            }
            let processed = self.process_row(temp_row);
                for col_index in 0..4 {
                    self.board[row_index][col_index] = processed[col_index];
                }
            }
    }
    fn move_right(&mut self) {
        for row_index in 0..4 { 
            let mut temp_row: Vec<u32> = Vec::new();
                for col_index in (0..4).rev() {
                    if self.board[row_index][col_index] == 0 {
                    continue;
                    } else {
                    temp_row.push(self.board[row_index][col_index]);
                }
            }
            let processed = self.process_row(temp_row);
                for (i, &value) in processed.iter().rev().enumerate() {
                    self.board[row_index][i] = value;
                }
            }
    }   
    fn move_up(&mut self) {
        for col_index in 0..4 { 
            let mut temp_row: Vec<u32> = Vec::new();
                for row_index in 0..4 {
                    if self.board[row_index][col_index] == 0 {
                    continue;
                    } else {
                    temp_row.push(self.board[row_index][col_index]);
                }
            }
            let processed = self.process_row(temp_row);
                for (i, &value) in processed.iter().enumerate() {
                    self.board[i][col_index] = value;
                }
            }
    }
    fn move_down(&mut self) {
        for col_index in 0..4 { 
            let mut temp_row: Vec<u32> = Vec::new();
                for row_index in 0..4 {
                    if self.board[row_index][col_index] == 0 {
                    continue;
                    } else {
                    temp_row.push(self.board[row_index][col_index]);
                }
            }
            let processed = self.process_row(temp_row);
                for (i, &value) in processed.iter().enumerate() {
                    self.board[3 - i][col_index] = value;
                }
            }
    }
    fn game_won(&mut self) {
        if self.won.is_none() {
            for row in &self.board{
                for &cell in row {
                    if cell == 2048 {
                    self.won = Some(2048);
                    return;
                    }
                }
            }
        }
    }
    fn game_over(&mut self) -> bool {
        for row in &self.board{
                for &cell in row {
                    if cell == 0 {
                        return false;
                    }
                }
            }
        for row in &self.board{
                for j in 0..3 {
                    if row[j] == row[j + 1] {
                        return false;
                    }
                }
            }
        for i in 0..4{
                for j in 0..3{
                    if self.board[j][i] == self.board[j + 1][i]{
                        return false;
                    }
                }
            }
        println!(r#"
      ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗ 
     ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗
     ██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝
     ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗
     ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║
      ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝
    "#);
        true
    }
}

// Основная функция программы
fn main() {
    println!(r#"
  /$$$$$$   /$$$$$$  /$$   /$$  /$$$$$$ 
 /$$__  $$ /$$$_  $$| $$  | $$ /$$__  $$
|__/  \ $$| $$$$\ $$| $$  | $$| $$  \ $$
  /$$$$$$/| $$ $$ $$| $$$$$$$$|  $$$$$$/
 /$$____/ | $$\ $$$$|_____  $$ >$$__  $$
| $$      | $$ \ $$$      | $$| $$  \ $$
| $$$$$$$$|  $$$$$$/      | $$|  $$$$$$/
|________/ \______/       |__/ \______/ 
    "#);
    let mut game = Game2048::new();
    game.spawn_tile();
    
    loop {
        game.print();
        println!("Введите команду (w/a/s/d) или 'q' для выхода:");
        println!("Общий счёт: {}",game.score);
        println!("Количество ходов: {}",game.moves);    

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().chars().next().unwrap_or(' ');
        println!("Получена команда: {:?}", command);
    match command {
    'w' => {
        let old_board = game.board.clone();
        game.move_up();
        
        if game.board != old_board {
            game.spawn_tile();
            game.game_won();
            game.moves += 1;  
            
            if game.game_over() {
                println!("Игра окончена! Вы проиграли.");
                break;
            }
        }
    },
    's' => {
        let old_board = game.board.clone();
        game.move_down();
        
        if game.board != old_board {
            game.spawn_tile();
            game.game_won();
            game.moves += 1;
            
            if game.game_over() {
                println!("Игра окончена! Вы проиграли.");
                break;
            }
        }
    },
    'a' => {
        let old_board = game.board.clone();
        game.move_left();
        
        if game.board != old_board {
            game.spawn_tile();
            game.game_won();
            game.moves += 1;
            
            if game.game_over() {
                println!("Игра окончена! Вы проиграли.");
                break;
            }
        }
    },
    'd' => {
        let old_board = game.board.clone();
        game.move_right();
        
        if game.board != old_board {
            game.spawn_tile();
            game.game_won();
            game.moves += 1;
            
            if game.game_over() {
                println!("Игра окончена! Вы проиграли.");
                break;
            }
        }
    },
    'q' => break,
    _ => println!("Ошибка ввода")
}
        
        if game.won.is_some() {
            println!("Поздравляем! Вы выиграли!");
            break;
        }
    }
    
}
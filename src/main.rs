use std::io::{self, Write};


struct Board {
    board: [[char; 3]; 3],    
}

impl Board {
    fn new() -> Board {
        let board = [
            ['-', '-', '-'],
            ['-', '-', '-'],
            ['-', '-', '-']
        ];

        Board {board: board}
    }

    fn make_move(&mut self, x: usize, y: usize, player: char) -> Result<String, String> {
        if self.board[y][x] != '-' {
            println!("{}", self.board[y][x]);
            println!("Position invalide!");
            return Err("La position donnée est déjà prise!".to_string());
        }
        self.board[y][x] = player;
        return Ok("Opération approuvée!".to_string())
    }

    fn is_game_over(&self) -> bool {
        const WINNING_MOVES: [[u8; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6]
        ];

        for winning_move in WINNING_MOVES {
            let a: (usize, usize) = ((winning_move[0] % 3) as usize, (winning_move[0] / 3) as usize);
            let b: (usize, usize) = ((winning_move[1] % 3) as usize, (winning_move[1] / 3) as usize);
            let c: (usize, usize) = ((winning_move[2] % 3) as usize, (winning_move[2] / 3) as usize);

            if self.board[a.0][a.1] == '-' || self.board[b.0][b.1] == '-' {
                return false;
            }

            if self.board[a.0][a.1] == self.board[b.0][b.1] &&
               self.board[a.0][a.1] == self.board[c.0][c.1] {
                return true;
            }
        }

        return false;
    }

    fn display(&self) {
        for row in self.board {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

struct Game {
    board: Board,
    next_player: char,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            next_player: 'o'
        }
    }

    fn run(&mut self) {
        while !self.board.is_game_over() {
            let coordinate = self.ask_move();
            self.make_move(coordinate.0, coordinate.1);
        }
    }

    // fn reset(&mut self) {
    //     self.board = Board::new();
    //     self.next_player = 'o';
    //     println!("Le jeu a été réinitialisé.");
    // }

    fn make_move(&mut self, x: usize, y: usize) -> Option<bool> {
        self.board.make_move(x, y, self.next_player).err();

        println!("Le joueur {} a placé sa marque à la position ({x}, {y}):", self.next_player);
        self.board.display();
        
        if self.board.is_game_over() {
            println!("Le joueur {} a gagné la partie !!!", self.next_player);
            return Some(true);
        }
        
        self.next_player = if self.next_player == 'x' {'o'} else {'x'};

        return Some(false);
    }

    fn ask_move(&self) -> (usize, usize) {
        println!("C'est au tour du joueur {} de jouer (entrez la position comme-ci '0 1')", self.next_player);
        
        loop {
            let mut player_move = String::new();

            print!("> ");
            io::stdout().flush().expect("Erreur lors du vidage");
            
            io::stdin()
                .read_line(&mut player_move)
                .expect("Erreur dans la lecture de l'entrée!");
            
            let mut iter = player_move.trim().split_whitespace();

            match (iter.next(), iter.next()) {
                (Some(_x), Some(_y)) => match (_x.parse::<usize>(), _y.parse::<usize>()) {
                    (Ok(x), Ok(y)) => {
                        if x < 3 || y < 3 {
                            return (x, y);
                        }
                        println!("Les deux valeurs doivent etre entre 0 et 2!");
                    },
                    _ => println!("Vous devez entrer 2 chiffres!"),
                },
                _ => println!("Vous devez séparer les positions horizontales et verticales par un espace!"),
            }

        }
    }
}

fn test() {
    let mut game = Game::new();

    if game.make_move(1,1).unwrap() { return; }
    if game.make_move(1,2).unwrap() { return; }
    if game.make_move(2,1).unwrap() { return; }
    if game.make_move(0,1).unwrap() { return; }
    if game.make_move(0,0).unwrap() { return; }
    if game.make_move(0,1).unwrap() { return; }
    if game.make_move(2,2).unwrap() { return; }
    if game.make_move(0,2).unwrap() { return; }

}

fn run() {
    let mut game = Game::new();

    game.run();
}



fn main() {
    // test();

    run()
}



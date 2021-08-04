use std::io;

mod board;

fn main() {
    let mut board = board::Board::new();
    loop {
        println!("{}", board);
        println!("Enter move: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut indices = input
            .trim()
            .split_whitespace()
            .map(str::parse::<usize>)
            .map(|x| x.map(|y| board::SquareIndex::new(&y)));
        if let (Some(Ok(Ok(outer))), Some(Ok(Ok(inner)))) = (indices.next(), indices.next()) {
            if let Err(e) = board.do_move(&outer, &inner) {
                println!("Invalid move: {}", e);
            }
        } else {
            println!("Invalid input");
        }
    }
}

use std::io;

mod board;
mod square;

fn main() {
    let mut board = board::OuterBoard::new();
    let mut active_player = square::Mark::O;

    while board.get_state() == board::BoardState::InProgress {
        println!("{}", board);
        println!("Enter move: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut indices = input
            .trim()
            .split_whitespace()
            .map(str::parse::<usize>)
            .map(|x| x.map(|y| board::Index::new(&y)));
        if let (Some(Ok(Ok(outer))), Some(Ok(Ok(inner)))) = (indices.next(), indices.next()) {
            if let Err(e) = board.do_move(&active_player, &outer, &inner) {
                println!("Invalid move: {}", e);
            } else {
                active_player = active_player.flip();
            }
        } else {
            println!("Invalid input");
        }
    }

    println!("{}", board);
    match board.get_state() {
        board::BoardState::Winner(m) => println!("Winner: {}!", m),
        board::BoardState::Draw => println!("It's a draw!"),
        board::BoardState::InProgress => unreachable!(),
    }
}

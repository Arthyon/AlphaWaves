use rand::Rng;
use shakmaty::*;
pub struct Engine {
    pub pos: Chess,
}
impl Engine {
    pub fn new() -> Engine {
        Engine {
            pos: Chess::default(),
        }
    }
    pub fn update_position(&mut self, input: Vec<&str>) {
        if input[1] == "startpos" {
            self.pos = Chess::default();
        }
        if input.len() > 3 {
            if input[2] == "moves" {
                for x in 3..input.len() {
                    let m = input[x].parse::<uci::Uci>();
                    match m {
                        Err(err) => println!("info string Illegal move: {}. {}", input[3], err),
                        Ok(uci_move) => {
                            let normal_move = uci_move.to_move(&self.pos).unwrap();
                            self.pos.play_unchecked(&normal_move);
                        }
                    }
                }
            }
        }
    }
    pub fn go(&mut self) {
        // let moves = self.pos.legals();
        // let mut rnd = rand::thread_rng();
        // let move_index = rnd.gen_range(0, moves.len());
        let legal_moves = self.pos.legals();
        let mut best_move = (legal_moves[0].to_owned(), 0.0);
        for legal_move in &legal_moves {
            // println!("info string evaluating {:?}", legal_move);
            let pos = self.pos.clone();

            let evaluation = self.evaluate_move(&pos, &legal_move, 0.0, 1, true);
            println!(
                "info string move {:?} evaluated to {:?}",
                legal_move, evaluation
            );
            if evaluation > best_move.1 {
                best_move = (legal_move.to_owned(), evaluation);
            }
        }
        if best_move.1 < 0.2 {
            let mut rnd = rand::thread_rng();
            let move_index = rnd.gen_range(0, legal_moves.len());
            println!(
                "bestmove {}",
                uci::Uci::from_move(&self.pos, &legal_moves[move_index]).to_string()
            );
        } else {
            println!(
                "bestmove {}",
                uci::Uci::from_move(&self.pos, &best_move.0).to_string()
            );
        }
    }
    fn eval_single_move(&mut self, m: &Move) -> f32 {
        match m {
            Move::Castle { .. } => 7.0,
            Move::Normal { capture, role, .. } => match capture {
                None => match role {
                    Role::Pawn => 0.0,
                    _ => 0.1,
                },
                Some(c) => match c {
                    Role::Bishop => 3.0,
                    Role::King => 99999.0,
                    Role::Knight => 3.0,
                    Role::Pawn => 1.0,
                    Role::Queen => 9.0,
                    Role::Rook => 5.0,
                },
            },
            _ => 0.0,
        }
    }
    fn evaluate_move(
        &mut self,
        pos: &Chess,
        current_move: &Move,
        current_eval: f32,
        remaining_depth: i32,
        own_turn: bool,
    ) -> f32 {
        if remaining_depth == 0 {
            return current_eval;
        }
        let eval = self.eval_single_move(&current_move);
        let new_eval = current_eval + eval;

        let mut new_pos = pos.clone();
        new_pos.play_unchecked(&current_move);
        let mut depth = 2;
        let mut o_turn = false;

        let mut legal_moves = new_pos.legals();
        let mut total_eval = new_eval;
        let mut best_evaluation = new_eval;
        let mut best_move = legal_moves[0].to_owned();
        while depth > 0 {
            for legal_move in legal_moves {
                // println!("evaluating move {:?} to be worth {}", legal_move, new_eval);
                let evaluation = self.eval_single_move(&legal_move);
                if evaluation > best_evaluation {
                    best_evaluation = evaluation;
                    best_move = legal_move.to_owned();
                }
                // } else if evaluation < best_evaluation {
                //     best_evaluation = evaluation;
                //     best_move = legal_move.to_owned();
                // }
            }
            new_pos.play_unchecked(&best_move);
            if o_turn {
                total_eval = total_eval + best_evaluation;
            } else {
                total_eval = total_eval - best_evaluation;
            }
            o_turn = !o_turn;
            legal_moves = new_pos.legals();
            depth = depth - 1;
            best_evaluation = 0.0;
        }

        total_eval
    }
}

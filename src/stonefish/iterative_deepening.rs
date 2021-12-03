use crate::uci::uci::StopFlag;

use super::{evaluation::Evaluation, node::Node};

impl Node {
    pub fn iterative_deepening(&mut self, max_depth: Option<usize>, stop_flag: StopFlag) -> Evaluation {
        let mut depth: usize = 1;

        let mut eval = self.evaluation;

        // Search at higher and higher depths
        loop {
            if let Some(max_depth) = max_depth {
                if depth > max_depth {
                    break;
                }
            }

            // Search at the current depth and update the evaluation
            if let Ok(new_eval) = self.minimax(depth, stop_flag.clone()) {
                eval = new_eval;
            } else {
                // Abort the search
                break;
            }

            depth += 1;
        }

        eval
    }
}

use std::fmt::Display;

use pleco::BitMove;

use super::Node;

pub type Line = Vec<BitMove>;

impl Node {
    /// Determine the number of nodes in the tree.
    pub fn size(&self) -> usize {
        if let Some(children) = &self.children {
            let mut size: usize = 1;

            for child in children {
                size += child.size();
            }

            size
        } else {
            1
        }
    }

    /// Determine the depth of the tree.
    pub fn depth(&self) -> usize {
        if let Some(children) = &self.children {
            let mut depth: usize = 0;

            for child in children {
                depth += depth.max(child.depth() + 1);
            }

            depth
        } else {
            0
        }
    }

    /// The best successor node.
    pub fn best_node(&self) -> Option<&Node> {
        if let Some(children) = &self.children {
            children.first()
        } else {
            None
        }
    }

    /// The best move to play.
    pub fn best_move(&self) -> Option<BitMove> {
        if let Some(node) = self.best_node() {
            node.board.last_move()
        } else {
            None
        }
    }
    
    /// The best line to play.
    pub fn best_line(&self) -> Line {
        if let Some(node) = self.best_node() {
            let best_move = vec![self.best_move().unwrap()];
            let best_child_line = node.best_line();
            let best_line = vec![best_move, best_child_line].concat();
            best_line
        } else {
            vec![]
        }
    }

    /// Format a line of moves.
    fn format_line(line: &Line) -> String {
        line.iter().map(|mv| mv.stringify()).collect::<Vec<String>>().join(" ")
    }

    /// Send the best move to the engine.
    pub fn send_best_move(&self) {
        if let Some(mv) = self.best_move() {
            println!("bestmove {}", mv.stringify());
        }
    }
}

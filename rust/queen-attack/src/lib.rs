#[derive(Debug)]
pub struct ChessPosition {
    square: u8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank > 7 || rank < 0 || file > 7 || file < 0 {
            None
        } else {
            Some(ChessPosition {
                square: (file * 8 + rank) as u8,
            })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let s = self.position.square;
        let o = other.position.square;
        let horizontal_min = s - s % 8;
        let horizontal_max = horizontal_min + 8;
        (o >= horizontal_min && o <= horizontal_max) //horizontal
            || s % 9 == o % 9 //diagonal down right
            || s % 7 == o % 7 //diagonal down left
            || s % 8 == o % 8 // vertical
    }
}

#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
		if (0..=7).contains(&rank) &&(0..=7).contains(&file) {
			Some(ChessPosition {rank, file})
		} else {
			None
		}
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
		Queen {
			position,
		}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
		let r1 = self.position.rank;
		let f1 = self.position.file;
		let r2 = other.position.rank;
		let f2 = other.position.file;

		if r1 == r2 || f1 == f2 {
			return true;
		}

		if (r1 - r2).abs() == (f1 - f2).abs() {
			return true;
		}
		false
    }
}
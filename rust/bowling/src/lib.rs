use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum Frame {
    Strike,
    Spare { roll1: u16, roll2: u16 },
    Open { roll1: u16, roll2: u16 },
    Incomplete { roll: u16 },
    SpareBonus { roll: u16 },
    StrikeBonus { roll1: u16, roll2: u16 },
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, roll: u16) -> Result<(), Error> {
        if roll > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let len = self.frames.len();
        match self.frames.last() {
            None => {
                if roll == 10 {
                    self.frames.push(Frame::Strike);
                } else {
                    self.frames.push(Frame::Incomplete { roll });
                }
                return Ok(());
            }
            Some(frame) => match frame {
                Frame::Strike | Frame::Spare { .. } | Frame::Open { .. } if len < 10 => {
                    if roll == 10 {
                        self.frames.push(Frame::Strike);
                    } else {
                        self.frames.push(Frame::Incomplete { roll });
                    }
                    return Ok(());
                }
                Frame::Incomplete { roll: roll1 } => {
                    if len == 11 {
                        if *roll1 != 10 && roll1 + roll > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        self.frames[len - 1] = Frame::StrikeBonus {
                            roll1: *roll1,
                            roll2: roll,
                        };
                        return Ok(());
                    }
                    match (*roll1 + roll).cmp(&10) {
                        Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                        Ordering::Equal => {
                            self.frames[len - 1] = Frame::Spare {
                                roll1: *roll1,
                                roll2: roll,
                            };
                        }
                        Ordering::Less => {
                            self.frames[len - 1] = Frame::Open {
                                roll1: *roll1,
                                roll2: roll,
                            };
                        }
                    }
                    return Ok(());
                }
                Frame::SpareBonus { .. } | Frame::StrikeBonus { .. } => {
                    return Err(Error::GameComplete)
                }
                Frame::Open { .. } => return Err(Error::GameComplete),
                Frame::Strike => {
                    self.frames.push(Frame::Incomplete { roll });
                    return Ok(());
                }
                Frame::Spare { .. } => {
                    self.frames.push(Frame::SpareBonus { roll });
                    return Ok(());
                }
            },
        }
    }

    pub fn score(&self) -> Option<u16> {
        let len = self.frames.len();
        if len < 10 {
            return None;
        }
        let mut score = 0;
        for (i, frame) in self.frames.iter().enumerate().take(10) {
            match frame {
                Frame::Strike => {
                    score += 10;
                    if i + 1 == len {
                        return None;
                    }
                    match self.frames[i + 1] {
                        Frame::Strike => {
                            score += 10;
                            if i + 2 == len {
                                return None;
                            }
                            match self.frames[i + 2] {
                                Frame::Strike => score += 10,
                                Frame::Open { roll1: r1, .. } => score += r1,
                                Frame::Spare { roll1: r1, .. } => score += r1,
                                Frame::StrikeBonus { roll1: r1, .. } => score += r1,
                                Frame::Incomplete { .. } => return None,
                                Frame::SpareBonus { .. } => unreachable!(),
                            }
                        }
                        Frame::Open {
                            roll1: r1,
                            roll2: r2,
                        } => score += r1 + r2,
                        Frame::Spare {
                            roll1: r1,
                            roll2: r2,
                        } => score += r1 + r2,
                        Frame::StrikeBonus {
                            roll1: r1,
                            roll2: r2,
                        } => score += r1 + r2,
                        Frame::Incomplete { .. } => return None,
                        Frame::SpareBonus { .. } => unreachable!(),
                    }
                }
                Frame::Open {
                    roll1: r1,
                    roll2: r2,
                } => score += r1 + r2,
                Frame::Spare {
                    roll1: r1,
                    roll2: r2,
                } => {
                    score += r1 + r2;
                    if i + 1 == len {
                        return None;
                    }
                    match self.frames[i + 1] {
                        Frame::Strike => score += 10,
                        Frame::Open { roll1: r1, .. } => score += r1,
                        Frame::Spare { roll1: r1, .. } => score += r1,
                        Frame::SpareBonus { roll: r } => score += r,
                        Frame::Incomplete { .. } => return None,
                        Frame::StrikeBonus { .. } => unreachable!(),
                    }
                }
                Frame::Incomplete { .. } => return None,
                Frame::SpareBonus { .. } => unreachable!(),
                Frame::StrikeBonus { .. } => unreachable!(),
            }
        }
        Some(score)
    }
}

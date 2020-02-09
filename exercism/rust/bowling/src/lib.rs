#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// 每局最多 2 次投球，分三种情况：
// open frame (失误)：两次投球后都没将所有球击中
// spare frame (补中)：经过两次投球后将所有球击中
// strike (全中)：第一次投球就将所有球击中
pub struct BowlingGame {
    cur_round: usize,
    frames: Vec<Vec<u16>>,
    tenth_frame: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            cur_round: 1,
            frames: vec![vec![]],
            tenth_frame: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.cur_round == 0 {
            return Err(Error::GameComplete);
        }

        // 将 pins 计入当前局
        let cur_frame = self.frames.get_mut(self.cur_round as usize - 1).unwrap();
        cur_frame.push(pins);
        // 如果是第 10 局，同时将相应的值复制一份到 tenth_frame 中
        if self.cur_round == 10 {
            self.tenth_frame.push(pins);
        }

        // 处理结果
        let total_pins: u16 = cur_frame.iter().sum();
        if total_pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // 处理特殊情况
        // 如果当前局是第 10 局
        if self.cur_round == 10 && total_pins < 10 && cur_frame.len() == 2 {
            // 如果第 10 局是失误
            // 马上结束，没有多余的投球
            self.cur_round = 0;
            return Ok(());
        }
        // 如果当前局是第 11 局
        if self.cur_round == 11 {
            if self.tenth_frame.len() == 2 || cur_frame.len() == 2 {
                // 如果第 10 局是补中，第 11 局投球一次后
                // 或者第 10 局是全中，第 11 局投球两次后
                // 马上结束，没有多余的投球
                self.cur_round = 0;
                return Ok(());
            }
        }
        // 如果当前是第 12 局
        if self.cur_round == 12 {
            // 第 12 局只有一次投球机会
            // 马上结束，没有多余的投球
            self.cur_round = 0;
            return Ok(());
        }

        // 如果当前局结束，进入下一局
        if total_pins == 10 || cur_frame.len() == 2 {
            self.cur_round += 1;
            self.frames.push(vec![]);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.cur_round != 0 {
            return None;
        }

        let mut scores = [0; 10];
        for i in 0..10 {
            let first_frame = &self.frames[i];
            let pins: u16 = first_frame.iter().sum();

            // open frame / 失误
            if pins < 10 {
                scores[i] = pins;
            } else {
                let second_frame = &self.frames[i + 1];
                if first_frame.len() == 2 {
                    // spare frame / 补中
                    scores[i] = pins + second_frame[0];
                } else {
                    // strike / 全中
                    if second_frame.len() == 2 {
                        scores[i] = pins + second_frame[0] + second_frame[1];
                    } else {
                        let third_frame = &self.frames[i + 2];
                        scores[i] = pins + second_frame[0] + third_frame[0];
                    }
                }
            }
        }
        Some(scores.iter().sum())
    }
}

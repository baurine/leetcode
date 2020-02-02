#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        // 使用 position() 找到第一个不正确的碱基
        let pos = dna.chars().position(|c| !"ATCG".contains(c));
        match pos {
            Some(x) => Err(x),
            None => Ok(DNA(String::from(dna))),
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA(self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => ' ',
            })
            .collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let pos = rna.chars().position(|c| !"AUCG".contains(c));
        match pos {
            Some(x) => Err(x),
            None => Ok(RNA(String::from(rna))),
        }
    }
}

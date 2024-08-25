pub struct LexerCursor {
    pub chars: Box<[char]>,
    pub position: usize,
}

impl LexerCursor {
    pub fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            position: 0,
        }
    }

    pub fn current(&self) -> Option<char> {
        self.chars.get(self.position).map(|x| *x)
    }

    pub fn advance(&mut self, by: usize) {
        self.position += by;
        self.position = self.position.min(self.chars.len())
    }

    pub fn rewind(&mut self, by: usize) {
        self.position = self.position.saturating_sub(by);
    }
}

impl Iterator for LexerCursor {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.chars.get(self.position).map(|x| *x);
        self.position += 1;
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.chars.len() - self.position,
            Some(self.chars.len() - self.position),
        )
    }
}

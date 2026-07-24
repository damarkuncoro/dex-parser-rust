pub struct ByteTracker {
    usage: Vec<bool>,
}

impl ByteTracker {
    pub fn new(size: usize) -> Self {
        Self { usage: vec![false; size] }
    }

    pub fn mark(&mut self, offset: usize, length: usize) {
        let end = (offset + length).min(self.usage.len());
        if offset < end {
            self.usage[offset..end].fill(true);
        }
    }

    pub fn get_gaps(&self) -> Vec<(usize, usize)> {
        let mut gaps = Vec::new();
        let mut start = None;

        for (i, &used) in self.usage.iter().enumerate() {
            if !used && start.is_none() {
                start = Some(i);
            } else if used && start.is_some() {
                gaps.push((start.unwrap(), i - start.unwrap()));
                start = None;
            }
        }

        if let Some(s) = start {
            gaps.push((s, self.usage.len() - s));
        }

        gaps
    }
}

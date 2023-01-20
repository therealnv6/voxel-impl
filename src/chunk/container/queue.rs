use std::ops::RangeBounds;

#[derive(Debug, Default)]
pub struct ChunkUpdateQueue {
    chunks: Vec<i32>,
}

impl ChunkUpdateQueue {
    pub fn queue(&mut self, chunk: i32) {
        self.chunks.push(chunk);
    }

    pub fn pull<T: RangeBounds<usize>>(&mut self, range: T) -> Vec<i32> {
        return if range.contains(&self.chunks.len()) {
            self.chunks.drain(..)
        } else {
            self.chunks.drain(range)
        }
        .collect();
    }

    pub fn has_queue(&self) -> bool {
        !self.chunks.is_empty()
    }
}

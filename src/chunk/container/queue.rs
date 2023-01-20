#[derive(Debug, Default)]
pub struct ChunkUpdateQueue {
    chunks: Vec<i32>,
}

impl ChunkUpdateQueue {
    pub fn queue(&mut self, chunk: i32) {
        self.chunks.push(chunk);
    }

    pub fn pull(&mut self) -> Vec<i32> {
        self.chunks.drain(..).collect()
    }

    pub fn has_queue(&self) -> bool {
        !self.chunks.is_empty()
    }
}

use std::ops::RangeBounds;

use ndshape::ConstShape;

use crate::chunk::ChunkShape;

type ChunkQueueData = (i32, [u8; ChunkShape::SIZE as usize]);

#[derive(Debug, Default)]
pub struct ChunkUpdateQueue {
    chunks: Vec<ChunkQueueData>,
}

impl ChunkUpdateQueue {
    pub fn queue(&mut self, chunk: ChunkQueueData) {
        self.chunks.push(chunk);
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn pull<T: RangeBounds<usize>>(&mut self, range: T) -> Vec<ChunkQueueData> {
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

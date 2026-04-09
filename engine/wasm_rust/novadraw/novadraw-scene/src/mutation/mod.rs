use crate::{BlockId, Figure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingMutation {
    AddChild { parent: BlockId, child: BlockId },
    RemoveChild { parent: BlockId, child: BlockId },
    Reparent { child: BlockId, new_parent: BlockId },
}

#[derive(Default)]
pub struct PendingMutations {
    queue: Vec<PendingMutation>,
}

impl PendingMutations {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&mut self, mutation: PendingMutation) {
        self.queue.push(mutation);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn drain(&mut self) -> Vec<PendingMutation> {
        self.queue.drain(..).collect()
    }
}

pub trait MutationContext {
    fn allocate_block(&mut self, figure: Box<dyn Figure>) -> BlockId;
    fn enqueue_mutation(&mut self, mutation: PendingMutation);
}

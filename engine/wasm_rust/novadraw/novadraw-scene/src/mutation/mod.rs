use crate::{BlockId, Figure};

pub enum PendingMutation {
    AddChild {
        parent: BlockId,
        child: BlockId,
    },
    AddChildFigure {
        parent: BlockId,
        figure: Box<dyn Figure>,
    },
    RemoveChild {
        parent: BlockId,
        child: BlockId,
    },
    Reparent {
        child: BlockId,
        new_parent: BlockId,
    },
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
    fn enqueue_mutation(&mut self, mutation: PendingMutation);

    fn add_child_later(&mut self, parent: BlockId, figure: Box<dyn Figure>) {
        self.enqueue_mutation(PendingMutation::AddChildFigure { parent, figure });
    }

    fn remove_child_later(&mut self, parent: BlockId, child: BlockId) {
        self.enqueue_mutation(PendingMutation::RemoveChild { parent, child });
    }

    fn reparent_later(&mut self, child: BlockId, new_parent: BlockId) {
        self.enqueue_mutation(PendingMutation::Reparent { child, new_parent });
    }
}

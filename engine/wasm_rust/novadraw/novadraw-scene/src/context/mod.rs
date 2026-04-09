use crate::{BlockId, Rectangle};

pub trait NovadrawContext {
    fn target_id(&self) -> BlockId;
    fn repaint(&mut self, rect: Option<Rectangle>);
    fn invalidate(&mut self);
}

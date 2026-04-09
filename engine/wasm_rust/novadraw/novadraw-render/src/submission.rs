use novadraw_geometry::Rectangle;

use crate::command::RenderCommand;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageSet {
    pub union: Option<Rectangle>,
    pub regions: Vec<Rectangle>,
}

impl DamageSet {
    pub fn is_empty(&self) -> bool {
        self.union.is_none()
    }

    pub fn set_union(&mut self, rect: Rectangle) {
        if rect.width <= 0.0 || rect.height <= 0.0 {
            self.union = None;
            self.regions.clear();
            return;
        }
        self.union = Some(rect);
        self.regions.clear();
        self.regions.push(rect);
    }

    pub fn set_regions(&mut self, regions: Vec<Rectangle>) {
        let filtered: Vec<Rectangle> = regions
            .into_iter()
            .filter(|rect| rect.width > 0.0 && rect.height > 0.0)
            .collect();

        if filtered.is_empty() {
            self.clear();
            return;
        }

        let union = filtered
            .iter()
            .copied()
            .reduce(|acc, rect| acc.union(rect))
            .expect("filtered regions should not be empty");

        self.union = Some(union);
        self.regions = filtered;
    }

    pub fn clear(&mut self) {
        self.union = None;
        self.regions.clear();
    }
}

#[derive(Debug, Clone)]
pub struct RenderSubmission {
    pub commands: Vec<RenderCommand>,
    pub damage: DamageSet,
}

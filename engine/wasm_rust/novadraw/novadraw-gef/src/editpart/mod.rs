//! 编辑部件
//!
//! EditPart 是 GEF 的核心概念，负责将模型与视图连接。

use std::sync::Arc;
use novadraw_scene::{SceneGraph, BlockId};

slotmap::new_key_type! { pub struct EditPartId; }

/// EditPart trait
///
/// 所有编辑部件都要实现此 trait。
pub trait EditPart: Send + Sync + 'static {
    /// 获取关联的块 ID
    fn id(&self) -> EditPartId;
    /// 获取关联的场景图块 ID
    fn block_id(&self) -> BlockId;
    /// 获取父 EditPart
    fn parent(&self) -> Option<EditPartId>;
    /// 设置父 EditPart
    fn set_parent(&mut self, parent: Option<EditPartId>);
    /// 获取名称
    fn get_name(&self) -> &str;
    /// 是否选中
    fn is_selected(&self) -> bool;
    /// 设置选中状态
    fn set_selected(&mut self, selected: bool);
    /// 是否可见
    fn is_visible(&self) -> bool;
    /// 设置可见性
    fn set_visible(&mut self, visible: bool);
    /// 刷新部件
    fn refresh(&mut self, scene: &SceneGraph);
    /// 添加子部件
    fn add_child(&mut self, child: Box<dyn EditPart + 'static>);
    /// 移除子部件
    fn remove_child(&mut self, child_id: EditPartId);
    /// 获取子部件
    fn children(&self) -> &[EditPartId];
    /// 激活部件
    fn activate(&mut self);
    /// 停用部件
    fn deactivate(&mut self);
}

/// 图形编辑部件
///
/// 对应场景图中的 RuntimeBlock。
pub struct GraphicalEditPart {
    id: EditPartId,
    block_id: BlockId,
    parent: Option<EditPartId>,
    children: Vec<EditPartId>,
    name: Arc<str>,
    selected: bool,
    visible: bool,
}

impl GraphicalEditPart {
    pub fn new(id: EditPartId, block_id: BlockId, name: impl Into<Arc<str>>) -> Self {
        Self {
            id,
            block_id,
            parent: None,
            children: Vec::new(),
            name: name.into(),
            selected: false,
            visible: true,
        }
    }

    pub fn block_id(&self) -> BlockId {
        self.block_id
    }
}

impl EditPart for GraphicalEditPart {
    fn id(&self) -> EditPartId {
        self.id
    }

    fn block_id(&self) -> BlockId {
        self.block_id
    }

    fn parent(&self) -> Option<EditPartId> {
        self.parent
    }

    fn set_parent(&mut self, parent: Option<EditPartId>) {
        self.parent = parent;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn refresh(&mut self, scene: &SceneGraph) {
        if let Some(_block) = scene.get_block(self.block_id) {
            // 可以在这里更新部件状态
        }
    }

    fn add_child(&mut self, child: Box<dyn EditPart>) {
        self.children.push(child.id());
    }

    fn remove_child(&mut self, child_id: EditPartId) {
        self.children.retain(|&id| id != child_id);
    }

    fn children(&self) -> &[EditPartId] {
        &self.children
    }

    fn activate(&mut self) {
        // 激活时注册事件监听器等
    }

    fn deactivate(&mut self) {
        // 停用时清理资源
    }
}

/// 编辑部件注册表
///
/// 管理所有 EditPart 的生命周期。
pub struct EditPartRegistry {
    parts: slotmap::SlotMap<EditPartId, Box<dyn EditPart>>,
    block_to_part: std::collections::HashMap<BlockId, EditPartId>,
}

impl EditPartRegistry {
    pub fn new() -> Self {
        Self {
            parts: slotmap::SlotMap::with_key(),
            block_to_part: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, part: Box<dyn EditPart>) -> EditPartId {
        let block_id = part.block_id();
        let id = self.parts.insert_with_key(|_key| part);
        self.block_to_part.insert(block_id, id);
        id
    }

    pub fn unregister(&mut self, id: EditPartId) {
        if let Some(part) = self.parts.remove(id) {
            self.block_to_part.remove(&part.block_id());
        }
    }

    pub fn get(&self, id: EditPartId) -> Option<&dyn EditPart> {
        self.parts.get(id).map(|p| p.as_ref())
    }

    pub fn get_mut(&mut self, id: EditPartId) -> Option<&mut dyn EditPart> {
        self.parts.get_mut(id).map(|p| p.as_mut())
    }

    pub fn get_by_block(&self, block_id: BlockId) -> Option<&dyn EditPart> {
        self.block_to_part.get(&block_id).and_then(|id| self.get(*id))
    }

    pub fn get_by_block_mut(&mut self, block_id: BlockId) -> Option<&mut dyn EditPart> {
        if let Some(&id) = self.block_to_part.get(&block_id) {
            self.parts.get_mut(id).map(|p| p.as_mut())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

impl Default for EditPartRegistry {
    fn default() -> Self {
        Self::new()
    }
}

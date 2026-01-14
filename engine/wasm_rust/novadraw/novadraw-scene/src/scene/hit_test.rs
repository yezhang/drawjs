//! 命中测试模块
//!
//! 使用显式栈实现非递归的深度优先遍历，避免堆栈溢出。

use novadraw_geometry::{Point, Rect};

use super::{BlockId, RuntimeBlock};

/// 命中测试结果
#[derive(Debug, Clone)]
pub struct HitTestResult {
    /// 命中的路径（从根到叶，包含自身）
    pub path: Vec<BlockId>,
}

impl HitTestResult {
    /// 获取命中的路径
    pub fn path(&self) -> &[BlockId] {
        &self.path
    }

    /// 获取最上层的父节点（直接子节点）
    pub fn top_parent(&self) -> Option<BlockId> {
        self.path.first().copied()
    }

    /// 获取最底层的节点（命中的节点）
    pub fn target(&self) -> BlockId {
        self.path.last().copied().unwrap()
    }
}

/// 场景图引用（用于命中测试）
pub struct HitTestRef<'a> {
    pub(crate) blocks: &'a slotmap::SlotMap<BlockId, RuntimeBlock>,
}

impl<'a> HitTestRef<'a> {
    /// 获取块
    pub fn get(&self, id: BlockId) -> Option<&RuntimeBlock> {
        self.blocks.get(id)
    }
}

impl<'a> Clone for HitTestRef<'a> {
    fn clone(&self) -> Self {
        Self { blocks: self.blocks }
    }
}

/// 命中测试任务枚举
///
/// 使用显式栈避免递归导致的栈溢出。
/// 遍历策略：逆序遍历子节点（后添加的在上层，先检测）
#[derive(Debug, Clone)]
enum HitTestTask {
    /// 检测当前节点是否包含点
    Check { block_id: BlockId },
    /// 继续检测子节点
    Descend { block_id: BlockId },
}

/// 命中测试器
///
/// 使用显式栈实现非递归的深度优先遍历。
/// 遍历策略：
/// 1. 从指定节点开始
/// 2. 逆序遍历直接子节点（后添加的在上层，先检测）
/// 3. 利用 bounds 进行剪枝：点不在 bounds 内则跳过子节点
pub struct HitTester<'a> {
    scene: HitTestRef<'a>,
    task_stack: Vec<HitTestTask>,
    /// 当前路径（从根到当前节点）
    current_path: Vec<BlockId>,
}

impl<'a> HitTester<'a> {
    /// 创建命中测试器
    pub fn new(scene: &HitTestRef<'a>) -> Self {
        Self {
            scene: scene.clone(),
            task_stack: Vec::new(),
            current_path: Vec::new(),
        }
    }

    /// 命中测试
    ///
    /// 从指定节点开始检测，返回包含路径的命中结果。
    /// 如果未命中，返回 None。
    ///
    /// # 算法
    ///
    /// - 从指定节点开始
    /// - 逆序遍历直接子节点（后添加的在上层，先检测）
    /// - 利用 bounds 进行剪枝
    /// - 找到命中的最深层节点即返回
    pub fn hit_test(&mut self, root_id: BlockId, point: Point) -> Option<HitTestResult> {
        self.task_stack.clear();
        self.current_path.clear();

        // 初始任务：检测根节点
        self.task_stack.push(HitTestTask::Check { block_id: root_id });

        while let Some(task) = self.task_stack.pop() {
            match task {
                HitTestTask::Check { block_id } => {
                    if let Some(block) = self.scene.get(block_id) {
                        // 不可见或禁用的节点跳过
                        if !block.is_visible || !block.is_enabled {
                            continue;
                        }

                        // 检查点是否在 bounds 内
                        if self.contains_point(block.figure.bounds(), point) {
                            // 添加到当前路径
                            self.current_path.push(block_id);

                            // 继续检测子节点
                            self.task_stack
                                .push(HitTestTask::Descend { block_id });
                        }
                    }
                }
                HitTestTask::Descend { block_id } => {
                    if let Some(block) = self.scene.get(block_id) {
                        // 逆序遍历：后添加的在上层，先检测
                        for &child_id in block.children.iter().rev() {
                            self.task_stack.push(HitTestTask::Check { block_id: child_id });
                        }
                    }
                }
            }
        }

        // current_path 的最后一个节点就是命中的节点
        self.current_path.last().map(|_target_id| HitTestResult {
            path: self.current_path.clone(),
        })
    }

    /// 检查点是否在矩形内
    #[inline]
    fn contains_point(&self, bounds: Rect, point: Point) -> bool {
        bounds.contains(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::SceneGraph;
    use crate::figure::RectangleFigure;

    /// 创建简单的测试场景
    fn create_test_scene() -> (SceneGraph, BlockId, BlockId, BlockId) {
        let mut scene = SceneGraph::new();

        // 创建根内容块 (0, 0, 200, 200)
        let contents = scene.set_contents(Box::new(RectangleFigure::new(0.0, 0.0, 200.0, 200.0)));

        // 添加子节点 A (10, 10, 80, 80)
        let child_a = scene.add_child_to(contents, Box::new(RectangleFigure::new(10.0, 10.0, 80.0, 80.0)));

        // 添加子节点 B (50, 50, 100, 100) - 在 A 之上（后添加）
        let child_b = scene.add_child_to(contents, Box::new(RectangleFigure::new(50.0, 50.0, 100.0, 100.0)));

        (scene, contents, child_a, child_b)
    }

    #[test]
    fn test_hit_test_z_order() {
        let (scene, _contents, _child_a, child_b) = create_test_scene();

        // 点 (60, 60) 同时在 A 和 B 的范围内
        // B 后添加，应该先命中
        let point = Point::new(60.0, 60.0);
        let result = scene.hit_test_with_path(point);

        assert!(result.is_some());
        let target = result.unwrap().target();
        // 目标应该是 B（后添加的在上层）
        assert_eq!(target, child_b);
    }

    #[test]
    fn test_hit_test_miss() {
        let (scene, contents, _child_a, _child_b) = create_test_scene();

        // 点 (5, 5) 在根范围内，但不在任何子节点范围内
        let point = Point::new(5.0, 5.0);
        let result = scene.hit_test_with_path(point);

        // 应该命中根节点（contents）
        assert!(result.is_some());
        assert_eq!(result.unwrap().target(), contents);
    }

    #[test]
    fn test_hit_test_outside() {
        let (scene, _contents, _child_a, _child_b) = create_test_scene();

        // 点 (300, 300) 在所有节点范围外
        let point = Point::new(300.0, 300.0);
        let result = scene.hit_test_with_path(point);

        // 应该未命中
        assert!(result.is_none());
    }

    #[test]
    fn test_hit_test_path() {
        let (scene, contents, _child_a, child_b) = create_test_scene();

        // 点 (60, 60) 命中路径: contents -> B
        let point = Point::new(60.0, 60.0);
        let result = scene.hit_test_with_path(point).unwrap();

        // 路径应该至少包含 contents 和 child_b
        assert!(result.path().len() >= 2);
        // 最后一个元素应该是目标（child_b）
        assert_eq!(result.target(), child_b);
        // 路径应该包含 contents
        assert!(result.path().contains(&contents));
    }

    #[test]
    fn test_hit_test_invisible_node() {
        let (mut scene, _contents, child_a, child_b) = create_test_scene();

        // 将 B 设为不可见
        scene.blocks.get_mut(child_b).unwrap().is_visible = false;

        // 点 (60, 60) 应该命中 A（因为 B 不可见）
        let point = Point::new(60.0, 60.0);
        let result = scene.hit_test_with_path(point).unwrap();

        // 应该命中 A
        assert_eq!(result.target(), child_a);
    }
}

use serde::{Deserialize, Serialize};
use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use uuid::Uuid;

use crate::color::Color;
use crate::render_ctx;

// 1. 定义运行时的高速 ID (SlotKey)
new_key_type! { pub struct BlockId; }

// 注意：这里的 children 存的是高速 BlockId，而不是 UUID
pub struct RuntimeBlock {
    pub id: BlockId, // 自己的运行时 ID
    pub uuid: Uuid,  // 自己的持久化 ID (身份证)

    pub children: Vec<BlockId>, // 树状结构使用运行时 ID 连接
    pub parent: Option<BlockId>,

    pub figure: Box<dyn Paint>,
    // ... 其他属性 (Rect, Color, Data)
}

// 绘制特性定义
pub trait Paint {
    fn paint(&self, gc: &mut render_ctx::RenderContext) {
        // 提供默认实现
    }
    //fn bounds(&self) -> Rect;
    //fn hit_test(&self, point: (f32, f32)) -> bool;
}

//impl<T> Paint for T {}

pub struct NullFigure {}

impl NullFigure {
    fn new() -> Self {
        NullFigure {}
    }
}

impl Paint for NullFigure {
    fn paint(&self, _gc: &mut render_ctx::RenderContext) {
        // keep empty
    }
}

pub struct RectangleFigure {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub fill_color: Color,
}

impl RectangleFigure {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill_color: Color::hex("#3498db"),
        }
    }

    pub fn new_with_color(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill_color: color,
        }
    }
}

impl Paint for RectangleFigure {
    fn paint(&self, gc: &mut render_ctx::RenderContext) {
        gc.set_fill_style(self.fill_color);
        gc.draw_rect(self.x, self.y, self.width, self.height);
    }
}

impl RuntimeBlock {
    fn paint(&self, gc: &mut render_ctx::RenderContext) {
        self.figure.paint(gc);
    }

    pub fn set_figure(&mut self, figure: Box<dyn Paint>) {
        self.figure = figure;
    }
}

// 存档用的结构体 (只认 UUID)
#[derive(Serialize, Deserialize)]
struct SerializedBlock {
    uuid: Uuid,
    children: Vec<Uuid>, // 存档里存的是 UUID 树
                         // data...
}

// 3. 场景管理器 (Arena + 索引)
pub struct SceneGraph {
    // 主存储：Arena (数据的所有者)
    pub blocks: SlotMap<BlockId, RuntimeBlock>,

    // 辅助索引：UUID -> BlockId 的快速查找表
    // 用于解析双链、加载存档、网络同步
    pub uuid_map: HashMap<Uuid, BlockId>,

    pub root: BlockId,
}

impl SceneGraph {
    pub fn new() -> Self {
        let mut blocks = SlotMap::with_key();
        let uuid = Uuid::new_v4();

        let root_id = blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: None,
            figure: Box::new(NullFigure::new()),
        });

        SceneGraph {
            blocks,
            uuid_map: HashMap::new(),
            root: root_id,
        }
    }

    pub fn render(&self) -> render_ctx::RenderContext {
        let mut gc = render_ctx::RenderContext::new();
        self.render_to_context(&mut gc);
        gc
    }
    fn render_to_context(&self, gc: &mut render_ctx::RenderContext) {
        self.traverse_dfs_stack(|block_id| {
            if let Some(runtime_block) = self.blocks.get(block_id) {
                runtime_block.paint(gc);
            }
        })
    }

    pub fn traverse_dfs_stack<F>(&self, mut visitor: F)
    where
        F: FnMut(BlockId),
    {
        let mut stack = Vec::new();
        stack.push(self.root);

        while let Some(node_id) = stack.pop() {
            visitor(node_id);

            if let Some(node) = self.blocks.get(node_id) {
                // 反向压栈以保证正向遍历顺序
                for &child_id in node.children.iter().rev() {
                    stack.push(child_id);
                }
            }
        }
    }

    pub fn new_block(&mut self, parent_id: Option<BlockId>, figure: Box<dyn Paint>) -> BlockId {
        let uuid = Uuid::new_v4(); // 1. 生成身份证

        // 2. 插入 SlotMap，获得运行时 ID
        let id = self.blocks.insert_with_key(|key| RuntimeBlock {
            id: key,
            uuid,
            children: Vec::new(),
            parent: parent_id,
            figure,
            // ...
        });

        // 3. 建立映射索引
        self.uuid_map.insert(uuid, id);

        // 4. 维护树结构 (使用 SlotKey)
        match parent_id {
            Some(pid) => {
                if let Some(parent) = self.blocks.get_mut(pid) {
                    parent.children.push(id);
                }
            }
            None => {
                // parent_id 为 None 时，添加到根节点
                self.blocks[self.root].children.push(id);
            }
        }

        id
    }

    // 1. 保存：运行时 -> 硬盘
    pub fn save(&self) -> String {
        let export_list: Vec<SerializedBlock> = self
            .blocks
            .values()
            .map(|b| {
                SerializedBlock {
                    uuid: b.uuid,
                    // 关键：把 Runtime ID 列表转回 UUID 列表
                    children: b
                        .children
                        .iter()
                        .map(|&cid| self.blocks[cid].uuid)
                        .collect(),
                }
            })
            .collect();

        serde_json::to_string(&export_list).unwrap()
    }

    // 2. 加载：硬盘 -> 运行时 (两步构建法)
    pub fn load(json: &str) -> Self {
        let loaded_data: Vec<SerializedBlock> = serde_json::from_str(json).unwrap();

        let mut scene = SceneGraph::new(); // 初始化空的

        // 第一步：创建所有节点，建立 UUID -> BlockId 映射
        // 此时 children 关系还是空的，因为子节点可能还没创建出来
        for item in &loaded_data {
            let id = scene.blocks.insert_with_key(|key| RuntimeBlock {
                id: key,
                uuid: item.uuid,
                children: Vec::new(), // 先留空
                parent: None,
                figure: Box::new(NullFigure::new()), // TODO 临时占位符
                                                     // ...
            });
            scene.uuid_map.insert(item.uuid, id);
        }

        // 第二步：重建树形关系
        // 现在所有节点都在内存里了，可以通过 UUID 查到 BlockId 了
        for item in &loaded_data {
            let parent_id = scene.uuid_map[&item.uuid];

            for child_uuid in &item.children {
                let child_id = scene.uuid_map[child_uuid];

                // 连接父子 (使用 BlockId)
                scene.blocks[parent_id].children.push(child_id);
                scene.blocks[child_id].parent = Some(parent_id);
            }
        }

        scene
    }
}

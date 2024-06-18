# 可信开源软件大作业：kruskal算法的可视化

# 介绍
本项目构建了一个kruskal算法可视化的小程序

### 运行
```shell
cd kruskal_visualization
cargo run
```

### 功能
- 自定义添加/删除节点
- 自定义添加/删除边
  ![](assets/17186937563962.png)

- 自动布局
- 实时绘制mst
  ![](assets/17186940038973.png)

- 控制台显示结果
  ![](assets/17186940614280.png)

### 依赖
- rustc 1.79.0
- bevy 0.13.2
- bevy_egui 0.27.1

```rust
[package]
name = "kruskal_visualization"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.13.2"
bevy_egui = "0.27.1"
```

### 项目结构
#### main.rs
负责注册子系统和管理game loop
##### gui输入
```rust
#[derive(Resource)]
struct UserEdgeInput {
    new_edge_u : usize,
    new_edge_v : usize,
    new_edge_w : i32,
    new_node_id : usize,
    delete_edge_u : usize,
    delete_edge_v : usize,
    delete_edge_w : i32,
    delete_node_id : usize,
}

impl Default for UserEdgeInput {
    fn default() -> UserEdgeInput {
        UserEdgeInput {
            new_edge_u: 0,
            new_edge_v: 0,
            new_edge_w: 0,
            new_node_id : 0,
            delete_edge_u : 0,
            delete_edge_v : 0,
            delete_edge_w : 0,
            delete_node_id : 0,
        }
    }
}
```
包括需要添加/删除的节点/边
通过gui来控制
##### 注册子系统
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<Graph>()
        .init_resource::<UserEdgeInput>()
        .add_systems(Update , debug_system)
        .add_systems(Update, ui_system)
        .run();
}
```
### 调试系统
```rust
pub fn debug_system(
    graph: ResMut<Graph>
) {
    if graph.is_changed() {
        println!("changed:{:?}" ,graph);
    }
}
```
资源改变了就println!
#### draw.rs
绘图工具库，用egui::painter实现了边和点的绘制
##### 常量
```rust
pub const NODE_RADIUS : f32 = 7.0;
pub const LINE_WIDTH : f32 = 1.0;
pub const LAYOUT_RADIUS : f32 = 100.0;

pub const FONT_COLOR : Color32 = Color32::WHITE;
pub const EDGE_COLOR : Color32 = Color32::WHITE;
pub const NODE_COLOR : Color32 = Color32::BLUE;
pub const MST_EDGE_COLOR : Color32 = Color32::RED;

pub const LAYOUT_CENTER : Vec2 = Vec2::new(600.0, 300.0);

```
包括字体，颜色等绘图信息
##### 绘制点
```rust
fn draw_node(
    painter : &Painter,
    node_id: &usize,
    position: Vec2,
    show_id : bool
) {
    let pos = Pos2::new(position.x, position.y);
    painter.circle_filled(pos, NODE_RADIUS, NODE_COLOR);
    if show_id {
        painter.text(pos, Align2::CENTER_CENTER, node_id.to_string(), FontId::default(), FONT_COLOR);
    }
}
```
##### 绘制边
```rust
fn draw_edge(
    painter : &Painter,
    edge : &Edge,
    (u_position, v_position) : (Vec2, Vec2),
    is_mst : bool,
    show_weight : bool,
) {
    let (point_u, point_v) = (Pos2::new(u_position.x, u_position.y), Pos2::new(v_position.x, v_position.y));
    painter.line_segment([point_u, point_v],
                         Stroke::new(LINE_WIDTH, if is_mst {MST_EDGE_COLOR} else {EDGE_COLOR}));
    if show_weight {
        let point_mid = Pos2::new((u_position.x + v_position.x) / 2.0,
                                  (u_position.y + v_position.y) / 2.0);
        painter.text(point_mid, Align2::CENTER_CENTER, edge.w.to_string(), FontId::default(), FONT_COLOR);
    }
}
```
支持显示边权
##### 自动布局
```rust
fn generate_circle_positions(num_nodes: usize, radius: f32, center : Vec2) -> Vec<Vec2> {
    let mut points = Vec::new();
    for i in 0..num_nodes {
        let angle = 2.0 * PI * (i as f32) / (num_nodes as f32);
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Vec2::new(x, y));
    }
    points
}
```
根据点的数目排列成圆
##### 画图
```rust
pub fn draw_graph(
    painter : &Painter,
    graph: &Graph,
) {
    let pos = generate_circle_positions(graph.nodes.len(), LAYOUT_RADIUS, LAYOUT_CENTER);
    let mut node_position = HashMap::new();
    let mut index = 0;
    for node in &graph.nodes {
        node_position.insert(node, pos[index]);
        draw_node(painter, node, pos[index], true);
        index += 1;
    }
    for edge in &graph.edges {
        let pos_u = node_position[&edge.u];
        let pos_v = node_position[&edge.v];
        draw_edge(painter, edge, (pos_u, pos_v), false, true);
    }
    for edge in &graph.kruskal().edges {
        let pos_u = node_position[&edge.u];
        let pos_v = node_position[&edge.v];
        draw_edge(painter, edge, (pos_u, pos_v), true, false);
    }
}
```
先画点，再画边，最后画mst的边
画点的过程中同时在hashmap记录点的坐标
### ui.rs
管理ui及ui事件
##### 加点和删点
```rust
ui.horizontal(|ui| {
            ui.label("Node ID:");
            ui.add(egui::DragValue::new(&mut user_edge_input.new_node_id));
            if ui.button("Add node").clicked() {
                graph.add_node(user_edge_input.new_node_id);
                let (_, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
                draw_graph(&painter, &graph);
            }
        });
        ui.horizontal(|ui| {
            ui.label("Node ID:");
            ui.add(egui::DragValue::new(&mut user_edge_input.delete_node_id));
            if ui.button("Remove node").clicked() {
                graph.remove_node(user_edge_input.delete_node_id);
                let (_, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
                draw_graph(&painter, &graph);
            }
        });
```
##### 加边和删边
```rust
ui.horizontal(|ui| {
            ui.label("U:");
            ui.add(egui::DragValue::new(&mut user_edge_input.new_edge_u));
            ui.label("V:");
            ui.add(egui::DragValue::new(&mut user_edge_input.new_edge_v));
            ui.label("W:");
            ui.add(egui::DragValue::new(&mut user_edge_input.new_edge_w));
            if ui.button("Add edge").clicked() {
                graph.add_edge(user_edge_input.new_edge_u, user_edge_input.new_edge_v, user_edge_input.new_edge_w);
                let (_, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
                draw_graph(&painter, &graph);
            }
        });
        ui.horizontal(|ui| {
            ui.label("U:");
            ui.add(egui::DragValue::new(&mut user_edge_input.delete_edge_u));
            ui.label("V:");
            ui.add(egui::DragValue::new(&mut user_edge_input.delete_edge_v));
            ui.label("W:");
            ui.add(egui::DragValue::new(&mut user_edge_input.delete_edge_w));
            if ui.button("Remove edge").clicked() {
                graph.remove_edge(user_edge_input.delete_edge_u, user_edge_input.delete_edge_v, user_edge_input.delete_edge_w);
                let (_, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
                draw_graph(&painter, &graph);
            }
        });
```
控件每改变一次重新绘图
### graph.rs
实现图的数据结构类，以及kruskal算法
##### 定义
```rust
pub const MAX_NODE_INDEX : usize = 1000;

#[derive(Resource, Debug)]
pub struct Graph {
    pub edges : Vec<Edge>,
    pub nodes : Vec<usize>,
}

impl Default for Graph {
    fn default() -> Graph {
        Graph {
            edges : vec![],
            nodes: vec![],
        }
    }
}
```
图由点和边构成
##### 成员函数
```rust
impl Graph {
    pub fn new(edge_: Vec<Edge>, nodes_: Vec<u32>) -> Graph {
        Graph {edges : vec![], nodes: vec![] }
    }

    pub fn add_edge (&mut self, u : usize, v : usize, w : i32) {
        if self.nodes.contains(&u) && self.nodes.contains(&v) {
            self.edges.push(Edge::new(u, v, w));
        }
    }

    pub fn remove_edge (&mut self, u : usize, v : usize, w : i32) {
        self.edges.retain(|edge|
            edge.u != u || edge.v != v || edge.w != w
        );
    }

    pub fn add_node (&mut self, id : usize) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }

    pub fn remove_node (&mut self, id : usize) {
        self.edges.retain(|edge|
            edge.u != id && edge.v != id
        );
        self.nodes.retain(|node_id|
            *node_id != id
        );
    }

    pub fn node_max_id (&self) -> usize {
        *self.nodes.iter().max().unwrap()
    }

    pub fn is_connected (&self) -> bool {
        let mut uf = UnionFind::new(self.node_max_id());
        for edge in self.edges.iter() {
            uf.union(edge.u, edge.v);
        }

        let id = uf.find(self.nodes[0]);
        for node in self.nodes.iter() {
            let cid = uf.find(*node);
            if cid != id {
                return false;
            }
        }
        return true;
    }

    pub fn kruskal(&self) -> Graph {
        let mut edges = vec![];
        for edge in self.edges.iter() {
            edges.push(Edge {
                u: edge.u,
                v: edge.v,
                w: edge.w,
            })
        }
        edges.sort();

        let mut uf = UnionFind::new(MAX_NODE_INDEX);
        let mut mst = Graph::default();
        for edge in edges.iter() {
            if uf.union(edge.u, edge.v) {
                mst.add_node(edge.u);
                mst.add_node(edge.v);
                mst.add_edge(edge.u, edge.v, edge.w);
            }
        }
        mst
    }
}
```
包括点和边的修改，以及最小生成树的计算方法
### edge.rs
实现边的数据结构类
##### 数据结构
```rust
#[derive(Debug)]
pub struct Edge {
    pub u : usize,
    pub v : usize,
    pub w : i32,
}
```
边的起点和终点，和边权
##### 实现的特征
```rust
impl Edge {
    pub fn new(u_ : usize, v_ : usize, w_ : i32) -> Edge {
        Edge { u : u_, v : v_, w : w_ }
    }
}

impl Default for Edge {
    fn default() -> Edge {
        Edge { u : 0, v : 0, w : 0 }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl Eq for Edge { }
```
定义了相等和偏序关系，用于sort
### union_find.rs
实现并查集，支持按秩合并
##### 数据结构
```rust
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
}
```
带权并查集
##### 成员函数
```rust
impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size as usize],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); 
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }
}
```
包括按秩合并，合并成功返回true反之false
以及路径压缩的find函数

# project
[https://github.com/annajcy/krusal_visualization.git](https://github.com/annajcy/krusal_visualization.git)

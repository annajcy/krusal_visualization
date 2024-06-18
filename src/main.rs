mod draw;
mod ui;
mod graph;
mod edge;
mod union_find;

use bevy::prelude::*;
use bevy_egui::*;

use crate::graph::Graph;
use crate::ui::*;

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

pub fn debug_system(
    graph: ResMut<Graph>
) {
    if graph.is_changed() {
        println!("changed:{:?}" ,graph);
    }
}

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
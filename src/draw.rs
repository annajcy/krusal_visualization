use std::collections::HashMap;
use std::vec::Vec;
use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Color32, FontId, Painter, Pos2, Stroke};

use crate::edge::Edge;
use crate::graph::{Graph};

pub const NODE_RADIUS : f32 = 7.0;
pub const LINE_WIDTH : f32 = 1.0;
pub const LAYOUT_RADIUS : f32 = 100.0;

pub const FONT_COLOR : Color32 = Color32::WHITE;
pub const EDGE_COLOR : Color32 = Color32::BLACK;
pub const NODE_COLOR : Color32 = Color32::BLUE;
pub const MST_EDGE_COLOR : Color32 = Color32::RED;

pub const LAYOUT_CENTER : Vec2 = Vec2::new(600.0, 300.0);


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




use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::Sense;

use crate::graph::Graph;
use crate::UserEdgeInput;
use crate::draw::*;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut user_edge_input: ResMut<UserEdgeInput>,
    mut graph : ResMut<Graph>
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
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
        ui.horizontal(|ui| {
            if ui.button("Calculate MST").clicked() {
                let mst = graph.kruskal();
                println!("mst:{:?}", mst);
            }
        });
        let (_, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
        draw_graph(&painter, &graph);
    });

}
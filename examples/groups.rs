//! Demonstrates a larger graph organized into Unreal-Engine-style comment/group boxes,
//! showing how nodes are dragged and resized together as members of a group.
#[path = "common/mod.rs"]
mod common;

use common::{Attribute, Group, Node, build_group, build_node};
use eframe::{self, egui};
use egui_nodes::{Context, GroupArgs, GroupConstructor, LinkArgs, NodeArgs, NodeConstructor};

const GROUP_COUNT: usize = 5;
const NODES_PER_GROUP: usize = 6;
const GROUP_SPACING_X: f32 = 260.0;
const NODE_SPACING_Y: f32 = 70.0;
const GROUP_PADDING_TOP: f32 = 50.0;

/// Node ids are `group_idx * NODES_PER_GROUP + node_in_group`, so they stay dense and
/// collision-free across every group. Group ids live in a disjoint range above them.
fn node_id(group_idx: usize, node_in_group: usize) -> usize {
    group_idx * NODES_PER_GROUP + node_in_group
}

fn group_id(group_idx: usize) -> usize {
    100_000 + group_idx
}

/// Each node has one input pin and one output pin; pin ids are derived from the node id
/// so they never collide with pin ids belonging to other nodes.
fn input_pin_id(id: usize) -> usize {
    id * 2
}

fn output_pin_id(id: usize) -> usize {
    id * 2 + 1
}

struct MyApp {
    ctx: Context,
    nodes: Vec<Node>,
    groups: Vec<Group>,
    links: Vec<(usize, usize)>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let mut groups = Vec::new();
        // Chain every node's output to the next node's input, both within a group and
        // across the gap between consecutive groups, so the whole graph is one long path.
        let mut links = Vec::new();
        let mut prev: Option<usize> = None;

        for group_idx in 0..GROUP_COUNT {
            let mut member_ids = Vec::with_capacity(NODES_PER_GROUP);
            for node_in_group in 0..NODES_PER_GROUP {
                let id = node_id(group_idx, node_in_group);
                if let Some(prev_id) = prev {
                    links.push((output_pin_id(prev_id), input_pin_id(id)));
                }
                prev = Some(id);
                member_ids.push(id);

                nodes.push(Node {
                    id,
                    title: format!("Node {group_idx}.{node_in_group}"),
                    origin: egui::pos2(
                        50.0 + group_idx as f32 * GROUP_SPACING_X,
                        30.0 + GROUP_PADDING_TOP + node_in_group as f32 * NODE_SPACING_Y,
                    ),
                    args: NodeArgs::new(),
                    attributes: vec![
                        Attribute::input(input_pin_id(id), "in", Default::default()),
                        Attribute::output(output_pin_id(id), "out", Default::default()),
                    ],
                });
            }

            let height = GROUP_PADDING_TOP + NODES_PER_GROUP as f32 * NODE_SPACING_Y + 20.0;
            groups.push(Group {
                id: group_id(group_idx),
                title: format!("Group {group_idx}"),
                origin: egui::pos2(30.0 + group_idx as f32 * GROUP_SPACING_X, 30.0),
                size: egui::vec2(220.0, height),
                args: GroupArgs {
                    outline: Some(egui::Color32::LIGHT_BLUE),
                    ..GroupArgs::new()
                },
                member_ids,
            });
        }

        Self {
            ctx: Context::default(),
            nodes,
            groups,
            links,
        }
    }
}

fn example_graph(
    ctx: &mut Context,
    nodes: &mut [Node],
    groups: &[Group],
    links: &mut Vec<(usize, usize)>,
    ui: &mut egui::Ui,
) {
    let group_constructors: Vec<GroupConstructor> = groups.iter().map(build_group).collect();
    let node_constructors: Vec<NodeConstructor> = nodes.iter_mut().map(build_node).collect();

    ctx.show(
        group_constructors,
        node_constructors,
        links.iter().enumerate().map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
        ui,
    );

    if let Some(idx) = ctx.link_destroyed() {
        links.remove(idx);
    }

    if let Some((start, end, _)) = ctx.link_created() {
        links.push((start, end))
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!(
                "{} nodes across {} groups",
                self.nodes.len(),
                self.groups.len()
            ));
            example_graph(
                &mut self.ctx,
                &mut self.nodes,
                &self.groups,
                &mut self.links,
                ui,
            );
        });
    }
}

fn main() {
    eframe::run_native(
        "egui_nodes groups example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
    .unwrap();
}

#[path = "common/mod.rs"]
mod common;

use common::{Attribute, Node, build_node};
use eframe::{self, egui};
use egui_nodes::{
    Context, GroupConstructor, LinkArgs, NodeArgs, NodeConstructor, PinArgs, PinShape,
};

struct MyApp {
    ctx: Context,
    nodes: Vec<Node>,
    links: Vec<(usize, usize)>,
}

impl Default for MyApp {
    fn default() -> Self {
        let nodes = vec![
            Node {
                id: 0,
                title: "Example Node A".to_string(),
                origin: [50.0, 150.0].into(),
                args: NodeArgs {
                    outline: Some(egui::Color32::LIGHT_BLUE),
                    ..NodeArgs::new()
                },
                attributes: vec![
                    Attribute::input(
                        0,
                        "Input",
                        PinArgs {
                            shape: PinShape::Triangle,
                            ..PinArgs::new()
                        },
                    ),
                    Attribute::static_attr(1, "Can't Connect to Me"),
                    Attribute::output(
                        2,
                        "Output",
                        PinArgs {
                            shape: PinShape::TriangleFilled,
                            ..PinArgs::new()
                        },
                    ),
                    {
                        let mut gain = 0.5f32;
                        Attribute::static_widget(6, move |ui| {
                            ui.add(
                                egui::Slider::new(&mut gain, 0.0..=1.0)
                                    .show_value(false)
                                    .text("Gain"),
                            )
                        })
                    },
                    {
                        let mut enabled = true;
                        Attribute::static_widget(7, move |ui| ui.checkbox(&mut enabled, "Enabled"))
                    },
                ],
            },
            Node {
                id: 1,
                title: "Example Node B".to_string(),
                origin: [225.0, 150.0].into(),
                args: NodeArgs::new(),
                attributes: vec![
                    Attribute::static_attr(3, "Can't Connect to Me"),
                    Attribute::output(4, "Output", PinArgs::new()),
                    Attribute::input(5, "Input", PinArgs::new()),
                ],
            },
        ];
        Self {
            ctx: Context::default(),
            nodes,
            links: Vec::new(),
        }
    }
}

fn example_graph(
    ctx: &mut Context,
    nodes: &mut [Node],
    links: &mut Vec<(usize, usize)>,
    ui: &mut egui::Ui,
) {
    let node_constructors: Vec<NodeConstructor> = nodes.iter_mut().map(build_node).collect();

    ctx.show(
        Vec::<GroupConstructor>::new(),
        node_constructors,
        links.iter().enumerate().map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
        ui,
    );

    // remove destroyed links
    if let Some(idx) = ctx.link_destroyed() {
        links.remove(idx);
    }

    // add created links
    if let Some((start, end, _)) = ctx.link_created() {
        links.push((start, end))
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("My egui Application");
            example_graph(&mut self.ctx, &mut self.nodes, &mut self.links, ui);
        });
    }
}

fn main() {
    eframe::run_native(
        "My egui app",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
    .unwrap();
}

//! Shared plain-data node/group definitions and constructor builders used by every
//! example in this crate. Not part of the published library — `egui_nodes::NodeConstructor`
//! / `GroupConstructor` are frame-scoped builders (their title/attribute callbacks borrow
//! the current `Ui` pass) and can't be stored across frames, so examples instead keep this
//! plain data around and turn it into fresh constructors every frame via [`build_node`] /
//! [`build_group`].
//!
//! Each example only exercises a subset of this module (e.g. `simple` never builds a
//! group, `groups` never uses a static attribute), so unused items are expected here.
#![allow(dead_code)]
use egui::Ui;
use egui_nodes::{GroupArgs, GroupConstructor, NodeArgs, NodeConstructor, PinArgs};

#[derive(Clone, Copy)]
pub enum AttributeKind {
    Input,
    Output,
    Static,
}

/// What an attribute row renders. `Label` is the common case (static, read-only text);
/// `Widget` lets a node host any interactive egui control (slider, checkbox, text edit,
/// button, ...) by owning whatever state the control mutates and rendering it each frame.
pub enum AttributeContent {
    Label(&'static str),
    Widget(Box<dyn FnMut(&mut Ui) -> egui::Response>),
}

/// A single connectable (or non-connectable) attribute row on a node.
pub struct Attribute {
    pub id: usize,
    pub kind: AttributeKind,
    pub content: AttributeContent,
    pub pin_args: PinArgs,
}

impl Attribute {
    pub fn input(id: usize, label: &'static str, pin_args: PinArgs) -> Self {
        Self {
            id,
            kind: AttributeKind::Input,
            content: AttributeContent::Label(label),
            pin_args,
        }
    }

    pub fn output(id: usize, label: &'static str, pin_args: PinArgs) -> Self {
        Self {
            id,
            kind: AttributeKind::Output,
            content: AttributeContent::Label(label),
            pin_args,
        }
    }

    pub fn static_attr(id: usize, label: &'static str) -> Self {
        Self {
            id,
            kind: AttributeKind::Static,
            content: AttributeContent::Label(label),
            pin_args: PinArgs::new(),
        }
    }

    /// A non-connectable attribute row that renders an arbitrary, stateful egui control.
    /// `render` owns (via capture) whatever value it edits, e.g. a slider closing over a
    /// `f32` field stored elsewhere in the app.
    pub fn static_widget(
        id: usize,
        render: impl FnMut(&mut Ui) -> egui::Response + 'static,
    ) -> Self {
        Self {
            id,
            kind: AttributeKind::Static,
            content: AttributeContent::Widget(Box::new(render)),
            pin_args: PinArgs::new(),
        }
    }

    /// A connectable input attribute row that also renders an arbitrary control (e.g. a
    /// numeric pin that's still editable by hand when nothing is plugged into it).
    pub fn input_widget(
        id: usize,
        pin_args: PinArgs,
        render: impl FnMut(&mut Ui) -> egui::Response + 'static,
    ) -> Self {
        Self {
            id,
            kind: AttributeKind::Input,
            content: AttributeContent::Widget(Box::new(render)),
            pin_args,
        }
    }

    /// A connectable output attribute row that also renders an arbitrary control (e.g. a
    /// numeric pin that's still editable by hand when nothing is plugged into it).
    pub fn output_widget(
        id: usize,
        pin_args: PinArgs,
        render: impl FnMut(&mut Ui) -> egui::Response + 'static,
    ) -> Self {
        Self {
            id,
            kind: AttributeKind::Output,
            content: AttributeContent::Widget(Box::new(render)),
            pin_args,
        }
    }
}

/// Persistent, app-owned description of a node.
pub struct Node {
    pub id: usize,
    pub title: String,
    pub origin: egui::Pos2,
    pub args: NodeArgs,
    pub attributes: Vec<Attribute>,
}

/// Persistent, app-owned description of a comment/group box.
pub struct Group {
    pub id: usize,
    pub title: String,
    pub origin: egui::Pos2,
    pub size: egui::Vec2,
    pub args: GroupArgs,
    pub member_ids: Vec<usize>,
}

/// Build a fresh, frame-scoped `NodeConstructor` from a stored `Node`. Takes `&mut Node`
/// (rather than `&Node`) because `AttrContent::Widget` closures are `FnMut` — a slider,
/// checkbox, etc. needs mutable access to the value it edits every time it's drawn.
pub fn build_node(node: &mut Node) -> NodeConstructor<'_> {
    let title = node.title.clone();
    let mut ctor = NodeConstructor::new(node.id, node.args)
        .with_origin(node.origin)
        .with_title(move |ui: &mut Ui| ui.label(title));
    for attr in &mut node.attributes {
        let id = attr.id;
        let pin_args = attr.pin_args;
        let content = &mut attr.content;
        let render = move |ui: &mut Ui| match content {
            AttributeContent::Label(text) => ui.label(*text),
            AttributeContent::Widget(render) => render(ui),
        };
        ctor = match attr.kind {
            AttributeKind::Input => ctor.with_input_attribute(id, pin_args, render),
            AttributeKind::Output => ctor.with_output_attribute(id, pin_args, render),
            AttributeKind::Static => ctor.with_static_attribute(id, render),
        };
    }
    ctor
}

/// Build a fresh, frame-scoped `GroupConstructor` from a stored `GroupDef`.
pub fn build_group(group: &Group) -> GroupConstructor<'_> {
    GroupConstructor::new(group.id, group.args)
        .with_title(move |ui: &mut Ui| ui.label(&group.title))
        .with_nodes(group.member_ids.iter().copied())
        .with_origin(group.origin)
        .with_size(group.size)
}

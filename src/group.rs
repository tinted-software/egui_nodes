use super::*;
use educe::Educe;

#[derive(Default, Clone, Copy, Debug)]
/// The Style of a Group. If feilds are None then the Context style is used
pub struct GroupArgs {
    pub background: Option<egui::Color32>,
    pub background_hovered: Option<egui::Color32>,
    pub background_selected: Option<egui::Color32>,
    pub outline: Option<egui::Color32>,
    pub titlebar: Option<egui::Color32>,
    pub titlebar_hovered: Option<egui::Color32>,
    pub titlebar_selected: Option<egui::Color32>,
    pub corner_rounding: Option<f32>,
    pub padding: Option<egui::Vec2>,
    pub border_thickness: Option<f32>,
}

impl GroupArgs {
    pub const fn new() -> Self {
        Self {
            background: None,
            background_hovered: None,
            background_selected: None,
            outline: None,
            titlebar: None,
            titlebar_hovered: None,
            titlebar_selected: None,
            corner_rounding: None,
            padding: None,
            border_thickness: None,
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct GroupDataColorStyle {
    pub background: egui::Color32,
    pub background_hovered: egui::Color32,
    pub background_selected: egui::Color32,
    pub outline: egui::Color32,
    pub titlebar: egui::Color32,
    pub titlebar_hovered: egui::Color32,
    pub titlebar_selected: egui::Color32,
}

#[derive(Default, Debug)]
pub struct GroupDataLayoutStyle {
    pub corner_rounding: f32,
    pub padding: egui::Vec2,
    pub border_thickness: f32,
}

/// Comment-box style node group. Members are dragged together when the
/// group's title bar is dragged. Membership is explicit (caller-supplied
/// each frame via [`GroupConstructor::with_nodes`]), not spatial containment.
#[derive(Educe)]
#[educe(Debug)]
pub(crate) struct GroupData {
    pub id: usize,
    pub origin: egui::Pos2,
    pub size: egui::Vec2,
    pub rect: egui::Rect,
    pub title_bar_rect: egui::Rect,
    #[educe(Debug(ignore))]
    pub color_style: GroupDataColorStyle,
    pub layout_style: GroupDataLayoutStyle,
    pub member_node_ids: Vec<usize>,
    pub member_node_indices: Vec<usize>,
    pub draggable: bool,
    pub resizable: bool,
    #[educe(Debug(ignore))]
    pub titlebar_shape: Option<egui::layers::ShapeIdx>,
    #[educe(Debug(ignore))]
    pub background_shape: Option<egui::layers::ShapeIdx>,
    #[educe(Debug(ignore))]
    pub outline_shape: Option<egui::layers::ShapeIdx>,
}

impl GroupData {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            origin: [100.0; 2].into(),
            size: [200.0, 150.0].into(),
            rect: [[0.0; 2].into(); 2].into(),
            title_bar_rect: [[0.0; 2].into(); 2].into(),
            color_style: Default::default(),
            layout_style: Default::default(),
            member_node_ids: Default::default(),
            member_node_indices: Default::default(),
            draggable: true,
            resizable: true,
            titlebar_shape: None,
            background_shape: None,
            outline_shape: None,
        }
    }
}

impl Default for GroupData {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Id for GroupData {
    fn id(&self) -> usize {
        self.id
    }
    fn new(id: usize) -> Self {
        GroupData::new(id)
    }
}

/// Used to construct a comment-box node group and stores the relevant ui code for its title.
/// Groups are always rendered behind nodes; dragging the group's title bar drags every
/// member node along with it.
#[derive(Educe, Default)]
#[educe(Debug)]
#[allow(clippy::type_complexity)]
pub struct GroupConstructor<'a> {
    pub(crate) id: usize,
    #[educe(Debug(ignore))]
    pub(crate) title: Option<Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>>,
    pub(crate) member_ids: Vec<usize>,
    pub(crate) pos: Option<egui::Pos2>,
    pub(crate) size: Option<egui::Vec2>,
    pub(crate) args: GroupArgs,
}

impl<'a> GroupConstructor<'a> {
    /// Create a new group to be displayed in a Context.
    /// id should be the same accross frames and should not be the same as any other currently used group
    pub fn new(id: usize, args: GroupArgs) -> Self {
        Self {
            id,
            args,
            ..Default::default()
        }
    }

    /// Add a title to a group
    pub fn with_title(mut self, title: impl FnOnce(&mut egui::Ui) -> egui::Response + 'a) -> Self {
        self.title.replace(Box::new(title));
        self
    }

    /// Add node ids that belong to this group. Dragging the group's title bar moves
    /// every listed member node along with it. Ids that don't match a currently
    /// displayed node are silently ignored.
    pub fn with_nodes(mut self, ids: impl IntoIterator<Item = usize>) -> Self {
        self.member_ids.extend(ids);
        self
    }

    /// Set the position of the group in screen space when it is first created.
    /// To modify it after creation use one of the set_group_pos methods of the Context
    pub fn with_origin(mut self, origin: egui::Pos2) -> Self {
        self.pos.replace(origin);
        self
    }

    /// Set the size of the group when it is first created.
    /// To modify it after creation use set_group_size on the Context
    pub fn with_size(mut self, size: egui::Vec2) -> Self {
        self.size.replace(size);
        self
    }

    /// Get the id of this GroupConstructor
    pub fn id(&self) -> usize {
        self.id
    }
}

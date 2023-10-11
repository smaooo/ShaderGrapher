use eframe::egui;
use egui_node_graph::{
    DataTypeTrait, Graph, GraphEditorState, NodeDataTrait, NodeId, NodeResponse, NodeTemplateIter,
    NodeTemplateTrait, UserResponseTrait, WidgetValueTrait,
};
use std::borrow::Cow;

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
enum SGNodeTemplate {
    Add,
    Subtract,
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for SGNodeTemplate {
    type NodeData = SGNodeData;
    type DataType = SGDataType;
    type ValueType = SGValueType;
    type UserState = SGGraphState;
    // TODO: What is this?
    type CategoryType = &'static str;

    fn node_finder_label(&self, user_state: &mut Self::UserState) -> std::borrow::Cow<'_, str> {
        // TODO: Update labels later
        Cow::Borrowed(match self {
            SGNodeTemplate::Add => "Add",
            SGNodeTemplate::Subtract => "Subtract",
        })
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<&'static str> {
        // TODO: Update categories later
        match self {
            SGNodeTemplate::Add | SGNodeTemplate::Subtract => vec!["Math"],
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    // TODO: What is this?
    fn user_data(&self, user_state: &mut Self::UserState) -> Self::NodeData {
        SGNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.

        // TODO: Implement this Later
    }
}
/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
struct SGNodeData {
    template: SGNodeTemplate,
}

impl NodeDataTrait for SGNodeData {
    type Response = SGResponse;
    type UserState = SGGraphState;
    type DataType = SGDataType;
    type ValueType = SGValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<SGNodeData, SGDataType, SGValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<SGResponse, SGNodeData>>
    where
        Self::Response: UserResponseTrait,
    {
        let responses = vec![];

        responses
    }
}

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible data types.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
enum SGDataType {
    // TODO: Should change these later
    Float,
    Vec2,
}

/// Defines the visual and name behavior of data types
impl DataTypeTrait<SGGraphState> for SGDataType {
    fn data_type_color(&self, user_state: &mut SGGraphState) -> egui::Color32 {
        // TODO: Update colors later
        match self {
            SGDataType::Float => egui::Color32::from_rgb(0, 0, 255),
            SGDataType::Vec2 => egui::Color32::from_rgb(0, 255, 0),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        // TODO: Update names later
        match self {
            SGDataType::Float => Cow::Borrowed("float"),
            SGDataType::Vec2 => Cow::Borrowed("2d vector"),
        }
    }
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
enum SGValueType {
    // TODO: Should change these later
    Scalar(f32),
    Vec2(egui::Vec2),
}

impl Default for SGValueType {
    // NOTE: This is just a dummy `Default` implementation. The library
    // requires it to circumvent some internal borrow checker issues.
    fn default() -> Self {
        todo!()
    }
}
// This trait is used to tell the library which UI to display for the
// inline parameter widgets.
impl WidgetValueTrait for SGValueType {
    type Response = SGResponse;
    type UserState = SGGraphState;
    type NodeData = SGNodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        // TODO: Implement this later
        todo!()
    }
}
/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

impl UserResponseTrait for SGResponse {}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct SGGraphState {
    active_node: Option<NodeId>,
}

struct AllSGNodeTemplates;
impl NodeTemplateIter for AllSGNodeTemplates {
    type Item = SGNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // TODO: Fill with node types
        vec![SGNodeTemplate::Add, SGNodeTemplate::Subtract]
    }
}
type SGGraph = Graph<SGNodeData, SGDataType, SGValueType>;
type EditorState =
    GraphEditorState<SGNodeData, SGDataType, SGValueType, SGNodeTemplate, SGGraphState>;

#[derive(Default)]
pub struct ShaderGrapher {
    state: EditorState,

    user_state: SGGraphState,
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "egui_node_graph";

#[cfg(feature = "persistence")]
impl ShaderGrapher {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            state,
            user_state: MyGraphState::default(),
        }
    }
}

impl eframe::App for ShaderGrapher {
    #[cfg(feature = "persistence")]
    /// If the persistence function is enabled,
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.draw_graph_editor(
                    ui,
                    AllSGNodeTemplates,
                    &mut self.user_state,
                    Vec::default(),
                )
            })
            .inner;
    }
}

// TODO: Implement this
// fn evaluate_node() {}

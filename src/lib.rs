mod rendered_node;
use lyon::{
    geom::{euclid::Point2D, Box2D, Point},
    path::{traits::Build, Builder, Path, Winding},
};
pub use rendered_node::{Margins, NodeElement, RenderedNode, SizedText};
use std::{collections::HashMap, fmt::Display, hash::Hash};
use tree_ds::prelude::Tree;
#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub trait Graphable: Eq + Clone + Ord // T: Clone,
{
    type ID: Clone;
    type Font: Clone;
    fn set_text(
        &self,
        font: Self::Font,
        max_width: Option<u32>,
        max_height: Option<u32>,
        position: Vec2,
        size: f32,
    ) -> Option<SizedText<Self::Font>>;
    fn get_links(&self) -> Vec<Self::ID>;
}
fn layout_node<ID, Content>(
    sub_tree_root: &ID,
    tree: &Tree<ID, Content>,
    font: <Content as Graphable>::Font,
    margins: &Margins,
    depth: u32,
    max_depth: Option<u32>,
    position: Vec2,
    text_size: f32,
) -> Result<
    (
        HashMap<ID, RenderedNode<ID, <Content as Graphable>::Font>>,
        Vec2,
    ),
    String,
>
where
    ID: Clone + Display + Eq + Hash + Ord,
    Content: Clone + Eq + Ord + Graphable<ID = ID>,
    // <Content as Graphable>::T: Graphable,
{
    let mut node_list = tree
        .get_node_by_id(sub_tree_root)
        .ok_or("Node not found")?
        .get_children_ids();
    let text_position = Vec2::new(
        position.x + margins.inner_margins.x,
        position.y + margins.inner_margins.y,
    );
    let node_content: Content = tree
        .get_node_by_id(&sub_tree_root)
        .ok_or("Tree broken")?
        .get_value()
        .ok_or("Empty node")?;
    let sized_text = node_content
        .set_text(font.clone(), None, None, text_position, text_size)
        .ok_or("Could not set text")?;
    // let (line_widths, paragraph_height) =
    //     text_dimensions(&text.content, font, text.size, LineHeight::Relative(1.25));
    let self_dimension = Vec2 {
        x: sized_text.dimensions.x + margins.inner_margins.x * 2.0,
        y: sized_text.dimensions.y + margins.inner_margins.y * 2.0,
    };
    let mut rendered_nodes: HashMap<ID, RenderedNode<ID, <Content as Graphable>::Font>> =
        HashMap::new();
    let mut rendered_node = RenderedNode {
        node_elements: vec![],
        dimensions: self_dimension,
        position: position.clone(),
        node_links: node_content.get_links(),
    };
    let mut subnode_position = position.clone();
    subnode_position.x += &margins.inner_margins.x;
    subnode_position.y += sized_text.dimensions.y + (&margins.inner_margins.y * 2.0);
    for node in node_list {
        let (mut rendered_subnodes, subnode_dimensions) = layout_node(
            &node,
            &tree,
            font.clone(),
            &margins,
            depth + 1,
            max_depth,
            Vec2 {
                x: subnode_position.x,
                y: subnode_position.y,
            },
            text_size,
        )?;
        rendered_nodes.extend(rendered_subnodes);
        subnode_position.y += subnode_dimensions.y + &margins.inner_margins.y;
        let minimum_outer_dimensions = subnode_dimensions.x + (margins.outer_margins.x * 2.00);
        if minimum_outer_dimensions > rendered_node.dimensions.x {
            rendered_node.dimensions.x = minimum_outer_dimensions;
        }
        rendered_node.dimensions.y += subnode_dimensions.y + margins.outer_margins.y;
    }

    let mut path_builder = Path::builder();
    path_builder.add_rectangle(
        &Box2D::new(
            Point2D::new(position.x, position.y),
            Point2D::new(
                position.x + rendered_node.dimensions.x,
                position.y + rendered_node.dimensions.y,
            ),
        ),
        Winding::Positive,
    );
    rendered_node
        .node_elements
        .push(NodeElement::Path(path_builder.build()));
    rendered_node
        .node_elements
        .push(NodeElement::Text(sized_text));
    let dimensions = rendered_node.dimensions.clone();
    rendered_nodes.insert(sub_tree_root.clone(), rendered_node);
    Ok((rendered_nodes, (dimensions)))
}

pub fn graph_layer_tree<ID, Content>(
    font: <Content as Graphable>::Font,
    tree: Tree<ID, Content>,
    margins: &Margins,
    position: Vec2,

    text_size: f32,
) -> Result<HashMap<ID, RenderedNode<ID, <Content as Graphable>::Font>>, String>
where
    Content: Clone + Eq + Graphable<ID = ID>,
    ID: Clone + Eq + Hash,
    ID: Ord,
    ID: std::fmt::Display,
{
    let root_layer = tree.get_root_node().ok_or("Empty tree".to_owned())?;
    let mut rendered_nodes: HashMap<ID, RenderedNode<ID, <Content as Graphable>::Font>> =
        HashMap::new();
    let (rendered_subnodes, _) = layout_node(
        &root_layer.get_node_id(),
        &tree,
        font,
        &margins,
        0,
        None,
        position,
        text_size,
    )?;
    rendered_nodes.extend(rendered_subnodes);
    Ok(rendered_nodes)
}

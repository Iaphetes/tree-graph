use lyon::{
    geom::{euclid::Point2D, Box2D, Point},
    path::{traits::Build, Builder, Path, Winding},
};
use std::{collections::HashMap, fmt::Display, hash::Hash};
use tree_ds::prelude::Tree;
#[derive(Clone, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
#[derive(Clone, Debug)]
pub struct SizedText<Font> {
    pub content: String,
    pub dimensions: Vec2,
    pub font: Font,
}
#[derive(Clone, Debug)]
enum NodeElement<Font> {
    Path(Path),
    Text(SizedText<Font>),
}
pub struct Margins {
    pub inner_margins: Vec2,
    pub outer_margins: Vec2,
}
pub trait Graphable: Eq + Clone + Ord {
    fn set_text<Font>(
        &self,
        font: Font,
        max_width: Option<u32>,
        max_height: Option<u32>,
        position: Vec2,
    ) -> Option<SizedText<Font>>;
    fn get_links<T>(&self) -> Vec<T>;
}
#[derive(Clone, Debug)]
pub struct RenderedNode<T, Font>
where
    T: Clone,
    Font: Clone,
{
    pub position: Vec2,
    pub node_elements: Vec<NodeElement<Font>>,
    pub node_links: Vec<T>,
    pub dimensions: Vec2,
}
fn layout_node<Q, T, Font>(
    sub_tree_root: &Q,
    tree: &Tree<Q, T>,
    font: Font,
    margins: &Margins,
    depth: u32,
    max_depth: Option<u32>,
    position: Vec2,
) -> Result<(HashMap<Q, RenderedNode<T, Font>>, Vec2), String>
where
    Q: Clone + Display + Eq + Hash + Ord,
    T: Clone + Eq + Graphable,
    Font: Clone,
{
    let mut node_list = tree
        .get_node_by_id(sub_tree_root)
        .ok_or("Node not found")?
        .get_children_ids();
    let text_position = Vec2::new(
        position.x + margins.inner_margins.x,
        position.y + margins.inner_margins.y,
    );
    let node_content: T = tree
        .get_node_by_id(&sub_tree_root)
        .ok_or("Tree broken")?
        .get_value()
        .ok_or("Empty node")?;
    let sized_text = node_content
        .set_text(font.clone(), None, None, text_position)
        .ok_or("Could not set text")?;
    // let (line_widths, paragraph_height) =
    //     text_dimensions(&text.content, font, text.size, LineHeight::Relative(1.25));
    let self_dimension = Vec2 {
        x: sized_text.dimensions.x + margins.inner_margins.x * 2.0,
        y: sized_text.dimensions.y + margins.inner_margins.y * 2.0,
    };
    let mut rendered_nodes: HashMap<Q, RenderedNode<T, Font>> = HashMap::new();
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

pub fn graph_layer_tree<Q, T, Font>(
    font: Font,
    tree: Tree<Q, T>,
    margins: &Margins,
    position: Vec2,
) -> Result<HashMap<Q, RenderedNode<T, Font>>, String>
where
    T: Clone + Eq + Graphable,
    Q: Clone + Eq + Hash,
    Q: Ord,
    Q: std::fmt::Display,
    Font: Clone,
{
    let root_layer = tree.get_root_node().ok_or("Empty tree".to_owned())?;
    let mut rendered_nodes: HashMap<Q, RenderedNode<T, Font>> = HashMap::new();
    let (rendered_subnodes, _) = layout_node(
        &root_layer.get_node_id(),
        &tree,
        font,
        &margins,
        0,
        None,
        position,
    )?;
    rendered_nodes.extend(rendered_subnodes);
    Ok(rendered_nodes)
}

// fn character_width(character: char, _font: Font, size: Pixels) -> Pixels {
//     size * 0.75
// }
// fn text_dimensions(text: &str, _font: Font, size: f32, line_height: f32) -> (Vec<Pixels>, Pixels) {
//     let line_widths: Vec<Pixels> = text
//         .split('\n')
//         .map(|line| {
//             let mut line_width: Pixels = Pixels(0.0);
//             for character in line.chars() {
//                 line_width.0 += character_width(character, _font, size).0
//             }
//             line_width
//         })
//         .collect();
//     let height: Pixels = Pixels(
//         line_widths.len() as f32
//             * match line_height {
//                 LineHeight::Relative(rel) => rel * size.0,
//                 LineHeight::Absolute(pixels) => pixels.0,
//             },
//     );

//     (line_widths, height)
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

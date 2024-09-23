use lyon::path::Path;

use crate::Vec2;
#[derive(Clone, Debug)]
pub struct SizedText<Font> {
    pub content: String,
    pub dimensions: Vec2,
    pub position: Vec2,
    pub font: Font,
    pub size: f32,
}
#[derive(Clone, Debug)]
pub enum NodeElement<Font> {
    Path(Path),
    Text(SizedText<Font>),
}
pub struct Margins {
    pub inner_margins: Vec2,
    pub outer_margins: Vec2,
}
#[derive(Clone, Debug)]
pub struct RenderedNode<ID, Font>
where
    ID: Clone,
    Font: Clone,
{
    pub position: Vec2,
    pub node_elements: Vec<NodeElement<Font>>,
    pub node_links: Vec<ID>,
    pub dimensions: Vec2,
}

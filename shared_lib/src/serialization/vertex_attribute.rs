use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::opengl::vertex_layout_manager::VertexLayoutManager;

#[derive(Serialize, Deserialize)]
pub(crate) struct VertexAttribute {
    name: Option<String>,
    // Optional: might not be used
    components: i32,
    data_type: Option<String>,
    // Optional: data type might not always be necessary
    normalized: Option<bool>,
    // Optional: normalization might not be relevant for all data types
    stride: Option<i32>,
    // Optional: stride might be uniform and not needed to be specified each time
    offset: Option<i32>, // Optional: offset might not be needed if data is tightly packed
}

#[derive(Serialize, Deserialize)]
pub(crate) struct VertexLayout {
    attributes: Vec<VertexAttribute>,
}

pub fn write_vertex_layout<N: AsRef<str>>(
    _layout_manager: &VertexLayoutManager,
    _file_name: N,
) -> Result<()> {
    Ok(())
}

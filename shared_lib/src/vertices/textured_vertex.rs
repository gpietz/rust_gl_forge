use crate::color::Color;
use crate::gl_prelude::{Vertex,VertexAttributeType};
use crate::opengl::vertex_attribute::VertexAttribute;
use crate::vertices::{VertexColor, VertexTexCoords};

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex {
    pub position: [f32; 3],   // XYZ coordinates
    pub tex_coords: [f32; 2], // UV texture coordinates
    pub color: [f32; 4],      // color of the vertex
}

impl TexturedVertex {
    pub fn new_xyz_uv(x: f32, y: f32, z: f32, u: f32, v: f32) -> TexturedVertex {
        Self {
            position: [x, y, z],
            tex_coords: [u, v],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    fn is_similar(&self, other: &Self, tolerance: f32) -> bool {
        self.position
            .iter()
            .zip(other.position.iter())
            .all(|(a, b)| (a - b).abs() <= tolerance)
            && self
                .tex_coords
                .iter()
                .zip(other.tex_coords.iter())
                .all(|(a, b)| (a - b).abs() <= tolerance)
            && self
                .color
                .iter()
                .zip(other.color.iter())
                .all(|(a, b)| (a - b).abs() <= tolerance)
    }

    /// Deduplicates a slice of vertices based on a specified similarity tolerance
    /// and generates a vector of indices corresponding to the unique vertices.
    ///
    /// This method iterates over each vertex in the input slice. It compares each
    /// vertex against a list of already found unique vertices using a custom
    /// similarity function that accepts a tolerance value for comparison. If a
    /// vertex is deemed similar to an existing one, the index of that existing
    /// vertex is added to the indices list. If no similar vertex is found, the
    /// vertex is added to the list of unique vertices, and its new index is
    /// recorded.
    ///
    /// # Parameters
    /// - `vertices`: A slice of vertices (`&[Self]`) that will be checked for
    ///   duplicates. This slice is not modified.
    /// - `tolerance`: A floating-point number (`f32`) representing the maximum
    ///   allowable difference between vertices for them to be considered the same.
    ///   This is used in the `is_similar` method for comparing vertex properties.
    ///
    /// # Returns
    /// - `Option<(Vec<Self>, Vec<u32>)>`: Returns `Some((unique_vertices, indices))`
    ///   if the number of unique vertices is less than the total number of vertices
    ///   passed in, indicating that some vertices were deduplicated. Returns `None`
    ///   if all vertices are unique, meaning no deduplication occurred.
    ///
    /// # Example
    /// ```
    /// use shared_lib::vertices::textured_vertex::TexturedVertex;
    ///
    /// let vertices = vec![
    ///     TexturedVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0],
    ///       color: [1.0, 1.0, 1.0, 1.0] },
    ///     TexturedVertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0],
    ///       color: [1.0, 1.0, 1.0, 1.0] },
    ///     TexturedVertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0],
    ///       color: [0.5, 0.5, 0.5, 1.0] },
    /// ];
    /// let tolerance = 0.01;
    /// let result = TexturedVertex::dedupe_vertices(&vertices, tolerance);
    /// if let Some((unique, indices)) = result {
    ///     println!("Unique vertices: {:?}", unique);
    ///     println!("Indices: {:?}", indices);
    /// }
    /// ```
    ///
    /// This method is useful for reducing data redundancy before rendering
    /// operations where vertex data can be specified once and reused multiple times.
    pub fn dedupe_vertices(vertices: &[Self], tolerance: f32) -> Option<(Vec<Self>, Vec<u32>)> {
        let mut unique_vertices = Vec::new();
        let mut indices = Vec::new();

        'outer: for vertex in vertices {
            for (index, unique_vertex) in unique_vertices.iter().enumerate() {
                if vertex.is_similar(unique_vertex, tolerance) {
                    indices.push(index as u32);
                    continue 'outer;
                }
            }
            unique_vertices.push(*vertex);
            indices.push(unique_vertices.len() as u32 - 1);
        }

        if vertices.len() - indices.len() > 0 {
            Some((unique_vertices, indices))
        } else {
            None
        }
    }
}

impl Default for TexturedVertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Vertex for TexturedVertex {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttributeType::Position.into(),
            VertexAttributeType::TexCoord.into(),
            VertexAttributeType::Color.into(),
        ]
    }
}

impl VertexTexCoords for TexturedVertex {
    fn with_tex_coords(self, u: f32, v: f32) -> Self {
        Self {
            position: self.position,
            tex_coords: [u, v],
            color: self.color,
        }
    }

    fn set_tex_coords(&mut self, u: f32, v: f32) -> &mut Self {
        self.tex_coords = [u, v];
        self
    }
}

impl VertexColor for TexturedVertex {
    fn with_color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            position: self.position,
            tex_coords: self.tex_coords,
            color: [r, g, b, a],
        }
    }

    fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.color = [r, g, b, a];
        self
    }

    fn set_color_ref(&mut self, color: &Color) -> &mut Self {
        self.color = [color.r, color.g, color.b, color.a];
        self
    }

    fn set_color_array(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = [color[0], color[1], color[2], color[3]];
        self
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
        self
    }

    fn set_opacity(&mut self, opacity: f32) -> &mut Self {
        self.color[3] = opacity;
        self
    }
}

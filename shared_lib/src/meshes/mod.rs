pub mod basic_mesh;

pub trait DynamicVertex {
    fn as_bytes(&self) -> &[u8];
}

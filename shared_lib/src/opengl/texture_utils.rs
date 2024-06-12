use gl::types::GLuint;
use image::{DynamicImage, ImageBuffer};

pub(crate) fn get_texture_from_gpu(texture_id: GLuint, width: i32, height: i32) -> DynamicImage {
    let mut data = vec![0u8; (width * height * 4) as usize];
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::GetTexImage(
            gl::TEXTURE_2D,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_mut_ptr() as *mut _,
        );
    }
    DynamicImage::ImageRgba8(ImageBuffer::from_raw(width as u32, height as u32, data).unwrap())
}

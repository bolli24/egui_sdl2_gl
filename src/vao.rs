#![allow(unsafe_code)]

use crate::{check_for_gl_error, gl_helper::{create_vertex_array, vertex_attrib_pointer_f32}};
use gl::types::GLuint;

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct BufferInfo {
    pub location: u32, //
    pub vector_size: i32,
    pub data_type: u32, //GL_FLOAT,GL_UNSIGNED_BYTE
    pub normalized: bool,
    pub stride: i32,
    pub offset: i32,
}

// ----------------------------------------------------------------------------

/// Wrapper around either Emulated VAO or GL's VAO.
#[allow(dead_code)]
pub(crate) struct VertexArrayObject {
    gl: gl::Gl,
    vao: GLuint,
    vbo: GLuint,
    buffer_infos: Vec<BufferInfo>,
}

impl VertexArrayObject {
    #[allow(clippy::needless_pass_by_value)] // false positive
    pub(crate) unsafe fn new(gl: &gl::Gl, vbo: GLuint, buffer_infos: Vec<BufferInfo>) -> Self {
        let vao = create_vertex_array(&gl).unwrap();
        // check_for_gl_error!("create_vertex_array");

        // Store state in the VAO:
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        for attribute in &buffer_infos {
            vertex_attrib_pointer_f32(
                gl,
                attribute.location,
                attribute.vector_size,
                attribute.data_type,
                attribute.normalized,
                attribute.stride,
                attribute.offset,
            );
            // check_for_gl_error!("vertex_attrib_pointer_f32");
            gl.EnableVertexAttribArray(attribute.location);
            check_for_gl_error!(&gl, "enable_vertex_attrib_array");
        }

        gl.BindVertexArray(0);

        Self {
            gl: gl.clone(),
            vao,
            vbo,
            buffer_infos,
        }
    }

    pub(crate) unsafe fn bind(&self) {
        self.gl.BindVertexArray(self.vao);
        check_for_gl_error!(&self.gl, "bind_vertex_array");
    }

    pub(crate) unsafe fn unbind(&self) {
        self.gl.BindVertexArray(0);
    }
}

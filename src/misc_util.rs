#![allow(unsafe_code)]

use gl::types::GLuint;

use crate::gl_helper::{create_shader, shader_source, get_shader_compile_status, get_shader_info_log, create_program, attach_shader, get_program_link_status, get_program_info_log};

pub(crate) unsafe fn compile_shader(
    gl: &gl::Gl,
    shader_type: u32,
    source: &str,
) -> Result<GLuint, String> {
    let shader = create_shader(gl, shader_type)?;

    shader_source(gl, shader, source);

    gl.CompileShader(shader);

    if get_shader_compile_status(gl, shader) {
        Ok(shader)
    } else {
        Err(get_shader_info_log(gl, shader))
    }
}

pub(crate) unsafe fn link_program<'a, T: IntoIterator<Item = GLuint>>(
    gl: &gl::Gl,
    shaders: T,
) -> Result<GLuint, String> {
    let program = create_program(gl)?;

    for shader in shaders {
        attach_shader(gl, program, shader);
    }

    gl.LinkProgram(program);

    if get_program_link_status(gl, program) {
        Ok(program)
    } else {
        Err(get_program_info_log(gl, program))
    }
}

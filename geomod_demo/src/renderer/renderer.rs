use crate::renderer::buffer::Buffer;
use crate::renderer::program::ShaderProgram;
use crate::renderer::shader::{Shader, ShaderError};
use crate::renderer::texture::Texture;
use crate::renderer::vertex_array::VertexArray;
use crate::set_attribute;
use image::ImageError;

use std::fs;
use std::path::Path;
use std::ptr;
use thiserror::Error;

use gl::types::*;
use std::ffi::CString;

type Pos = [f32; 2];
type TextureCoords = [f32; 2];

struct Pt2(Pos);

#[repr(C, packed)]
struct Vertex(Pos, TextureCoords);

#[rustfmt::skip]
const VERTICES: [Vertex; 4] = [
    Vertex([-0.5, -0.5],  [0.0, 1.0]),
    Vertex([ 0.5, -0.5],  [1.0, 1.0]),
    Vertex([ 0.5,  0.5],  [1.0, 0.0]),
    Vertex([-0.5,  0.5],  [0.0, 0.0]),
];

#[rustfmt::skip]
const FULL: [Vertex; 6] = [
    Vertex([-1.0, -1.0],  [0.0, 1.0]),
    Vertex([1.0, -1.0],  [1.0, 1.0]),
    Vertex([-1.0, 1.0],  [1.0, 0.0]),
    Vertex([ 1.0, -1.0],  [1.0, 1.0]),
    Vertex([ 1.0,  1.0],  [1.0, 0.0]),
    Vertex([-1.0,  1.0],  [0.0, 0.0]),
];

#[derive(Debug, Error)]
pub enum RendererInitError {
    #[error{"{0}"}]
    ImageError(#[from] ImageError),
    #[error{"{0}"}]
    ShaderError(#[from] ShaderError),
}

pub struct RenderPassCrap {
    shader: ShaderProgram,
    vertex_buffer: Buffer,
    vertex_array: VertexArray
}

pub struct Renderer {
    basic_shader: RenderPassCrap,
    grid_shader: RenderPassCrap,
    tool_shader: RenderPassCrap,
    pub mousepos: [f32; 2],
    pub gridoffset: [f32; 2],
    pub gridscale: f32
}

impl Renderer {
    pub fn new() -> Result<Self, RendererInitError> {
        unsafe {
            //
            let grid_shader: RenderPassCrap = make_grid_shader();
            let basic_shader: RenderPassCrap = make_basic_shader();
            let tool_shader: RenderPassCrap = make_tool_shader();

            return Ok( Self {
                basic_shader,
                grid_shader,
                tool_shader,
                mousepos: [0.0, 0.0],
                gridoffset: [0.0, 0.0],
                gridscale: 1.0
            });
        }
        
        
    }

    pub fn draw(&self) {
        unsafe {
            
            self.grid_shader.shader.apply();
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
            self.grid_shader.vertex_array.bind();
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Uniform2f(gl::GetUniformLocation(self.grid_shader.shader.id, CString::new("gridOffset").unwrap().as_ptr()), self.gridoffset[0], self.gridoffset[1]);
            gl::Uniform1f(gl::GetUniformLocation(self.grid_shader.shader.id, CString::new("gridScale").unwrap().as_ptr()), self.gridscale);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            
            self.tool_shader.shader.apply();
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
            self.tool_shader.vertex_array.bind();
            gl::Uniform2f(gl::GetUniformLocation(self.tool_shader.shader.id, CString::new("gridOffset").unwrap().as_ptr()), self.gridoffset[0], self.gridoffset[1]);
            gl::Uniform1f(gl::GetUniformLocation(self.tool_shader.shader.id, CString::new("gridScale").unwrap().as_ptr()), self.gridscale);
            gl::Uniform2f(gl::GetUniformLocation(self.tool_shader.shader.id, CString::new("mousePos").unwrap().as_ptr()), self.mousepos[0], self.mousepos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            
            
            /*self.basic_shader.shader.apply();
            self.basic_shader.vertex_array.bind();
            gl::Uniform1f(gl::GetUniformLocation(self.basic_shader.shader.id, CString::new("offsetX").unwrap().as_ptr()), self.mousepos[0]);
            gl::Uniform1f(gl::GetUniformLocation(self.basic_shader.shader.id, CString::new("offsetY").unwrap().as_ptr()), self.mousepos[1]);
            gl::DrawArrays(gl::POINTS, 0, 4);*/
        }
    }

    
}

pub unsafe fn make_basic_shader() -> RenderPassCrap {
    let basic_vertex_source = fs::read_to_string("shaders/basic.vs").unwrap();
    let basic_fragment_source = fs::read_to_string("shaders/basic.fs").unwrap();
    let basic_geometry_source = fs::read_to_string("shaders/basic.gs").unwrap();
    let basic_geometry_shader = Shader::new(&basic_geometry_source[..], gl::GEOMETRY_SHADER).unwrap();
    let basic_vertex_shader = Shader::new(&basic_vertex_source[..], gl::VERTEX_SHADER).unwrap();
    let basic_fragment_shader = Shader::new(&basic_fragment_source[..], gl::FRAGMENT_SHADER).unwrap();
    let basic_shader = ShaderProgram::new(&[basic_geometry_shader, basic_vertex_shader, basic_fragment_shader]).unwrap();

    let basic_vertex_array = VertexArray::new();
    basic_vertex_array.bind();

    let basic_vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    basic_vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);

    let pos_attrib = basic_shader.get_attrib_location("position").unwrap();
    set_attribute!(basic_vertex_array, pos_attrib, Vertex::0);
    let color_attrib = basic_shader.get_attrib_location("vertexTexCoord").unwrap();
    set_attribute!(basic_vertex_array, color_attrib, Vertex::1);

    gl::Uniform1f(gl::GetUniformLocation(basic_shader.id, CString::new("offsetX").unwrap().as_ptr()), 0.25);
    gl::Uniform1f(gl::GetUniformLocation(basic_shader.id, CString::new("offsetY").unwrap().as_ptr()), 0.25);

    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable(gl::BLEND);

    return RenderPassCrap {
        shader: basic_shader,
        vertex_buffer: basic_vertex_buffer,
        vertex_array: basic_vertex_array
    }
}

pub unsafe fn make_grid_shader() -> RenderPassCrap {
    let vertex_source = fs::read_to_string("shaders/grid/grid.vs").unwrap();
    let fragment_source = fs::read_to_string("shaders/grid/grid.fs").unwrap();
    let vertex_shader = Shader::new(&vertex_source[..], gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::new(&fragment_source[..], gl::FRAGMENT_SHADER).unwrap();
    let shader = ShaderProgram::new(&[vertex_shader, fragment_shader]).unwrap();
    shader.apply();

    let vertex_array = VertexArray::new();
    vertex_array.bind();

    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.set_data(&FULL, gl::STATIC_DRAW);

    let pos_attrib = shader.get_attrib_location("position").unwrap();
    set_attribute!(vertex_array, pos_attrib, Vertex::0);
    let color_attrib = shader.get_attrib_location("vertexTexCoord").unwrap();
    set_attribute!(vertex_array, color_attrib, Vertex::1);

    //gl::Uniform2f(gl::GetUniformLocation(shader.id, CString::new("gridOffset").unwrap().as_ptr()), 0.0, 0.0);

    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable(gl::BLEND);

    return RenderPassCrap {
        shader: shader,
        vertex_buffer: vertex_buffer,
        vertex_array: vertex_array
    }
}

pub unsafe fn make_tool_shader() -> RenderPassCrap {
    let vertex_source = fs::read_to_string("shaders/tool/tool.vs").unwrap();
    let fragment_source = fs::read_to_string("shaders/tool/tool.fs").unwrap();
    let vertex_shader = Shader::new(&vertex_source[..], gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::new(&fragment_source[..], gl::FRAGMENT_SHADER).unwrap();
    let shader = ShaderProgram::new(&[vertex_shader, fragment_shader]).unwrap();
    shader.apply();

    let vertex_array = VertexArray::new();
    vertex_array.bind();

    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
    vertex_buffer.set_data(&FULL, gl::STATIC_DRAW);

    let pos_attrib = shader.get_attrib_location("position").unwrap();
    set_attribute!(vertex_array, pos_attrib, Vertex::0);
    let color_attrib = shader.get_attrib_location("vertexTexCoord").unwrap();
    set_attribute!(vertex_array, color_attrib, Vertex::1);

    //gl::Uniform2f(gl::GetUniformLocation(shader.id, CString::new("gridOffset").unwrap().as_ptr()), 0.0, 0.0);

    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable(gl::BLEND);

    return RenderPassCrap {
        shader: shader,
        vertex_buffer: vertex_buffer,
        vertex_array: vertex_array
    }
}

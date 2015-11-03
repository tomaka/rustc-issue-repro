use std::cell::{Cell, RefCell, RefMut};
use std::collections::hash_map;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

use cgmath::Matrix;
use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Vector4;

use glium;
use glium::Surface;
use glium::index::PrimitiveType;
use glium::texture::SrgbTexture2dArray;

use image;
use resources;
use resources::ResourceId;
use text_render_atlas::Font;

pub const MAX_TRIANGLES_PER_BATCH: u16 = 1024;

pub const NUM_SUBBUFFERS: u16 = 4;

pub struct BatchSystem {
    context: glium::Display,
    vertex_buffer: RefCell<glium::VertexBuffer<SpriteVertex>>,
    next_subbufer: Cell<u16>,
    program: glium::Program,
    sprites_arrays: RefCell<Vec<(glium::texture::SrgbTexture2dArray, Vec<Option<ResourceId>>)>>,

    sprites_dimensions: RefCell<HashMap<ResourceId, (u32, u32)>>,
    font_atlas: RefCell<HashMap<ResourceId, (Font, usize, usize)>>,
}

impl BatchSystem {
    pub fn new(context: &glium::Display) -> BatchSystem {
        let vb_len = NUM_SUBBUFFERS as usize * MAX_TRIANGLES_PER_BATCH as usize * 3;
        let vb = glium::VertexBuffer::empty_persistent(context, vb_len).unwrap();

        BatchSystem {
            context: context.clone(),
            vertex_buffer: RefCell::new(vb),
            next_subbufer: Cell::new(0),
            program: program!(context,
                140 => {
                    vertex: r"",
                    fragment: r""
                }
            ).unwrap(),
            sprites_arrays: RefCell::new(Vec::new()),
            sprites_dimensions: RefCell::new(HashMap::new()),
            font_atlas: RefCell::new(HashMap::new()),
        }
    }

    /// Starts a batch in order to draw stuff.
    #[inline]
    pub fn batch(&self) -> Batch {
        Batch {
            system: self,
            triangles: Vec::new(),
        }
    }

    /// Ensures that a texture has been loaded.
    #[inline]
    pub fn load_texture(&self, texture: &ResourceId) {
        self.get_texture(texture);
    }

    /// Ensures that a font has been loaded.
    pub fn load_font(&self, font: &ResourceId) {
    }

    /// Returns the height/width ratio of a texture's dimensions.
    #[inline]
    pub fn get_texture_height_per_width(&self, texture: &ResourceId) -> f32 {
        0.0
    }

    /// Returns the index of the texture atlas and index in the texture atlas of the texture.
    fn get_texture(&self, resource_name: &ResourceId) -> (usize, usize) {
        (0, 0)
    }

    /// Add a texture to `sprites_arrays` and returns the position in the double arrays system.
    fn add_texture_to_atlas(&self, texture: &glium::texture::SrgbTexture2d,
                            resource_name: &ResourceId) -> (usize, usize)
    {
        (0, 0)
    }
}

/// Objects that allows one to draw stuff.
pub struct Batch<'a> {
    system: &'a BatchSystem,
    triangles: Vec<Vec<SpriteVertex>>,
}

impl<'a> Batch<'a> {
    pub fn draw<S>(&mut self, target: &mut S, blend_color: [f32; 3], depth_test: bool)
        where S: Surface
    {
    }
}

/// The structure used to hold each vertex.
#[derive(Debug, Copy, Clone)]
struct SpriteVertex {
    position: [f32; 3],
    color: [f32; 3],
    tex_coords: [f32; 2],
    tex_index: u32,
    is_text: f32,
}

implement_vertex!(SpriteVertex, position, color, tex_coords, tex_index, is_text);

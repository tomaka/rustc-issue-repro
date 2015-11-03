use glium;

use resources;
use resources::ResourceId;

pub struct ModelsSystem {
    program: glium::Program,
    tree_model: glium::VertexBuffer<Vertex>,
}

impl ModelsSystem {
    pub fn new(display: &glium::Display) -> ModelsSystem {
        let program = program!(display,
                140 => {
                    vertex: "",
                    fragment: ""
                }
            ).unwrap();

        ModelsSystem {
            program: program,
            tree_model: glium::VertexBuffer::immutable(display, &Vec::<Vertex>::new()).unwrap(),
        }
    }

    pub fn draw_trees(&self) {

    }
}

#[derive(Debug, Copy, Clone)]
struct Vertex {
    i_position: [f32; 3],
    i_normal: [f32; 3],
    i_color: [f32; 3],
}

implement_vertex!(Vertex, i_position);

#[derive(Debug, Copy, Clone)]
struct PerInstance {
    i_instance_matrix: [[f32; 4]; 4],
}

implement_vertex!(PerInstance, i_instance_matrix);

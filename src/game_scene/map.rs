use glium;
use image;

use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Matrix4;

use std::f32::consts::PI;

#[inline]
pub fn lat_long_to_world(pos: &Vector2<f32>, distance: f32) -> Vector3<f32> {
    Vector3::new(pos.x.cos() * pos.y.cos() * distance, pos.y.sin() * distance,
                 -pos.x.sin() * pos.y.cos() * distance)
}

pub struct MapDrawSystem {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    elevation_map: glium::Texture2d,
    climate_map: glium::Texture2d,
}

impl MapDrawSystem {
    pub fn new(display: &glium::Display) -> MapDrawSystem {
        let (vertex_buffer, index_buffer) = generate_sphere(display);

        let elevation_map = glium::texture::Texture2d::empty(display, 1024, 1024).unwrap();
        let climate_map = glium::texture::Texture2d::empty(display, 1024, 1024).unwrap();

        let program = program!(display,
            140 => {
                vertex: "",
                fragment: ""
            },
        ).unwrap();

        MapDrawSystem {
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            elevation_map: elevation_map,
            climate_map: climate_map,
        }
    }

    pub fn draw<T>(&self, target: &mut T, proj: &Matrix4<f32>, view: &Matrix4<f32>,
                   climate_map: &glium::Texture2d, ground_colors: &glium::Texture2d)
        where T: glium::Surface
    {
        let uniforms = uniform! {
            u_view_to_screen: proj.clone(),
            u_world_to_view: view.clone(),
            u_view_light_direction: [0.0, 0.0, 1.0f32],
            u_elevation: &self.elevation_map,
            u_climate: self.climate_map.sampled().minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                                                 .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            u_climate_map: climate_map,
            u_ground_color: ground_colors,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program,
                    &uniforms, &params)
              .unwrap();
    }
}

pub struct ElementsInfos {
    pub ground_opacity: f32,
    pub owner_color_opacity: f32,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    map_coords: [f32; 2],
}

implement_vertex!(Vertex, position, map_coords);

fn generate_sphere(display: &glium::Display)
                   -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>)
{
    let num_longitude = 64;
    let num_latitude = 64;

    let mut vertex_data = Vec::with_capacity((num_longitude + 1) * num_latitude);
    let mut index_data = Vec::with_capacity((num_longitude + 1) * (num_latitude - 1) * 6);

    let vertex_buffer = glium::VertexBuffer::new(display, &vertex_data).unwrap();
    let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_data).unwrap();
    (vertex_buffer, index_buffer)
}

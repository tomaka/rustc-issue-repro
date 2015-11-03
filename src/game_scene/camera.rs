use cgmath::{self, EuclideanVector, Vector, Vector2, Vector3, Matrix4};
use game_scene::map;

use std::f32::consts::PI;

pub struct CameraState {
    distance: f32,
    current_longitude: f32,
    current_latitude: f32,
    aspect: f32,
}

impl CameraState {
    pub fn new(dimensions: (u32, u32)) -> CameraState {
        CameraState {
            distance: 1.5,
            current_longitude: 0.0,
            current_latitude: 0.0,
            aspect: dimensions.0 as f32 / dimensions.1 as f32,
        }
    }

    pub fn set_viewport(&mut self, dimensions: (u32, u32)) {
    }

    pub fn get_perspective(&self) -> Matrix4<f32> {
        cgmath::perspective(cgmath::deg(45.0), self.aspect, 0.01, 6.0)
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        let camera_pos = map::lat_long_to_world(&Vector2::new(self.current_longitude, self.current_latitude), self.distance + 1.05);
        let up = map::lat_long_to_world(&Vector2::new(self.current_longitude, self.current_latitude + 0.01), self.distance + 1.05);
        view_matrix(&camera_pos, &Vector3::new(0.0, 0.0, 0.0), &(up - camera_pos).normalize())
    }
}

fn view_matrix(camera: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
    let f = target.sub_v(camera);
    let f = f.normalize();

    let up = up.normalize();

    let s = f.cross(&up);
    let u = s.normalize().cross(&f);

    let view = Matrix4::new(
        s.x, u.x, -f.x, 0.0,
        s.y, u.y, -f.y, 0.0,
        s.z, u.z, -f.z, 0.0,
        0.0, 0.0,  0.0, 1.0
    );

    let position = Matrix4::new(
            1.0,       0.0,       0.0,   0.0,
            0.0,       1.0,       0.0,   0.0,
            0.0,       0.0,       1.0,   0.0,
        -camera.x, -camera.y, -camera.z, 1.0,
    );

    view * position
}

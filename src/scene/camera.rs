use {
    crate::math::transform::Transform,
    glam::{EulerRot, Mat4, Quat, Vec3},
    winit::keyboard::KeyCode,
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub aspect_ratio: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub move_speed: f32,
    pub turn_speed: f32,
    forward_pressed: bool,
    backward_pressed: bool,
    right_pressed: bool,
    left_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    turn_right_pressed: bool,
    turn_left_pressed: bool,
    turn_up_pressed: bool,
    turn_down_pressed: bool,
    pitch: f32,
    yaw: f32,
}

impl Camera {
    pub fn new(
        transform: Transform,
        aspect_ratio: f32,
        fov_y: f32,
        z_near: f32,
        z_far: f32,
        move_speed: f32,
        turn_speed: f32,
    ) -> Self {
        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        Self {
            transform,
            aspect_ratio,
            fov_y,
            z_near,
            z_far,
            move_speed,
            turn_speed,
            yaw,
            pitch,
            forward_pressed: false,
            backward_pressed: false,
            right_pressed: false,
            left_pressed: false,
            up_pressed: false,
            down_pressed: false,
            turn_right_pressed: false,
            turn_left_pressed: false,
            turn_up_pressed: false,
            turn_down_pressed: false,
        }
    }
    pub fn build_view_matrix(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.transform.rotation, self.transform.position).inverse()
    }

    pub fn build_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far)
    }

    pub fn update(&mut self, dt: f32) {
        let move_forward = (self.backward_pressed as i32 - self.forward_pressed as i32) as f32;
        let move_right = (self.right_pressed as i32 - self.left_pressed as i32) as f32;
        let move_up = (self.up_pressed as i32 - self.down_pressed as i32) as f32;

        let turn_yaw = (self.turn_right_pressed as i32 - self.turn_left_pressed as i32) as f32;
        let turn_pitch = (self.turn_up_pressed as i32 - self.turn_down_pressed as i32) as f32;

        let movement_dir = Vec3::new(move_right, move_up, move_forward);
        self.transform.position += self.transform.rotation * movement_dir * self.move_speed * dt;

        self.yaw -= turn_yaw * self.turn_speed * dt;
        self.pitch = (self.pitch + turn_pitch * self.turn_speed * dt)
            .clamp(-90_f32.to_radians() + 0.001, 90_f32.to_radians() - 0.001);

        self.transform.rotation =
            Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);
    }

    pub fn handle_keyboard_input(&mut self, code: KeyCode, is_pressed: bool) {
        match code {
            KeyCode::KeyW => self.forward_pressed = is_pressed,
            KeyCode::KeyS => self.backward_pressed = is_pressed,
            KeyCode::KeyD => self.right_pressed = is_pressed,
            KeyCode::KeyA => self.left_pressed = is_pressed,
            KeyCode::KeyE => self.up_pressed = is_pressed,
            KeyCode::KeyQ => self.down_pressed = is_pressed,
            KeyCode::ArrowRight => self.turn_right_pressed = is_pressed,
            KeyCode::ArrowLeft => self.turn_left_pressed = is_pressed,
            KeyCode::ArrowUp => self.turn_up_pressed = is_pressed,
            KeyCode::ArrowDown => self.turn_down_pressed = is_pressed,
            _ => {}
        }
    }
}

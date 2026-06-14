use {
    crate::math::transform::Transform,
    glam::{Mat4, Quat, Vec3},
    winit::keyboard::KeyCode,
};

#[derive(Default, Debug, Clone, Copy)]
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
    yaw: f32
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
        let forward = transform.rotation * Vec3::NEG_Z;

        let yaw = forward.x.atan2(-forward.z);

        let horizontal_length = (forward.x * forward.x + forward.z * forward.z).sqrt();
        let pitch = forward.y.atan2(horizontal_length);

        Self {
            transform,
            aspect_ratio,
            fov_y,
            z_near,
            z_far,
            move_speed,
            turn_speed,
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
            yaw,
            pitch
        }
    }
    pub fn build_view_matrix(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.transform.rotation, self.transform.position).inverse()
    }

    pub fn build_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far)
    }

    pub fn update(&mut self, dt: f32) {
        let mut forward_vel = 0.0;
        if self.forward_pressed {
            forward_vel -= self.move_speed;
        }
        if self.backward_pressed {
            forward_vel += self.move_speed;
        }

        let mut right_vel = 0.0;
        if self.right_pressed {
            right_vel += self.move_speed;
        }
        if self.left_pressed {
            right_vel -= self.move_speed;
        }

        let mut up_vel = 0.0;
        if self.up_pressed {
            up_vel += self.move_speed;
        }
        if self.down_pressed {
            up_vel -= self.move_speed;
        }

        let mut turn_right_vel = 0.0;
        if self.turn_right_pressed {
            turn_right_vel += self.turn_speed;
        }
        if self.turn_left_pressed {
            turn_right_vel -= self.turn_speed;
        }

        let mut turn_up_vel = 0.0;
        if self.turn_up_pressed {
            turn_up_vel += self.turn_speed;
        }
        if self.turn_down_pressed {
            turn_up_vel -= self.turn_speed;
        }

        self.transform.position += forward_vel * (self.transform.rotation * Vec3::Z) * dt;
        self.transform.position += right_vel * (self.transform.rotation * Vec3::X) * dt;
        self.transform.position += up_vel * (self.transform.rotation * Vec3::Y) * dt;

        self.yaw += -turn_right_vel * dt;
        self.pitch += turn_up_vel * dt;

        let limit = 90_f32.to_radians() - 0.001; 
        self.pitch = self.pitch.clamp(-limit, limit);

        let yaw_quat = Quat::from_rotation_y(self.yaw);
        let pitch_quat = Quat::from_rotation_x(self.pitch);

        self.transform.rotation = yaw_quat * pitch_quat;
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

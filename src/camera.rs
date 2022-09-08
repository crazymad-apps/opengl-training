pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3],

    pub matrix: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> Camera {
        let matrix = view_matrix(&position, &direction, &up);

        Camera {
            position: position,
            direction: direction,
            up: up,
            matrix: matrix,
        }
    }

    pub fn update_matrix(&mut self) {
        self.matrix = view_matrix(&self.position, &self.direction, &self.up);
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn set_direction(&mut self, direction: [f32; 3]) {
        self.direction = direction;
    }

    pub fn set_up(&mut self, up: [f32; 3]) {
        self.up = up;
    }

    pub fn move_forward(&mut self, step: f32) {
        self.position[2] += step;
    }

    pub fn move_backward(&mut self, step: f32) {
        self.position[2] -= step;
    }

    pub fn move_left(&mut self, step: f32) {
        self.position[0] -= step;
    }

    pub fn move_right(&mut self, step: f32) {
        self.position[0] += step;
    }
}

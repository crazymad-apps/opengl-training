#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

pub fn cube_mesh(center: &[f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let a1 = Vertex {
        position: (-0.5, 0.5, -0.5f32),
    };
    let b1 = Vertex {
        position: (0.5, 0.5, -0.5f32),
    };
    let c1 = Vertex {
        position: (0.5, -0.5, -0.5f32),
    };
    let d1 = Vertex {
        position: (-0.5, -0.5, -0.5f32),
    };

    let a2 = Vertex {
        position: (-0.5, 0.5, 0.5f32),
    };
    let b2 = Vertex {
        position: (0.5, 0.5, 0.5f32),
    };
    let c2 = Vertex {
        position: (0.5, -0.5, 0.5f32),
    };
    let d2 = Vertex {
        position: (-0.5, -0.5, 0.5f32),
    };

    let indices = vec![
        0, 1, 2, 2, 3, 0, // 正面
        0, 3, 4, 4, 3, 7, // 左面
        7, 6, 4, 4, 6, 5, // 背面
        5, 1, 0, 0, 4, 5, // 上面
        5, 6, 1, 1, 6, 2, // 右面
        2, 6, 3, 3, 6, 7u16, // 下面
    ];

    let vertices = vec![a1, b1, c1, d1, a2, b2, c2, d2];

    return (vertices, indices);
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

pub struct IcoSphere {
    vertices: Vec<Vertex>,
}

// Icosahedron sphere with subdivisions implemented
impl IcoSphere {
    pub fn new(radius: f32) -> IcoSphere {
        pass;
    }

    fn initial_icosahedron(radius: f32) -> IcoSphere {
        vertices: Vec<Vertex> = Vec<Vertex>::new();
        vertices.push(Vertex { vec![0.0, 1.0, 0.0]})

    }
}

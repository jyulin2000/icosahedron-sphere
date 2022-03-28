extern crate libm;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position: [
            x,
            y,
            z,
        ]}
    }
    pub fn magnitude(&self) -> f32 {
        let p = self.position;
        (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt()
    }

    pub fn to_unit(&self) -> Vertex {
        let m = self.magnitude();
        (1.0 / m) * *self
    }
}

impl ops::Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vertex) -> Vertex {
        Vertex {
            position: [
                self.position[0] + rhs.position[0],
                self.position[1] + rhs.position[1],
                self.position[2] + rhs.position[2],
            ]
        }
    }
}

impl ops::Mul<Vertex> for f32 {
    type Output = Vertex;

    fn mul(self, rhs: Vertex) -> Vertex {
        Vertex {
            position: [
                self * rhs.position[0],
                self * rhs.position[1],
                self * rhs.position[2],
            ]
        }
    }
}

implement_vertex!(Vertex, position);

pub struct IcoSphere {
    radius: f32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}

// Icosahedron sphere with subdivisions implemented
impl IcoSphere {
    pub fn new(radius: f32, n_subdivisions: u32) -> IcoSphere {
        let mut initial_sphere = IcoSphere::initial_icosahedron(radius);
        for _ in 0..n_subdivisions {
            initial_sphere = initial_sphere.subdivide();
        }

        initial_sphere
    }

    fn gen_initial_vertices(radius: f32) -> Vec<Vertex> {
        use libm::atanf;
        use std::f32::consts::PI;
        let h_angle: f32 = 72.0 * PI / 180.0;
        let v_angle: f32 = atanf(1.0 / 2.0);
        let mut vertices: Vec<Vertex> = Vec::new();
        vertices.push(Vertex { position: [0.0, radius, 0.0] } );
        
        let angle_offset = -126.0 * PI / 180.0;
        for n in 0..5 {
            let vertex = Vertex { position: [
                radius * v_angle.cos() * (angle_offset + h_angle * (n as f32)).cos(),
                radius * v_angle.sin(),
                radius * v_angle.cos() * (angle_offset + h_angle * (n as f32)).sin()
            ]};

            vertices.push(vertex);
        }
        for n in 0..5 {
            let vertex = Vertex { position: [
                radius * v_angle.cos() * (angle_offset + h_angle / 2.0 + h_angle * (n as f32)).cos(),
                radius * -v_angle.sin(),
                radius * v_angle.cos() * (angle_offset + h_angle / 2.0 + h_angle * (n as f32)).sin()
            ]};

            vertices.push(vertex);
        }

        vertices.push(Vertex { position: [0.0, -radius, 0.0] } );
        
        vertices
    }
    
    fn initial_icosahedron(radius: f32) -> IcoSphere {
        let vertices_init = IcoSphere::gen_initial_vertices(radius);
        
        let mut vertices: Vec<Vertex> = Vec::new();

        for i in 1..=5 {
            let mut add_index_triple = |index_triple: Vec<u32>| {
                let vertex_triple: Vec<Vertex> = index_triple
                    .iter()
                    .map(|index| { vertices_init[*index as usize] })
                    .collect();

                vertices.extend(vertex_triple);
            };

            // Top layer
            add_index_triple(vec![0, i, (i % 5) + 1]);
            
            // Second Layer
            add_index_triple(vec![i, (i+3) % 5 + 6, i+5]);

            // Third Layer
            add_index_triple(vec![i+5, i, (i+5) % 5 + 1]);

            // Bottom Layer
            add_index_triple(vec![11, i+5, (i % 5) + 6])
        }

        let indices: Vec<u32> = IcoSphere::gen_indices(vertices.len());

        IcoSphere{ radius: radius, vertices: vertices, indices: indices }
    }

    fn gen_indices(n: usize) -> Vec<u32> {
        ( 0..n )
            .map(|x| { x as u32 })
            .collect()
    }

    fn subdivide(&self) -> IcoSphere {
        let vertices_old = &self.vertices;
        let mut vertices = Vec::new();
        for i in 0..(vertices_old.len() / 3) {
            let v1 = vertices_old[i*3];
            let v2 = vertices_old[i*3 + 1];
            let v3 = vertices_old[i*3 + 2];

            let m1 = self.get_extruded_midpoint(v1, v2);
            let m2 = self.get_extruded_midpoint(v2, v3);
            let m3 = self.get_extruded_midpoint(v3, v1);

            vertices.extend([m1, m2, v2]);
            vertices.extend([m2, m3, v3]);
            vertices.extend([m3, m1, v1]);
            vertices.extend([m1, m2, m3]);
        }

        let indices = IcoSphere::gen_indices(vertices.len());
        IcoSphere { radius: self.radius, vertices: vertices, indices: indices }
    }

    fn get_extruded_midpoint(&self, p1: Vertex, p2: Vertex) -> Vertex {
        self.radius * (p1 + p2).to_unit()
    }
}

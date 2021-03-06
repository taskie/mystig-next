use glium;
use std::f32;

pub trait HasXY {
    fn from_xy(x: f32, y: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn xy(&self) -> [f32; 2] {
        [self.x(), self.y()]
    }
}

trait HasXYMut: HasXY {
    fn x_mut(&mut self) -> &mut f32;
    fn y_mut(&mut self) -> &mut f32;
}

#[derive(Copy, Clone, Debug)]
pub struct ColoredVertex2D {
    pub position: [f32; 2],
    pub vert_color: [f32; 4],
}

implement_vertex!(ColoredVertex2D, position, vert_color);

#[derive(Copy, Clone, Debug)]
pub struct Vertex2D {
    pub position: [f32; 2],
}

impl Vertex2D {
    pub fn new(position: [f32; 2]) -> Vertex2D {
        Vertex2D { position }
    }
}

impl HasXY for Vertex2D {
    fn from_xy(x: f32, y: f32) -> Vertex2D {
        Vertex2D { position: [x, y] }
    }
    fn x(&self) -> f32 {
        self.position[0]
    }
    fn y(&self) -> f32 {
        self.position[1]
    }
}

impl HasXYMut for Vertex2D {
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.position[0]
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.position[1]
    }
}

implement_vertex!(Vertex2D, position);

// Shape

pub fn rect_line<T: HasXY>(vertex: T, width: f32, height: f32) -> [T; 5] {
    let lt = T::from_xy(vertex.x(), vertex.y());
    let rt = T::from_xy(vertex.x() + width, vertex.y());
    let lb = T::from_xy(vertex.x(), vertex.y() + height);
    let rb = T::from_xy(vertex.x() + width, vertex.y() + height);
    let lt2 = T::from_xy(vertex.x(), vertex.y());
    [lt, rt, rb, lb, lt2]
}

pub fn rect_fill<T: HasXY>(vertex: T, width: f32, height: f32) -> [T; 4] {
    let lt = T::from_xy(vertex.x(), vertex.y());
    let rt = T::from_xy(vertex.x() + width, vertex.y());
    let lb = T::from_xy(vertex.x(), vertex.y() + height);
    let rb = T::from_xy(vertex.x() + width, vertex.y() + height);
    [lt, rt, lb, rb]
}

pub fn regular_polygon_line(center: Vertex2D, radius: f32, n: usize) -> Vec<Vertex2D> {
    let mut vs = Vec::with_capacity(n);
    for i in 0..(n + 1) {
        let t = f32::consts::PI * 2.0 * (i as f32) / (n as f32);
        vs.push(Vertex2D::from_xy(
            center.x() + radius * f32::cos(t),
            center.y() + radius * f32::sin(t),
        ));
    }
    vs
}

pub fn regular_polygon_fill(center: Vertex2D, radius: f32, n: usize) -> Vec<Vertex2D> {
    let mut vs = Vec::with_capacity(n);
    for i in 0..n {
        let k = if i % 2 == 0 {
            -((i / 2) as i32)
        } else {
            ((i + 1) / 2) as i32
        };
        let t = f32::consts::PI * 2.0 * (k as f32) / (n as f32);
        vs.push(Vertex2D::from_xy(
            center.x() + radius * f32::cos(t),
            center.y() + radius * f32::sin(t),
        ));
    }
    vs
}

pub fn circle_n(radius: f32) -> usize {
    let n = 2.0 * f32::consts::PI / f32::asin(1.0 / (f32::sqrt(radius + 1.0)));
    n as usize
}

pub fn circle_line(center: Vertex2D, radius: f32) -> Vec<Vertex2D> {
    regular_polygon_line(center, radius, circle_n(radius))
}

pub fn circle_fill(center: Vertex2D, radius: f32) -> Vec<Vertex2D> {
    regular_polygon_fill(center, radius, circle_n(radius))
}

struct Path2DBuilder<T: Clone + HasXY> {
    vertices: Vec<T>,
}

impl<T> Path2DBuilder<T>
where
    T: Clone + HasXY,
{
    fn new() -> Path2DBuilder<T> {
        Path2DBuilder { vertices: Vec::new() }
    }

    fn add_xy(&mut self, x: f32, y: f32) {
        self.vertices.push(T::from_xy(x, y))
    }

    fn build(&self) -> Vec<T> {
        self.vertices.clone()
    }
}

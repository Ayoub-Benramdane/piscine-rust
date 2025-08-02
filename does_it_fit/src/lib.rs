pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    let mut area;
    match kind {
        areas_volumes::GeometricalShapes::Square => {
            area = areas_volumes::square_area(a) as f64;
            area *= times as f64
        }
        areas_volumes::GeometricalShapes::Circle => {
            area = areas_volumes::circle_area(a);
            area *= times as f64
        }
        areas_volumes::GeometricalShapes::Rectangle => {
            area = areas_volumes::rectangle_area(a, b) as f64;
            area *= times as f64
        }
        areas_volumes::GeometricalShapes::Triangle => {
            area = areas_volumes::triangle_area(a, b);
            area *= times as f64
        }
    }
    ((x * y) as f64) >= area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let mut area;
    match kind {
        areas_volumes::GeometricalVolumes::Cube => {
            area = areas_volumes::cube_volume(a) as f64;
            area *= times as f64
        }
        areas_volumes::GeometricalVolumes::Sphere => {
            area = areas_volumes::sphere_volume(a);
            area *= times as f64
        }
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            area = areas_volumes::triangular_pyramid_volume(a as f64, b);
            area *= times as f64
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            area = areas_volumes::parallelepiped_volume(a, b, c) as f64;
            area *= times as f64
        }
        areas_volumes::GeometricalVolumes::Cone => {
            area = areas_volumes::cone_volume(a, b);
            area *= times as f64
        }
    }
    ((x * y * z) as f64) >= area
}

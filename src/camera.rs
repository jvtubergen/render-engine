use eframe::epaint::{Pos2, Vec2};
use nalgebra::{Matrix4, Vector3, Transform3, Point3, UnitQuaternion, Translation3, Isometry3, Unit};

// pub type Point = OPoint<f32, Const<3>>;
pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;
pub type Transform = Transform3<f32>;
pub type Translation = Translation3<f32>;
pub type Matrix = Matrix4<f32>;
pub type Isometry = Isometry3<f32>;


#[derive(Clone)]
pub struct Camera {
    pub pos: Point,
    pub dir: Vector,
    /// Field of view in radians. (Default is `f32::consts::FRAC_PI_4`).
    pub fov: f32, 
    pub screen: Vec2,
    pub w_c: Isometry
}
impl Camera {

    pub fn new(
        pos: Point,
        dir: Vector,
        fov: f32,
        screen: Vec2
    ) -> Self {
        Self {
            pos, dir, fov, screen,
            w_c: Camera::compute_world_isometry(&dir, pos)
        }
    }

    /// Camera ray -> global ray.
    pub fn compute_world_isometry(dir: &Vector, pos: Point) -> Isometry {
        // Use position and direction of camera to derive rotation matrix.
        let rot = UnitQuaternion::face_towards(dir, &Vector::z());
        // Construct isometry from camera position and its rotation.
        Isometry::from_parts(pos.into(), rot)
    }

}
use eframe::epaint::{Pos2, Vec2};
use nalgebra::{Matrix4, Vector3, Transform3, Point3, UnitQuaternion, Translation3, Isometry3, Unit};

// pub type Point = OPoint<f32, Const<3>>;
pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;
pub type Transform = Transform3<f32>;
pub type Translation = Translation3<f32>;
pub type Matrix = Matrix4<f32>;
pub type Isometry = Isometry3<f32>;


pub struct Camera {
    pub pos: Point,
    pub dir: Vector,
    /// Field of view in radians. (Default is `f32::consts::FRAC_PI_4`).
    pub fov: f32, 
    pub screen: Vec2,
    w_c: Isometry
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

    /// Pixel location -> direction vector in camera frame.
    fn t_c(&self, x: f32, y: f32) -> Isometry {
        
        // Construct the quaternions for rotation.
        let dir_x = Vector::x_axis();
        // Note: Vertical change in pixel position means rotation around x-axis.
        let angle_x = self.fov * (y - 0.5 * self.screen.y) / self.screen.y;

        let dir_y = Vector::y_axis();
        // Note: Horizontal change in pixel position means rotation around y-axis.
        let angle_y = self.fov * (x - 0.5 * self.screen.x) / self.screen.x; 

        // Combine rotations around x and y axis.
        let rot_x = UnitQuaternion::from_axis_angle(&dir_x, angle_x);
        let rot_y = UnitQuaternion::from_axis_angle(&dir_y, angle_y);
        let rot = rot_y * rot_x;

        // Construct isometry.
        Isometry::from_parts(Vector::zeros().into(), rot)

    }
    
    /// Convert pixel location into a world ray vector.
    pub fn ray(&self, x: f32, y: f32) -> Unit<Vector> {

        let t_c = self.t_c(x, y);

        // First move (x,y) position into camera ray.
        let camera_ray = t_c * Vector::z_axis();
        // Then to world ray.
        let world_ray = self.w_c * camera_ray;

        world_ray
    }


}
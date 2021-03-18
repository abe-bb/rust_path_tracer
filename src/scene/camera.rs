use crate::common::{Ray, Spacial, Vec3, VertexFormat};

pub struct Camera<T: VertexFormat> {
    look_from: Vec3<T>,
    look_at: Vec3<T>,
    up: Vec3<T>,
    aspect_ratio: T,
    horizontal_fov: T,
    vertical_fov: T,
    view_min: Vec3<T>,
    view_max: Vec3<T>,
    x_res: T,
    y_res: T,
    width: u32,
    height: u32,
    u: Vec3<T>,
    v: Vec3<T>,
    w: Vec3<T>,
}

impl<T: VertexFormat> Spacial<T> for Camera<T> {
    fn location(&self) -> &Vec3<T> {
        &self.look_from
    }
}

impl<T: VertexFormat> Camera<T> {
    pub fn new(
        look_at: Vec3<T>,
        look_from: Vec3<T>,
        up: Vec3<T>,
        width: u32,
        height: u32,
        horizontal_fov: T,
    ) -> Camera<T> {
        let x_res = T::from(width).unwrap();
        let y_res = T::from(height).unwrap();
        let aspect_ratio = x_res / y_res;

        let vertical_fov = T::from(2.0).unwrap()
            * ((horizontal_fov / T::from(2.0).unwrap()).tan() * (y_res / x_res)).atan();

        let horizontal_distance: T =
            (horizontal_fov / T::from(2.0).unwrap()).tan() * T::from(2.0).unwrap().sqrt();
        let vertical_distance =
            (vertical_fov / T::from(2.0).unwrap()).tan() * T::from(2.0).unwrap().sqrt();

        let vpn = look_from.sub(&look_at).normalize();

        let u = up.cross(&vpn).normalize();
        let v = vpn.cross(&u).normalize();

        Camera {
            look_at,
            look_from,
            up,
            aspect_ratio,
            horizontal_fov,
            vertical_fov,
            view_min: Vec3::new(-horizontal_distance, -vertical_distance, T::zero()),
            view_max: Vec3::new(horizontal_distance, vertical_distance, T::zero()),
            x_res,
            y_res,
            width,
            height,
            w: vpn,
            u,
            v,
        }
    }

    pub fn ray(&self, i: T, j: T) -> Ray<T> {
        let u =
            (i - T::zero()) * ((self.view_max.x - self.view_min.x) / self.x_res) + self.view_min.x;
        let v =
            (j - T::zero()) * ((self.view_max.y - self.view_min.y) / self.y_res) + self.view_min.y;
        let w = T::zero();

        // location of pixel (or sub pixel) in world space
        let pixel_loc = self
            .look_at
            .add(&self.u.mul(u))
            .add(&self.v.mul(v))
            .add(&self.w.mul(w));

        let origin = self.look_from.clone();

        let direction = pixel_loc.sub(&origin).normalize();

        Ray::new(origin, direction)
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn x_res(&self) -> &T {
        &self.x_res
    }
    pub fn y_res(&self) -> &T {
        &self.y_res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite_corner_ray_creation() {
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let look_from = Vec3::new(0.0, 0.0, 10.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let x = 1920;
        let y = 1080;
        let h_fov = 1.22173;

        let camera = Camera::new(look_at, look_from, up, x, y, h_fov);

        let ray1 = camera.ray(0.0, 0.0);
        let ray2 = camera.ray(1920.0, 1080.0);

        assert_eq!(ray1.direction.x, -ray2.direction.x);
        assert_eq!(ray1.direction.y, -ray2.direction.y);
        assert_eq!(ray1.direction.z, ray2.direction.z);
    }
}

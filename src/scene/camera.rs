use crate::common::{Vec3, VertexComp};

pub struct Camera<T: VertexComp> {
    look_from: Vec3<T>,
    look_at: Vec3<T>,
    up: Vec3<T>,
    aspect_ratio: T,
    horizontal_fov: T,
    vertical_fov: T,
    view_min: Vec3<T>,
    view_max: Vec3<T>,
    vpn: Vec3<T>,
    x_res: u16,
    y_res: u16,
}

impl<T: VertexComp> Camera<T> {
    pub fn new(
        look_at: Vec3<T>,
        look_from: Vec3<T>,
        up: Vec3<T>,
        width: T,
        height: T,
        horizontal_fov: T,
        x_res: u16,
        y_res: u16,
    ) -> Camera<T> {
        let aspect_ratio = width / height;

        let vertical_fov = T::from(2.0).unwrap()
            * ((horizontal_fov / T::from(2.0).unwrap()).tan() * aspect_ratio).atan();

        let horizontal_distance = (horizontal_fov / T::from(2.0).unwrap()).tan();
        let vertical_distance = (vertical_fov / T::from(2.0).unwrap()).tan();

        let vpn = look_from.sub(&look_at).normalize();

        Camera {
            look_at,
            look_from,
            up,
            aspect_ratio,
            horizontal_fov,
            vertical_fov,
            view_min: Vec3::new(-horizontal_distance, -vertical_distance, T::zero()),
            view_max: Vec3::new(horizontal_distance, vertical_distance, T::zero()),
            vpn,
            x_res,
            y_res,
        }
    }
}

use crate::common;

pub struct Camera<T: common::VertexComp> {
    look_from: common::Vec3<T>,
    look_at: common::Vec3<T>,
    up: common::Vec3<T>,
    aspect_ratio: T,
    horizontal_fov: T,
    vertical_fov: T,
    view_min: common::Vec3<T>,
    view_max: common::Vec3<T>,
    vpn: common::Vec3<T>,
    x_res: u16,
    y_res: u16,
}

impl<T: common::VertexComp> Camera<T> {
    pub fn new(
        look_at: common::Vec3<T>,
        look_from: common::Vec3<T>,
        up: common::Vec3<T>,
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
            view_min: common::Vec3::new(-horizontal_distance, -vertical_distance, T::zero()),
            view_max: common::Vec3::new(horizontal_distance, vertical_distance, T::zero()),
            vpn,
            x_res,
            y_res,
        }
    }
}

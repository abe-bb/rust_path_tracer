use crate::common::{Color, Spacial, Vec3, VertexFormat};

pub trait LightSource<T: VertexFormat>: Spacial<T> {
    fn set_color(&mut self, color: Color<T>);
    fn color(&self) -> &Color<T>;
    fn light_vector(&self, point: &Vec3<T>) -> Vec3<T>;
}

pub struct PointLight<T: VertexFormat> {
    color: Color<T>,
    position: Vec3<T>,
}

impl<T: VertexFormat> PointLight<T> {
    pub fn new(color: Color<T>, position: Vec3<T>) -> Self {
        PointLight { color, position }
    }
}

impl<T: VertexFormat> Spacial<T> for PointLight<T> {
    fn location(&self) -> &Vec3<T> {
        &self.position
    }
}

impl<T: VertexFormat> LightSource<T> for PointLight<T> {
    fn set_color(&mut self, color: Color<T>) {
        self.color = color;
    }

    fn color(&self) -> &Color<T> {
        &self.color
    }

    fn light_vector(&self, point: &Vec3<T>) -> Vec3<T> {
        self.position.sub(point).normalize()
    }
}

pub struct DirectionalLight<T: VertexFormat> {
    color: Color<T>,
    position: Vec3<T>,
    light_direction: Vec3<T>,
}

impl<T: VertexFormat> DirectionalLight<T> {
    pub fn new(color: Color<T>, light_direction: Vec3<T>) -> DirectionalLight<T> {
        DirectionalLight {
            color,
            position: Vec3::new(T::infinity(), T::infinity(), T::infinity()),
            light_direction,
        }
    }
}

impl<T: VertexFormat> Spacial<T> for DirectionalLight<T> {
    fn location(&self) -> &Vec3<T> {
        &self.position
    }
}

impl<T: VertexFormat> LightSource<T> for DirectionalLight<T> {
    fn set_color(&mut self, color: Color<T>) {
        self.color = color;
    }

    fn color(&self) -> &Color<T> {
        &self.color
    }

    fn light_vector(&self, point: &Vec3<T>) -> Vec3<T> {
        self.light_direction.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn point_light_vector_is_normalized() {
        let color = Color::from_color_vertex(Vec3::new(0.5, 0.5, 0.5)).unwrap();
        let position = Vec3::new(10.0, 10.0, 10.0);
        let intersection_point = Vec3::new(0.0, 0.0, 0.0);

        let point_light = PointLight::new(color, position);

        assert_eq!(
            1.0,
            point_light.light_vector(&intersection_point).mag_sqrd()
        )
    }

    #[test]
    fn point_light_vector_is_correct() {
        let color = Color::from_color_vertex(Vec3::new(0.5, 0.5, 0.5)).unwrap();
        let position = Vec3::new(10.0, 10.0, 10.0);
        let intersection_point = Vec3::new(0.0, 0.0, 0.0);

        let point_light = PointLight::new(color, position);

        let expected_light_vector =
            Vec3::new(1.0 / 3_f64.sqrt(), 1.0 / 3_f64.sqrt(), 1.0 / 3_f64.sqrt());
        let light_vector = point_light.light_vector(&intersection_point);

        assert!(approx_eq!(
            f64,
            expected_light_vector.x,
            light_vector.x,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            expected_light_vector.y,
            light_vector.y,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            expected_light_vector.z,
            light_vector.z,
            ulps = 2
        ));
    }
}

use ray_tracer::common::{Color, Vec3};
use ray_tracer::scene::camera::Camera;
use ray_tracer::scene::light::PointLight;
use ray_tracer::scene::visible::material::Material;
use ray_tracer::scene::visible::sphere::Sphere;
use ray_tracer::scene::visible::Body;
use ray_tracer::scene::Scene;

fn main() {
    let ambiant = Color::new(1.0, 1.0, 1.0).unwrap();

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0);
    let material = Material::new(
        0.5,
        Color::new(0.5, 0.5, 0.5).unwrap(),
        0.2,
        Color::new(0.6, 0.6, 0.6).unwrap(),
        32.0,
        0.1,
        ambiant.clone(),
        0.0,
    );
    let body = Body::new(Box::new(sphere), material);

    let visible = Box::new(body);
    let light_source = Box::new(PointLight::new(
        Color::new(1.0, 1.0, 0.0).unwrap(),
        Vec3::new(0.0, 3.0, 8.0),
    ));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 8.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        100,
        70.0_f64.to_radians(),
    );
    let background_color = Color::new(0.2, 0.2, 0.2).unwrap();

    let mut scene = Scene::new(camera, ambiant, background_color);

    scene.add_light(light_source);
    scene.add_visible(visible);

    let image = scene.render();
}

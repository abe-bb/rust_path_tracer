use ray_tracer::common::{Color, Vec3};
use ray_tracer::io::write_image_ppm;
use ray_tracer::scene::camera::Camera;
use ray_tracer::scene::light::PointLight;
use ray_tracer::scene::visible::material::Material;
use ray_tracer::scene::visible::sphere::Sphere;
use ray_tracer::scene::visible::Body;
use ray_tracer::scene::Scene;

fn main() {
    let ambiant = Color::new(1.0, 1.0, 1.0).unwrap();

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5);
    let material = Material::new(
        0.6,
        Color::new(0.5, 0.5, 0.5).unwrap(),
        1.0,
        Color::new(1.0, 1.0, 1.0).unwrap(),
        256.0,
        0.1,
        ambiant.clone(),
        0.1,
    );

    let sphere1 = Sphere::new(Vec3::new(-0.75, 0.5, 1.0), 0.2);
    let material1 = Material::new(
        0.6,
        Color::new(1.0, 1.0, 1.0).unwrap(),
        0.1,
        Color::new(1.0, 1.0, 0.1).unwrap(),
        256.0,
        0.1,
        ambiant.clone(),
        0.0,
    );

    let body = Body::new(Box::new(sphere), material);
    let body1 = Body::new(Box::new(sphere1), material1);

    let visible = Box::new(body);
    let visible1 = Box::new(body1);

    let light_source = Box::new(PointLight::new(
        Color::new(1.0, 1.0, 1.0).unwrap(),
        Vec3::new(0.0, 0.5, 3.0),
    ));

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 12.0),
        Vec3::new(0.0, 0.0, 20.0),
        Vec3::new(0.0, 1.0, 0.0),
        1920,
        1080,
        70.0_f64.to_radians(),
    );
    let background_color = Color::new(0.2, 0.2, 0.2).unwrap();

    let mut scene = Scene::new(camera, ambiant, background_color);

    scene.add_light(light_source);
    scene.add_visible(visible);
    scene.add_visible(visible1);

    let image = scene.render();

    write_image_ppm("test.ppm", &image);
}

use ray_tracer::common::{Color, Vec3};
use ray_tracer::io;
use ray_tracer::scene::camera::Camera;
use ray_tracer::scene::light::PointLight;
use ray_tracer::scene::visible::material::Material;
use ray_tracer::scene::visible::mesh::Triangle;
use ray_tracer::scene::visible::sphere::Sphere;
use ray_tracer::scene::visible::Body;
use ray_tracer::scene::Scene;

fn main() {
    let ambiant = Color::new(1.0, 1.0, 1.0).unwrap();

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5);
    let material = Material::new(
        0.6,
        Color::new(0.5, 0.5, 0.5).unwrap(),
        0.35,
        Color::new(1.0, 1.0, 1.0).unwrap(),
        64.0,
        0.1,
        ambiant.clone(),
        0.1,
    );

    let sphere1 = Sphere::new(Vec3::new(-0.75, 0.5, 1.0), 0.2);
    let material1 = Material::new(
        0.6,
        Color::new(0.3, 0.3, 1.0).unwrap(),
        0.3,
        Color::new(0.3, 0.3, 0.1).unwrap(),
        64.0,
        0.1,
        ambiant.clone(),
        0.0,
    );

    let triangle = Triangle::new(
        Vec3::new(0.55, -0.55, 4.0),
        Vec3::new(-0.25, 0.55, 4.0),
        Vec3::new(-0.55, -0.55, 4.0),
    );
    let material2 = Material::new(
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
    let body2 = Body::new(Box::new(triangle), material2);

    let visible = Box::new(body);
    let visible1 = Box::new(body1);
    let visible2 = Box::new(body2);

    let light_source1 = Box::new(PointLight::new(
        Color::new(0.80, 0.80, 0.80).unwrap(),
        Vec3::new(1.0, 0.5, 3.0),
    ));

    let light_source2 = Box::new(PointLight::new(
        Color::new(0.7, 0.7, 1.0).unwrap(),
        Vec3::new(-1.0, -0.5, 3.0),
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

    scene.add_light(light_source1);
    scene.add_light(light_source2);
    scene.add_visible(visible);
    scene.add_visible(visible1);
    // scene.add_visible(visible2);

    let image = scene.render();

    io::write_image_ppm("test.ppm", &image);
}

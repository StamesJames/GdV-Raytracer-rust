use cg_raytracer_rust::raytracer_lib::{raytracer::Raytracer, scene::Scene};

fn main() {
    // let vec_1 = Vec3([2.,2.,2.]);
    // println!("{:?}", vec_1.norm());
    // println!("{:?}", vec_1.normalized());
    // println!("{:?}", vec_1.normalized().norm());

    let scene = Scene::read_from_file("./res/scenes/spheres/spheres.sce").unwrap();
    println!("spheres Scene gelesen");
    let mut raytracer = Raytracer::new(scene, 1);
    println!("Raytracer initialized");
    raytracer.compute_image();
    let image = raytracer.image;
    println!("Image computed");
    if image.write_image("./results/spheres.png") {
        println!("hat was geschrieben;")
    } else {
        println!("hat nichts geschrieben")
    }
}

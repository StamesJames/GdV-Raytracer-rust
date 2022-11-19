use cg_raytracer_rust::raytracer_lib::{raytracer::Raytracer, scene::Scene, utils::vector::Vec3};

fn main() {
    // let vec_1 = Vec3([2.,2.,2.]);
    // println!("{:?}", vec_1.norm());
    // println!("{:?}", vec_1.normalized());
    // println!("{:?}", vec_1.normalized().norm());

    let scene = Scene::read_from_file("./res/scenes/spheres/spheres.sce").unwrap();
    println!("spheres Scene gelesen");
    let raytracer = Raytracer::new(scene, 1);
    println!("Raytracer initialized");
    let image = raytracer.compute_image();
    println!("Image computed");
    if image.write_image("./results/spheres.png") {
        println!("hat was geschrieben;")
    } else {
        println!("hat nichts geschrieben")
    }
    
}

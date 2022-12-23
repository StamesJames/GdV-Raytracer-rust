use cg_raytracer_rust::raytracer_lib::{raytracer::Raytracer, scene::Scene};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let scene = Scene::read_from_file("./res/scenes/spheres/spheres.sce").unwrap();
    let mut raytracer = Raytracer::new(scene, 1);

    c.bench_function("compute spheres", |b| b.iter(|| raytracer.compute_image()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

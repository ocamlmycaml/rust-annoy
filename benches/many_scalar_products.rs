#![feature(test)]

extern crate test;
extern crate annoy;
extern crate rand;

use test::Bencher;
use rand::Rng;

macro_rules! bench_vec_sizes {
    ($($name:ident => $dim:expr),*) => {
        $(
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut rng = rand::thread_rng();

                let v: Vec<f32> = (0..$dim).map(|_| rng.gen_range(0.0, 100.0)).collect();
                let w: Vec<f32> = (0..$dim).map(|_| rng.gen_range(0.0, 100.0)).collect();

                b.iter(|| annoy::scalar_product(&v, &w))
            }
        )*
    }
}

bench_vec_sizes! {
    scalar_product_3_vecs => 3,
    scalar_product_32_vecs => 32,
    scalar_product_64_vecs => 64,
    scalar_product_128_vecs => 128,
    scalar_product_256_vecs => 256,
    scalar_product_512_vecs => 512,
    scalar_product_1024_vecs => 1024
}

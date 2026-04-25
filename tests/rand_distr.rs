#![cfg(feature = "rand_distr")]

use microfloat::f8e4m3;
use rand::Rng;
use rand_distr::Distribution;

#[test]
fn rand_distr_samples() {
    let mut rng = rand::rng();
    let _: f8e4m3 = rng.sample(rand_distr::StandardUniform);
    let _: f8e4m3 = rng.sample(rand_distr::StandardNormal);
    let _: f8e4m3 = rng.sample(rand_distr::Exp1);
    let _: f8e4m3 = rng.sample(rand_distr::Open01);
    let _: f8e4m3 = rng.sample(rand_distr::OpenClosed01);

    let mut weight = <f8e4m3 as rand::distr::weighted::Weight>::ZERO;
    <f8e4m3 as rand::distr::weighted::Weight>::checked_add_assign(&mut weight, &f8e4m3::ONE)
        .unwrap();
    assert_eq!(weight, f8e4m3::ONE);

    let uniform = rand_distr::Uniform::new(f8e4m3::ZERO, f8e4m3::ONE).unwrap();
    let _: f8e4m3 = rng.sample(uniform);
    let inclusive = rand_distr::Uniform::new_inclusive(f8e4m3::ZERO, f8e4m3::ONE).unwrap();
    let sample: f8e4m3 = inclusive.sample(&mut rng);
    assert!(sample >= f8e4m3::ZERO && sample <= f8e4m3::ONE);

    assert!(rand_distr::Uniform::new(f8e4m3::ONE, f8e4m3::ZERO).is_err());
    assert!(rand_distr::Uniform::new_inclusive(f8e4m3::ONE, f8e4m3::ZERO).is_err());
}

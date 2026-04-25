use crate::{
    f4e2m1fn, f6e2m3fn, f6e3m2fn, f8e3m4, f8e4m3, f8e4m3b11fnuz, f8e4m3fn, f8e4m3fnuz, f8e5m2,
    f8e5m2fnuz, f8e8m0fnu,
};

use ::rand_distr::uniform::UniformFloat;
use rand::{Rng, distr::Distribution};

macro_rules! impl_distribution_via_f32 {
    ($type:ty, $distr:ty) => {
        impl Distribution<$type> for $distr {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $type {
                <$type>::from_f32(<Self as Distribution<f32>>::sample(self, rng))
            }
        }
    };
}

macro_rules! impl_rand_distr {
    ($($type:ty => $sampler:ident),* $(,)?) => {
        $(
            impl_distribution_via_f32!($type, ::rand_distr::StandardUniform);
            impl_distribution_via_f32!($type, ::rand_distr::StandardNormal);
            impl_distribution_via_f32!($type, ::rand_distr::Exp1);
            impl_distribution_via_f32!($type, ::rand_distr::Open01);
            impl_distribution_via_f32!($type, ::rand_distr::OpenClosed01);

            impl rand::distr::weighted::Weight for $type {
                const ZERO: Self = Self::ZERO;

                fn checked_add_assign(&mut self, value: &Self) -> Result<(), ()> {
                    *self += *value;
                    Ok(())
                }
            }

            #[derive(Debug, Clone, Copy)]
            pub struct $sampler(UniformFloat<f32>);

            impl ::rand_distr::uniform::SampleUniform for $type {
                type Sampler = $sampler;
            }

            impl ::rand_distr::uniform::UniformSampler for $sampler {
                type X = $type;

                fn new<B1, B2>(low: B1, high: B2) -> Result<Self, ::rand_distr::uniform::Error>
                where
                    B1: ::rand_distr::uniform::SampleBorrow<Self::X> + Sized,
                    B2: ::rand_distr::uniform::SampleBorrow<Self::X> + Sized,
                {
                    Ok(Self(UniformFloat::new(
                        low.borrow().to_f32(),
                        high.borrow().to_f32(),
                    )?))
                }

                fn new_inclusive<B1, B2>(
                    low: B1,
                    high: B2,
                ) -> Result<Self, ::rand_distr::uniform::Error>
                where
                    B1: ::rand_distr::uniform::SampleBorrow<Self::X> + Sized,
                    B2: ::rand_distr::uniform::SampleBorrow<Self::X> + Sized,
                {
                    Ok(Self(UniformFloat::new_inclusive(
                        low.borrow().to_f32(),
                        high.borrow().to_f32(),
                    )?))
                }

                fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                    <$type>::from_f32(self.0.sample(rng))
                }
            }
        )*
    };
}

impl_rand_distr!(
    f8e3m4 => Float8E3M4Sampler,
    f8e4m3 => Float8E4M3Sampler,
    f8e4m3b11fnuz => Float8E4M3B11FnuzSampler,
    f8e4m3fn => Float8E4M3FnSampler,
    f8e4m3fnuz => Float8E4M3FnuzSampler,
    f8e5m2 => Float8E5M2Sampler,
    f8e5m2fnuz => Float8E5M2FnuzSampler,
    f8e8m0fnu => Float8E8M0FnuSampler,
    f4e2m1fn => Float4E2M1FnSampler,
    f6e2m3fn => Float6E2M3FnSampler,
    f6e3m2fn => Float6E3M2FnSampler,
);

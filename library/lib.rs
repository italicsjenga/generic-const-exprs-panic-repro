#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub trait LibTrait {
    const NUM: usize;

    fn configure(&mut self, cfg: &Config<{ Self::NUM }>);
}

pub struct Config<const N: usize> {
    pub config: [u8; N],
}

pub struct ImplementsTraitOverConstGeneric<const N: usize>;

impl<const N: usize> LibTrait for ImplementsTraitOverConstGeneric<N> {
    const NUM: usize = N;

    fn configure(&mut self, _: &Config<{ Self::NUM }>) {}
}

// but this works:

// impl LibTrait for ImplementsTraitOverConstGeneric<4> {
//     const NUM: usize = 4;

//     fn configure(&mut self, _: &Config<{ Self::NUM }>) {}
// }

// or this:

pub struct NonTraitWithConstGeneric<const N: usize>;

impl<const N: usize> NonTraitWithConstGeneric<N> {
    pub const NUM: usize = N;

    pub fn configure(&mut self, _: &Config<{ Self::NUM }>) {}
}

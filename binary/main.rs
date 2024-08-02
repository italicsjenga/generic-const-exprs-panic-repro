use library::*;

fn main() {
    // this doesn't compile
    let mut inner = ImplementsTraitOverConstGeneric::<4>;

    // but this would
    // let mut inner = NonTraitWithConstGeneric::<4>;

    // if we don't call this function then it'll compile fine
    inner.configure(&Config {
        config: [0, 0, 0, 0],
    });
}

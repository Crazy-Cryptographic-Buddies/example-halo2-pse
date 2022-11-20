use std::marker::PhantomData;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::Chip;
use halo2_proofs::plonk::{Advice, Circuit, Column, ConstraintSystem, Fixed, Instance, Selector};
use halo2_proofs::poly::Rotation;

struct MyConfig {
    advice: [Column<Advice>; 3],
    instance: Column<Instance>,
    s_add: Selector,
    s_mul: Selector,
    s_add_c: Selector,
    s_mul_c: Selector,
}

struct FChip<F: FieldExt> {
    config: MyConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for FChip<F> {
    type Config = MyConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> FChip<F> {
    fn construct(config: <Self as Chip<F>>::Config, _loaded: <Self as Chip<F>>::Loaded) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 3],
        instance: Column<Instance>,
        constant: Column<Fixed>,
    ) -> <Self as Chip<F>>::Config {
        let s_add = meta.selector();
        let s_mul = meta.selector();
        let s_add_with_constant = meta.selector();
        let s_mul_with_constant = meta.selector();
        meta.create_gate("add", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[2], Rotation::cur());
            vec![s_add * (lhs + rhs - out)];
        });

        meta.create_gate("mul", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[2], Rotation::cur());
            vec![s_mul * (lhs * rhs - out)];
        });

        meta.create_gate("add with constant", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let fixed = meta.query_fixed(constant, Rotation::cur());
            let out = meta.query_advice(advice[2], Rotation::cur());
            vec![s_add_with_constant * (lhs + fixed - out)];
        });

        meta.create_gate("mul with constant", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let fixed = meta.query_fixed(constant, Rotation::cur());
            let out = meta.query_advice(advice[2], Rotation::cur());
            vec![s_mul_with_constant * (lhs * fixed - out)];
        });

        Self::Config {
            advice: advice,
            instance: instance,
            s_add: s_add,
            s_mul: s_mul,
            s_add_c: s_add_with_constant,
            s_mul_c: s_mul_with_constant,
        }
    }
}



fn main() {
    println!("Hello, world!");
}

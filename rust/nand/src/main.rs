use std::marker::PhantomData as P;

trait Eval {
    type Output;
}

struct T;
struct F;

#[rustfmt::skip]
impl Eval for T { type Output = T; }
#[rustfmt::skip]
impl Eval for F { type Output = F; }

struct Nand<L, R>(P<(L, R)>);

impl<L, R> Eval for Nand<L, R>
where
    L: Eval,
    R: Eval,
    Nand<<L as Eval>::Output, <R as Eval>::Output>: EvalPrim,
{
    type Output = <Nand<<L as Eval>::Output, <R as Eval>::Output> as EvalPrim>::Output;
}

trait EvalPrim {
    type Output;
}

#[rustfmt::skip]
impl EvalPrim for Nand<T, T> { type Output = F; }
#[rustfmt::skip]
impl EvalPrim for Nand<F, T> { type Output = T; }
#[rustfmt::skip]
impl EvalPrim for Nand<T, F> { type Output = T; }
#[rustfmt::skip]
impl EvalPrim for Nand<F, F> { type Output = T; }

type Not<T> = Nand<T, T>;
type And<L, R> = Not<Nand<L, R>>;
type Or<L, R> = Nand<Not<L>, Not<R>>; // A || B  = !(!A && !B)

type SuperNot = Not<Not<Not<Not<Not<T>>>>>;

fn main() {
    println!(
        "{}",
        std::any::type_name::<<And<T, Or<F, Not<T>>> as Eval>::Output>()
    );

    println!(
        "{}",
        std::any::type_name::<<Not<SuperNot> as Eval>::Output>()
    );
}

use std::marker::PhantomData;

use crate::world::UnsafeWorldCell;

use super::{IntoSystem, System, SystemParam, SystemParamItem};

pub struct FunctionSystem<Marker, F> {
    name: &'static str,
    func: F,
    marker: PhantomData<Marker>,
}

impl<Marker, F> System for FunctionSystem<Marker, F>
where
    Marker: 'static,
    F: SystemParamFunction<Marker>,
{
    fn run(&mut self, world: &mut crate::world::World) {
        let unsafe_world_cell = UnsafeWorldCell::new(world);
        let params = F::Param::get_param(unsafe_world_cell);
        self.func.run(params)
    }
}

impl<Marker, F> IntoSystem<Marker> for F
where
    Marker: 'static,
    F: SystemParamFunction<Marker>,
{
    type System = FunctionSystem<Marker, F>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            name: std::any::type_name::<F>(),
            func: self,
            marker: PhantomData,
        }
    }
}

pub trait SystemParamFunction<Marker>: 'static {
    type Param: SystemParam;

    fn run(&mut self, param_value: SystemParamItem<Self::Param>);
}

impl<F> SystemParamFunction<fn()> for F
where
    F: 'static,
    for<'a> &'a mut F: FnMut(),
{
    type Param = ();

    fn run(&mut self, param_value: SystemParamItem<Self::Param>) {
        fn call_inner(mut f: impl FnMut()) {
            f()
        }

        call_inner(self)
    }
}

macro_rules! impl_spf_from {
    ( $($n:ident),* ) => {
        #[allow(non_snake_case)]
        impl<F, $($n,)*> SystemParamFunction<fn($($n,)*)> for F
        where
            F: 'static,
            for<'a> &'a mut F: FnMut($($n,)*) + FnMut($(SystemParamItem<$n>,)*),
            $($n: SystemParam,)*
        {
            type Param = ($($n,)*);

            fn run(&mut self, param_value: SystemParamItem<Self::Param>) {
                fn call_inner<$($n,)*>(mut f: impl FnMut($($n,)*), $($n: $n,)*) {
                    f($($n,)*)
                }
                let ($($n,)*) = param_value;
                call_inner(self, $($n,)*)
            }
        }
    };
}

impl_spf_from!(T1);
impl_spf_from!(T1, T2);
impl_spf_from!(T1, T2, T3);
impl_spf_from!(T1, T2, T3, T4);
impl_spf_from!(T1, T2, T3, T4, T5);
impl_spf_from!(T1, T2, T3, T4, T5, T6);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_spf_from!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

use crate::world::{UnsafeWorldCell, World};

pub trait SystemParam: Sized {
    type Item<'world>: SystemParam;

    fn get_param<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world>;
}

pub type SystemParamItem<'w, P> = <P as SystemParam>::Item<'w>;

impl SystemParam for () {
    type Item<'world> = ();

    fn get_param<'world>(_world: UnsafeWorldCell<'world>) -> Self::Item<'world> {
        ()
    }
}

// impl<T> SystemParam for T
// where
//     T: SystemParam,
// {
//     type Item<'world> = T::Item<'world>;

//     fn get_param<'world>(_world: UnsafeWorldCell<'world>) -> Self::Item<'world> {
//         ()
//     }
// }

macro_rules! impl_sp_for {
    ( $($n:ident),* ) => {
        #[allow(non_snake_case)]
        impl<$($n,)*> SystemParam for ($($n,)*)
        where $($n: SystemParam,)* {
            type Item<'world> = ($($n::Item<'world>,)*);

            fn get_param<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world> {
                $(let $n = $n::get_param(world);)*

                ($($n,)*)
            }
        }
    };
}

impl_sp_for!(T1);
impl_sp_for!(T1, T2);
impl_sp_for!(T1, T2, T3);
impl_sp_for!(T1, T2, T3, T4);
impl_sp_for!(T1, T2, T3, T4, T5);
impl_sp_for!(T1, T2, T3, T4, T5, T6);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_sp_for!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

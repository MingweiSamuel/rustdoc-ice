#![feature(generic_associated_types)]

pub trait MyTrait {
    type MyGat<'s>
    where
        Self: 's;
    fn my_fn(&self) -> Self::MyGat<'_>;
}

impl MyTrait for () {
    type MyGat<'s> = &'s ();
    fn my_fn(&self) -> Self::MyGat<'_> {
        &()
    }
}

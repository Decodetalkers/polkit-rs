use crate::Listener;
use glib::{object::IsA, subclass::prelude::*};
unsafe impl<T: ListenerImpl> IsSubclassable<T> for Listener {}

impl<T: ListenerImpl> ListenerImplExt for T {}

pub trait ListenerImpl: ObjectImpl + ObjectSubclass<Type: IsA<Listener>> {}

pub trait ListenerImplExt: ListenerImpl {}

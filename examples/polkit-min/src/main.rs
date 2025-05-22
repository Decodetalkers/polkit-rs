use polkit_agent::Listener;

use glib::prelude::*;
use glib::subclass::prelude::*;
use polkit_agent::traits::ListenerExt;

glib::wrapper! {
     pub struct MyPolkit(ObjectSubclass<imp::MyPolkit>)
         @implements Listener;
}

impl Default for MyPolkit {
    fn default() -> Self {
        glib::Object::new()
    }
}

mod imp {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use polkit_agent::subclass::ListenerImpl;

    #[derive(Default)]
    pub struct MyPolkit;

    #[glib::object_subclass]
    impl ObjectSubclass for MyPolkit {
        const NAME: &'static str = "MyPolkit";
        type Type = super::MyPolkit;
        type ParentType = super::Listener;

        fn class_init(klass: &mut Self::Class) {}
    }

    impl ObjectImpl for MyPolkit {
        fn constructed(&self) {}
        fn dispose(&self) {}
    }

    impl ListenerImpl for MyPolkit {}
}

fn main() {
    println!("Hello, world!");
}

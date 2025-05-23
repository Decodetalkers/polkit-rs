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

    impl ListenerImpl for MyPolkit {}
    #[glib::object_subclass]
    impl ObjectSubclass for MyPolkit {
        const NAME: &'static str = "MyPolkit";
        type Type = super::MyPolkit;
        type ParentType = super::Listener;

        fn class_init(klass: &mut Self::Class) {
            let klass = klass.as_mut();
        }
    }

    impl ObjectImpl for MyPolkit {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
        }
        fn dispose(&self) {}
    }
}

fn main() {
    println!("Hello, world!");
}

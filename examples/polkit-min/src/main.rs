use polkit_agent::Listener;

use glib::prelude::*;
use glib::subclass::prelude::*;

glib::wrapper! {
     pub struct MyPolkit(ObjectSubclass<imp::MyPolkit>)
         @extends Listener;
}

impl Default for MyPolkit {
    fn default() -> Self {
        glib::Object::new()
    }
}

mod imp {
    use glib::subclass::prelude::*;
    use polkit_agent::subclass::ListenerImpl;

    use polkit_agent::gio;
    use polkit_agent::polkit;
    #[derive(Default)]
    pub struct MyPolkit;

    impl ListenerImpl for MyPolkit {
        fn initilate_authentication(
            &self,
            _action_id: &str,
            _message: &str,
            _icon_name: &str,
            _details: &polkit::Details,
            _cookie: &str,
            _identities: &[polkit::Identity],
            _cancelable: gio::Cancellable,
            _callback: gio::ffi::GAsyncReadyCallback,
            _user_data: glib::ffi::gpointer,
        ) {
        }
    }
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

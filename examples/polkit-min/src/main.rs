use polkit_agent::Listener;
use polkit_agent::RegisterFlags;
use polkit_agent::gio;
use polkit_agent::polkit::UnixSession;
use polkit_agent::traits::ListenerExt;
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
            identities: &[polkit::Identity],
            _cancelable: gio::Cancellable,
            _callback: gio::ffi::GAsyncReadyCallback,
            _user_data: glib::ffi::gpointer,
        ) {
            println!("{_cookie}");
            for identity in identities {}
        }
        fn initiate_authentication_finish(
            &self,
            _gio_result: gio::AsyncResult,
            _error: Option<glib::Error>,
        ) -> bool {
            true
        }
    }
    #[glib::object_subclass]
    impl ObjectSubclass for MyPolkit {
        const NAME: &'static str = "MyPolkit";
        type Type = super::MyPolkit;
        type ParentType = super::Listener;

        fn class_init(_klass: &mut Self::Class) {}
    }

    impl ObjectImpl for MyPolkit {
        fn constructed(&self) {
            self.parent_constructed();
        }
        fn dispose(&self) {}
    }
}

const OBJECT_PATH: &str = "/org/waycrate/PolicyKit1/AuthenticationAgent";

fn main() {
    let main_loop = glib::MainLoop::new(None, true);
    let my_polkit = MyPolkit::default();

    let Ok(Some(subject)) =
        UnixSession::new_for_process_sync(nix::unistd::getpid().as_raw(), gio::Cancellable::NONE)
    else {
        return;
    };
    let Ok(_handle) = my_polkit.register(
        RegisterFlags::NONE,
        &subject,
        OBJECT_PATH,
        gio::Cancellable::NONE,
    ) else {
        return;
    };
    main_loop.run();
}

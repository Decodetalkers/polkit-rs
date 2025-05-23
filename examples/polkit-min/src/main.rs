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
    use dialoguer::FuzzySelect;
    use dialoguer::theme::ColorfulTheme;
    use glib::object::Cast;
    use glib::subclass::prelude::*;
    use polkit_agent::gio::prelude::CancellableExt;
    use polkit_agent::subclass::ListenerImpl;

    use polkit_agent::Session as AgentSession;
    use polkit_agent::gio;
    use polkit_agent::polkit;
    use polkit_agent::polkit::UnixUser;
    use rpassword::prompt_password;

    fn choose_user(users: &[UnixUser]) -> Option<(String, usize)> {
        let names: Vec<String> = users
            .iter()
            .map(|user| user.name().unwrap().to_string())
            .collect();
        let index = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Which user?")
            .default(0)
            .items(&names)
            .interact()
            .ok()?;
        Some((names[index].clone(), index))
    }

    #[derive(Default)]
    pub struct MyPolkit;

    impl ListenerImpl for MyPolkit {
        fn initiate_authentication(
            &self,
            _action_id: &str,
            _message: &str,
            _icon_name: &str,
            _details: &polkit::Details,
            cookie: &str,
            identities: Vec<polkit::Identity>,
            cancelable: gio::Cancellable,
            task: gio::Task<String>,
        ) {
            let sub_loop = glib::MainLoop::new(None, true);
            let users: Vec<UnixUser> = identities
                .into_iter()
                .map(|idenifier| idenifier.dynamic_cast())
                .flatten()
                .collect();
            let Some((name, index)) = choose_user(&users) else {
                cancelable.cancel();
                return;
            };
            let session = AgentSession::new(&users[index], cookie);

            let sub_loop2 = sub_loop.clone();
            session.connect_completed(move |session, success| {
                if success {
                    if success {
                        println!("succeeded");
                    }
                    session.cancel();
                    unsafe {
                        task.clone().return_result(Ok("success".to_string()));
                    }
                    sub_loop2.quit();
                }
            });
            session.connect_show_info(|_session, info| {
                println!("info: {info}");
            });
            session.connect_show_error(|_session, error| {
                eprintln!("error: {error}");
            });
            session.connect_request(move |session, request, _echo_on| {
                if !request.starts_with("Password:") {
                    return;
                }
                let Ok(password) = prompt_password(format!("{name} password: ")) else {
                    session.cancel();
                    cancelable.cancel();
                    return;
                };
                session.response(&password);
            });
            session.initiate();
            sub_loop.run();
        }
        fn initiate_authentication_finish(
            &self,
            gio_result: gio::AsyncResult,
            _error: Option<glib::Error>,
        ) -> bool {
            println!("finally!");
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

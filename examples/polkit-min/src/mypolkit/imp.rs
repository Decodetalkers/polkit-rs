use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use glib::MainLoop;
use glib::object::Cast;
use glib::subclass::prelude::*;
use polkit_agent_rs::Session as AgentSession;
use polkit_agent_rs::gio;
use polkit_agent_rs::gio::prelude::CancellableExt;
use polkit_agent_rs::polkit;
use polkit_agent_rs::polkit::UnixUser;
use polkit_agent_rs::subclass::ListenerImpl;
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
use std::sync::Arc;
use std::sync::atomic::AtomicU8;

fn start_session(
    session: &AgentSession,
    name: String,
    cancellable: gio::Cancellable,
    task: gio::Task<String>,
    sub_loop: MainLoop,
    cookie: String,
    count: Arc<AtomicU8>,
) {
    let name2 = name.clone();
    let cancellable2 = cancellable.clone();

    session.connect_completed(move |session, success| {
        let name2 = name2.clone();
        let cancellable2 = cancellable2.clone();
        let task = task.clone();
        let sub_loop2 = sub_loop.clone();
        let cookie = cookie.clone();
        let count = count.clone();
        if !success {
            count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if count.load(std::sync::atomic::Ordering::Relaxed) >= 3 {
                unsafe {
                    task.return_result(Ok("Failed".to_string()));
                }
                session.cancel();

                sub_loop.quit();
                return;
            }
            let user: UnixUser = UnixUser::new_for_name(&name2).unwrap();
            let session = AgentSession::new(&user, &cookie);
            start_session(
                &session,
                name2,
                cancellable2,
                task,
                sub_loop2,
                cookie,
                count,
            );
        } else {
            unsafe {
                task.return_result(Ok("success".to_string()));
            }
        }
        session.cancel();

        sub_loop.quit();
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
            cancellable.cancel();
            return;
        };
        session.response(&password);
    });
    session.initiate();
    println!("ggg");
}

impl ListenerImpl for MyPolkit {
    type Message = String;
    fn initiate_authentication(
        &self,
        _action_id: &str,
        _message: &str,
        _icon_name: &str,
        _details: &polkit::Details,
        cookie: &str,
        identities: Vec<polkit::Identity>,
        cancellable: gio::Cancellable,
        task: gio::Task<Self::Message>,
    ) {
        let sub_loop = glib::MainLoop::new(None, true);
        let users: Vec<UnixUser> = identities
            .into_iter()
            .flat_map(|idenifier| idenifier.dynamic_cast())
            .collect();
        let Some((name, index)) = choose_user(&users) else {
            cancellable.cancel();
            return;
        };
        let session = AgentSession::new(&users[index], cookie);

        let sub_loop2 = sub_loop.clone();
        let count = Arc::new(AtomicU8::new(0));
        start_session(
            &session,
            name,
            cancellable,
            task,
            sub_loop2,
            cookie.to_string(),
            count,
        );
        sub_loop.run();
    }
    fn initiate_authentication_finish(
        &self,
        gio_result: Result<gio::Task<Self::Message>, glib::Error>,
    ) -> bool {
        match gio_result {
            Ok(_) => true,
            Err(err) => {
                println!("err: {err:?}");
                false
            }
        }
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

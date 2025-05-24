use polkit_agent_rs::RegisterFlags;
use polkit_agent_rs::gio;
use polkit_agent_rs::polkit::UnixSession;
use polkit_agent_rs::traits::ListenerExt;
mod mypolkit;
use mypolkit::MyPolkit;

const OBJECT_PATH: &str = "/org/waycrate/PolicyKit1/AuthenticationAgent";

fn main() {
    let main_loop = glib::MainLoop::new(None, true);
    let my_polkit = MyPolkit::default();

    let Ok(subject) =
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

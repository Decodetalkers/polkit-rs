use polkit_agent_rs::Listener;
mod imp;

glib::wrapper! {
     pub struct MyPolkit(ObjectSubclass<imp::MyPolkit>)
         @extends Listener;
}

impl Default for MyPolkit {
    fn default() -> Self {
        glib::Object::new()
    }
}

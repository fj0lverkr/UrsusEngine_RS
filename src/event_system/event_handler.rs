use sdl2::event::Event;
pub trait EventHandler {
    fn handle_event(&self, _: &Event) -> bool {
        false
    }
}

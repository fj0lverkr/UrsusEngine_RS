use sdl2::event::Event;
pub trait EventReceiver {
    fn handle_event(&self, _: Event) -> bool {
        false
    }
}

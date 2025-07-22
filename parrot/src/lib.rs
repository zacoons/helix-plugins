use helix_event::register_hook;
use helix_term::events::PostInsertChar;
use helix_term::plugins::Plugins;

pub struct Plugin {}

impl Plugin {
    #[unsafe(no_mangle)]
    fn init(config: &helix_term::config::Config) -> &mut Plugin {
        let slf = Plugins::alloc::<Plugin>();

        register_hook!(move |event: &mut PostInsertChar<'_, '_>| {
            event.cx.editor.set_status(event.c.to_string());
            Ok(())
        });

        slf
    }
}

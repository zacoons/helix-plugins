use helix_event::register_hook;
use helix_term::compositor::Context;
use helix_term::events::ConfigDidChange;
use helix_term::plugins::Plugins;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub str: Option<String>,
}

pub struct Plugin {
    str: String,
}

impl Plugin {
    #[unsafe(no_mangle)]
    fn init(config: &helix_term::config::Config) -> &mut Plugin {
        let slf = Plugins::alloc::<Plugin>();
        slf.str = "Hello world!".to_string();

        slf.config_updated(config);
        // register_hook!(move |event: &mut ConfigDidChange<'_>| {
        //     // slf.config_updated(event.new);
        //     Ok(())
        // });

        slf
    }

    #[unsafe(no_mangle)]
    fn config_updated(&mut self, config: &helix_term::config::Config) {
        if let Some(plugin_config) = Plugins::get_config_for("hello_world") {
            if let Some(str) = plugin_config.get("str") {
                self.str = str.as_str().unwrap().to_string();
            }
        }
    }

    #[unsafe(no_mangle)]
    fn available_commands(&mut self) -> Vec<String> {
        vec![String::from("hello")]
    }

    #[unsafe(no_mangle)]
    fn hello(&mut self, cx: &mut Context, args: &str) {
        cx.editor.set_error(self.str.to_string());
    }
}

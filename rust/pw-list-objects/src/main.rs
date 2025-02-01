use pipewire::{context::Context, main_loop::MainLoop};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mainloop = MainLoop::new(None)?;
    let context = Context::new(&mainloop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let globals = Arc::new(Mutex::new(Vec::new()));

    // Add registry listener to collect global objects
    let globals_clone = globals.clone();
    let _listener = registry
        .add_listener_local()
        .global(move |global| {
            let mut guard = globals_clone.lock().unwrap();
            let id = global.id.clone();
            // TODO: Source Device でFilter
            if let Some(d) = global.props {
                let nick = d.get("node.nick").and_then(|s| Some(s.to_string()));
                let class = d.get("media.class").and_then(|s| Some(s.to_string()));
                if nick.is_some() && class.is_some() {
                    guard.push((id, nick.unwrap(), class.unwrap()));
                }
            }
        })
        .register();

    let _sync = core.sync(0)?;
    let _core_listner = {
        let mainloop_clone = mainloop.clone();
        core.add_listener_local()
            .done(move |_id, _seq| {
                mainloop_clone.quit();
            })
            .register()
    };

    // Run the main loop to collect objects
    mainloop.run();

    // Extract and sort collected objects by ID
    let mut globals = globals.lock().unwrap();
    globals.sort();

    drop(dbg!(globals));

    Ok(())
}

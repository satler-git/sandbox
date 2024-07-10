use std::{thread::sleep, time::Duration};

use anyhow::{Result};
use tray_icon::{menu, Icon, TrayIconBuilder};

fn water_notify() -> Result<()> {
    Ok(notifica::notify("Water notifier", "水を飲んでね")?)
}

fn main() -> Result<()> {
    let icon = Icon::from_rgba(vec![65, 199, 255, 204], 1, 1)?;
    let tray_menu = menu::Menu::with_items(&[&{
        menu::MenuItem::new(
            "quit",
            true,
            Some(menu::accelerator::Accelerator::new(
                None,
                menu::accelerator::Code::KeyQ,
            )),
        )
    }])?;

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Water notifier")
        .with_icon(icon)
        .build()?;

    loop {
        sleep(Duration::from_secs(60 * 90));
        water_notify()?;
    }
}

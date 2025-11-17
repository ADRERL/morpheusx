// Input handling for installer menu
use super::state::InstallerMenu;
use crate::tui::input::Keyboard;
use crate::tui::renderer::Screen;
use crate::BootServices;

pub fn handle_key_input(
    menu: &mut InstallerMenu,
    key: crate::tui::input::Key,
    screen: &mut Screen,
    keyboard: &mut Keyboard,
    bs: &BootServices,
) -> bool {
    // Global rain toggle
    if key.unicode_char == b'x' as u16 || key.unicode_char == b'X' as u16 {
        crate::tui::rain::toggle_rain(screen);
        screen.clear();
        super::render::render_menu(menu, screen, bs);
        return false;
    }

    match key.scan_code {
        0x01 => navigate_up(menu),
        0x02 => navigate_down(menu),
        0x17 => return true, // ESC - exit
        _ => handle_enter_key(menu, key, screen, keyboard, bs),
    }
    false
}

fn navigate_up(menu: &mut InstallerMenu) {
    if menu.selected_esp > 0 {
        menu.selected_esp -= 1;
    }
}

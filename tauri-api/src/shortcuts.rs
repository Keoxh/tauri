use hotkey;

/// The shortcut manager builder.
pub struct ShortcutManager {
  hk: hotkey::Listener,
}

impl Default for ShortcutManager {
  fn default() -> Self {
    let hk = hotkey::Listener::new();
    Self { hk }
  }
}

impl ShortcutManager {
  /// Initializes a new instance of the shortcut manager.
  pub fn new() -> Self {
    Default::default()
  }

  /// Registers a new shortcut handler.
  pub fn register_shortcut<H: Fn() + 'static, E: Fn(String)>(
    &mut self,
    shortcut: String,
    handler: H,
    error: E,
  ) {
    let hk = &mut self.hk;

    let mut shortcut_modifier: u32 = 0;
    let mut shortcut_key: u32 = 0;

    let mut modifiers = shortcut.split('+').peekable();
    while let Some(key) = modifiers.next() {
      if modifiers.peek().is_some() {
        let hotkey_modifier = match key.to_uppercase().as_str() {
          "ALT" => hotkey::modifiers::ALT,
          "CONTROL" | "CTRL" => hotkey::modifiers::CONTROL,
          "SHIFT" => hotkey::modifiers::SHIFT,
          "SUPER" => hotkey::modifiers::SUPER,
          _ => {
            error(format!("unknown modifier {}", key));
            return;
          }
        };
        shortcut_modifier |= hotkey_modifier;
      } else {
        let hotkey_key = match key.to_uppercase().as_str() {
          "BACKSPACE" => hotkey::keys::BACKSPACE,
          "TAB" => hotkey::keys::TAB,
          "ENTER" | "RETURN" => hotkey::keys::ENTER,
          "CAPSLOCK" => hotkey::keys::CAPS_LOCK,
          "ESCAPE" => hotkey::keys::ESCAPE,
          "SPACEBAR" => hotkey::keys::SPACEBAR,
          "PAGEUP" => hotkey::keys::PAGE_UP,
          "PAGEDOWN" => hotkey::keys::PAGE_DOWN,
          "END" => hotkey::keys::END,
          "HOME" => hotkey::keys::HOME,
          "LEFT" => hotkey::keys::ARROW_LEFT,
          "RIGHT" => hotkey::keys::ARROW_RIGHT,
          "UP" => hotkey::keys::ARROW_UP,
          "DOWN" => hotkey::keys::ARROW_DOWN,
          "PRINTSCREEN" => hotkey::keys::PRINT_SCREEN,
          "INSERT" => hotkey::keys::INSERT,
          "DELETE" => hotkey::keys::DELETE,
          _ => {
            let chars: Vec<char> = key.chars().collect();
            if chars.len() != 1 {
              error(format!(
                "shortcut '{}' last element should be a character, found {}",
                shortcut, key
              ));
              return;
            } else {
              chars[0] as u32
            }
          }
        };
        shortcut_key = hotkey_key;
      }
    }

    let hotkey_registration = hk.register_hotkey(shortcut_modifier, shortcut_key, handler);

    if let Err(e) = hotkey_registration {
      error(e);
    }
  }

  /// Starts listening to the shortcuts.
  pub fn listen(self) {
    self.hk.listen();
  }
}
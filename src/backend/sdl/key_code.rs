use crate::KeyCode;

pub fn convert_key_code(key: Option<sdl2::keyboard::Keycode>) -> KeyCode {
	let key = match key {
		Some(x) => x,
		None => return KeyCode::Unidentified,
	};

	use sdl2::keyboard::Keycode as I;
	let chr = |x: &str| KeyCode::Character(x.into());

	match key {
		I::Backspace          => KeyCode::Backspace,
		I::Tab                => KeyCode::Tab,
		I::Return             => KeyCode::Enter,
		I::Escape             => KeyCode::Escape,
		I::Space              => chr(" "),
		I::Exclaim            => chr("!"),
		I::Quotedbl           => chr("\""),
		I::Hash               => chr("#"),
		I::Dollar             => chr("$"),
		I::Percent            => chr("%"),
		I::Ampersand          => chr("&"),
		I::Quote              => chr("'"),
		I::LeftParen          => chr("("),
		I::RightParen         => chr(")"),
		I::Asterisk           => chr("*"),
		I::Plus               => chr("+"),
		I::Comma              => chr("),"),
		I::Minus              => chr("-"),
		I::Period             => chr("."),
		I::Slash              => chr("/"),
		I::Num0               => chr("0"),
		I::Num1               => chr("1"),
		I::Num2               => chr("2"),
		I::Num3               => chr("3"),
		I::Num4               => chr("4"),
		I::Num5               => chr("5"),
		I::Num6               => chr("6"),
		I::Num7               => chr("7"),
		I::Num8               => chr("8"),
		I::Num9               => chr("9"),
		I::Colon              => chr(":"),
		I::Semicolon          => chr(";"),
		I::Less               => chr("<"),
		I::Equals             => chr("="),
		I::Greater            => chr(">"),
		I::Question           => chr("?"),
		I::At                 => chr("@"),
		I::LeftBracket        => chr("["),
		I::Backslash          => chr("\\"),
		I::RightBracket       => chr("]"),
		I::Caret              => chr("^"),
		I::Underscore         => chr("_"),
		I::Backquote          => chr("`"),
		I::A                  => chr("A"),
		I::B                  => chr("B"),
		I::C                  => chr("C"),
		I::D                  => chr("D"),
		I::E                  => chr("E"),
		I::F                  => chr("F"),
		I::G                  => chr("G"),
		I::H                  => chr("H"),
		I::I                  => chr("I"),
		I::J                  => chr("J"),
		I::K                  => chr("K"),
		I::L                  => chr("L"),
		I::M                  => chr("M"),
		I::N                  => chr("N"),
		I::O                  => chr("O"),
		I::P                  => chr("P"),
		I::Q                  => chr("Q"),
		I::R                  => chr("R"),
		I::S                  => chr("S"),
		I::T                  => chr("T"),
		I::U                  => chr("U"),
		I::V                  => chr("V"),
		I::W                  => chr("W"),
		I::X                  => chr("X"),
		I::Y                  => chr("Y"),
		I::Z                  => chr("Z"),
		I::Delete             => KeyCode::Delete,
		I::CapsLock           => KeyCode::CapsLock,
		I::F1                 => KeyCode::F1,
		I::F2                 => KeyCode::F2,
		I::F3                 => KeyCode::F3,
		I::F4                 => KeyCode::F4,
		I::F5                 => KeyCode::F5,
		I::F6                 => KeyCode::F6,
		I::F7                 => KeyCode::F7,
		I::F8                 => KeyCode::F8,
		I::F9                 => KeyCode::F9,
		I::F10                => KeyCode::F10,
		I::F11                => KeyCode::F11,
		I::F12                => KeyCode::F12,
		I::PrintScreen        => KeyCode::PrintScreen,
		I::ScrollLock         => KeyCode::ScrollLock,
		I::Pause              => KeyCode::Pause,
		I::Insert             => KeyCode::Insert,
		I::Home               => KeyCode::Home,
		I::PageUp             => KeyCode::PageUp,
		I::End                => KeyCode::End,
		I::PageDown           => KeyCode::PageDown,
		I::Right              => KeyCode::ArrowRight,
		I::Left               => KeyCode::ArrowLeft,
		I::Down               => KeyCode::ArrowDown,
		I::Up                 => KeyCode::ArrowUp,
		I::NumLockClear       => KeyCode::NumLock,
		I::KpDivide           => chr("/"),
		I::KpMultiply         => chr("*"),
		I::KpMinus            => chr("-"),
		I::KpPlus             => chr("+"),
		I::KpEnter            => KeyCode::Enter,
		I::Kp1                => chr("1"),
		I::Kp2                => chr("2"),
		I::Kp3                => chr("3"),
		I::Kp4                => chr("4"),
		I::Kp5                => chr("5"),
		I::Kp6                => chr("6"),
		I::Kp7                => chr("7"),
		I::Kp8                => chr("8"),
		I::Kp9                => chr("9"),
		I::Kp0                => chr("0"),
		I::KpPeriod           => chr("."),
		I::Application        => KeyCode::ContextMenu,
		I::Power              => KeyCode::Power,
		I::KpEquals           => chr("="),
		I::F13                => KeyCode::Unidentified,
		I::F14                => KeyCode::Unidentified,
		I::F15                => KeyCode::Unidentified,
		I::F16                => KeyCode::Unidentified,
		I::F17                => KeyCode::Unidentified,
		I::F18                => KeyCode::Unidentified,
		I::F19                => KeyCode::Unidentified,
		I::F20                => KeyCode::Unidentified,
		I::F21                => KeyCode::Unidentified,
		I::F22                => KeyCode::Unidentified,
		I::F23                => KeyCode::Unidentified,
		I::F24                => KeyCode::Unidentified,
		I::Execute            => KeyCode::Execute,
		I::Help               => KeyCode::Help,
		I::Menu               => KeyCode::ContextMenu, // Not sure
		I::Select             => KeyCode::Select,
		I::Stop               => KeyCode::MediaStop,
		I::Again              => KeyCode::Again,
		I::Undo               => KeyCode::Undo,
		I::Cut                => KeyCode::Cut,
		I::Copy               => KeyCode::Copy,
		I::Paste              => KeyCode::Paste,
		I::Find               => KeyCode::Find,
		I::Mute               => KeyCode::AudioVolumeMute,
		I::VolumeUp           => KeyCode::AudioVolumeUp,
		I::VolumeDown         => KeyCode::AudioVolumeDown,
		I::KpComma            => chr(","),
		I::KpEqualsAS400      => chr("="),
		I::AltErase           => KeyCode::Unidentified,
		I::Sysreq             => KeyCode::Unidentified,
		I::Cancel             => KeyCode::Cancel,
		I::Clear              => KeyCode::Clear,
		I::Prior              => KeyCode::PageUp,
		I::Return2            => KeyCode::Enter,
		I::Separator          => KeyCode::Unidentified,
		I::Out                => KeyCode::Unidentified,
		I::Oper               => KeyCode::Unidentified,
		I::ClearAgain         => KeyCode::Unidentified,
		I::CrSel              => KeyCode::CrSel,
		I::ExSel              => KeyCode::ExSel,
		I::Kp00               => KeyCode::Unidentified,
		I::Kp000              => KeyCode::Unidentified,
		I::ThousandsSeparator => KeyCode::Unidentified,
		I::DecimalSeparator   => KeyCode::Unidentified,
		I::CurrencyUnit       => KeyCode::Unidentified,
		I::CurrencySubUnit    => KeyCode::Unidentified,
		I::KpLeftParen        => chr("("),
		I::KpRightParen       => chr(")"),
		I::KpLeftBrace        => chr("["),
		I::KpRightBrace       => chr("]"),
		I::KpTab              => chr("\t"),
		I::KpBackspace        => KeyCode::Backspace,
		I::KpA                => chr("A"),
		I::KpB                => chr("B"),
		I::KpC                => chr("C"),
		I::KpD                => chr("D"),
		I::KpE                => chr("E"),
		I::KpF                => chr("F"),
		I::KpXor              => KeyCode::Unidentified,
		I::KpPower            => chr("^"),
		I::KpPercent          => chr("%"),
		I::KpLess             => chr("<"),
		I::KpGreater          => chr(">"),
		I::KpAmpersand        => chr("&"),
		I::KpDblAmpersand     => KeyCode::Unidentified,
		I::KpVerticalBar      => chr("|"),
		I::KpDblVerticalBar   => KeyCode::Unidentified,
		I::KpColon            => chr(":"),
		I::KpHash             => chr("#"),
		I::KpSpace            => chr(" "),
		I::KpAt               => chr("@"),
		I::KpExclam           => chr("!"),
		I::KpMemStore         => KeyCode::Unidentified,
		I::KpMemRecall        => KeyCode::Unidentified,
		I::KpMemClear         => KeyCode::Unidentified,
		I::KpMemAdd           => KeyCode::Unidentified,
		I::KpMemSubtract      => KeyCode::Unidentified,
		I::KpMemMultiply      => KeyCode::Unidentified,
		I::KpMemDivide        => KeyCode::Unidentified,
		I::KpPlusMinus        => chr("±"),
		I::KpClear            => KeyCode::Unidentified,
		I::KpClearEntry       => KeyCode::Unidentified,
		I::KpBinary           => KeyCode::Unidentified,
		I::KpOctal            => KeyCode::Unidentified,
		I::KpDecimal          => KeyCode::Unidentified,
		I::KpHexadecimal      => KeyCode::Unidentified,
		I::LCtrl              => KeyCode::Control,
		I::LShift             => KeyCode::Shift,
		I::LAlt               => KeyCode::Alt,
		I::LGui               => KeyCode::Meta,
		I::RCtrl              => KeyCode::Control,
		I::RShift             => KeyCode::Shift,
		I::RAlt               => KeyCode::Alt,
		I::RGui               => KeyCode::Meta,
		I::Mode               => KeyCode::ModeChange,
		I::AudioNext          => KeyCode::MediaTrackNext,
		I::AudioPrev          => KeyCode::MediaTrackPrevious,
		I::AudioStop          => KeyCode::MediaStop,
		I::AudioPlay          => KeyCode::MediaPlay,
		I::AudioMute          => KeyCode::AudioVolumeMute,
		I::MediaSelect        => KeyCode::MediaPlay,
		I::Www                => KeyCode::LaunchWebBrowser,
		I::Mail               => KeyCode::LaunchMail,
		I::Calculator         => KeyCode::LaunchApplication2,
		I::Computer           => KeyCode::LaunchApplication1,
		I::AcSearch           => KeyCode::BrowserSearch,
		I::AcHome             => KeyCode::BrowserHome,
		I::AcBack             => KeyCode::BrowserBack,
		I::AcForward          => KeyCode::BrowserForward,
		I::AcStop             => KeyCode::BrowserStop,
		I::AcRefresh          => KeyCode::BrowserRefresh,
		I::AcBookmarks        => KeyCode::BrowserFavorites,
		I::BrightnessDown     => KeyCode::BrightnessDown,
		I::BrightnessUp       => KeyCode::BrightnessUp,
		I::DisplaySwitch      => KeyCode::DisplaySwap,
		I::KbdIllumToggle     => KeyCode::Unidentified,
		I::KbdIllumDown       => KeyCode::Unidentified,
		I::KbdIllumUp         => KeyCode::Unidentified,
		I::Eject              => KeyCode::Eject,
		I::Sleep              => KeyCode::Standby,
	}
}
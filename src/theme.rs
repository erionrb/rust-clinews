use crossterm::style::Color::{Rgb, DarkBlue};
use termimad::{MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(DarkBlue);
    skin.italic.set_bg(Rgb {
        r: 28,
        g: 28,
        b: 28,
    });
    skin.bullet = StyledChar::from_fg_char(DarkBlue, '+');
    skin.set_headers_fg(DarkBlue);
    skin.quote_mark = StyledChar::from_fg_char(DarkBlue, '|');
    skin.quote_mark.set_fg(Rgb {
        r: 0,
        g: 255,
        b: 35,
    });
    skin.inline_code.set_fg(Rgb {
        r: 255,
        g: 0,
        b: 200,
    });
    skin.italic.set_fg(Rgb {
        r: 0,
        g: 255,
        b: 35,
    });

    skin
}

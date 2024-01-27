use core::{
    fmt,
    ops::{Deref, DerefMut},
    ptr::write_volatile,
};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ColorPalette(u8);

impl ColorPalette {
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self(foreground as u8 | ((background as u8) << 4))
    }
}

impl From<ColorPalette> for u8 {
    fn from(value: ColorPalette) -> Self {
        value.0
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScreenChar {
    pub ch: u8,
    pub palette: ColorPalette,
}

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 25;
type ScreenBuffer = [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT];

pub struct Screen<'a> {
    buff: &'a mut ScreenBuffer,
}

impl Screen<'_> {
    pub fn get() -> Self {
        Self {
            buff: unsafe { &mut *(0xB8000 as *mut ScreenBuffer) },
        }
    }
}

impl Deref for Screen<'_> {
    type Target = ScreenBuffer;

    fn deref(&self) -> &Self::Target {
        self.buff
    }
}

impl DerefMut for Screen<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buff
    }
}

pub struct ScreenController<'a> {
    screen: Screen<'a>,
    cursor_x: usize,
    cursor_y: usize,
    palette: ColorPalette,
}

impl<'a> ScreenController<'a> {
    pub fn new(screen: Screen<'a>, palette: ColorPalette) -> ScreenController<'a> {
        Self {
            screen,
            cursor_x: 0,
            cursor_y: 0,
            palette,
        }
    }

    fn scroll(&mut self) {
        for y in 0..SCREEN_HEIGHT - 1 {
            for x in 0..SCREEN_WIDTH {
                unsafe { write_volatile(&mut self.screen[y][x], self.screen[y + 1][x]) };
            }
        }

        for x in 0..SCREEN_WIDTH {
            unsafe {
                write_volatile(
                    &mut self.screen[SCREEN_HEIGHT - 1][x],
                    ScreenChar {
                        ch: ' ' as u8,
                        palette: self.palette,
                    },
                );
            }
        }
    }

    pub fn write_byte(&mut self, b: u8) {
        if b != '\n' as u8 {
            let sc = ScreenChar {
                ch: b,
                palette: self.palette,
            };

            unsafe { write_volatile(&mut self.screen[self.cursor_y][self.cursor_x], sc) };

            self.cursor_x += 1;
        } else {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        if self.cursor_x >= SCREEN_WIDTH {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        while self.cursor_y >= SCREEN_HEIGHT {
            self.scroll();
            self.cursor_x = 0;
            self.cursor_y -= 1;
        }
    }

    pub fn write_char(&mut self, ch: char) {
        self.write_byte(ch as u8);
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    pub fn set_palette(&mut self, palette: ColorPalette) {
        self.palette = palette;
    }

    pub fn repaint(&mut self) {
        for line in &mut *self.screen {
            for sc in line {
                unsafe { write_volatile(&mut sc.palette, self.palette) };
            }
        }
    }

    pub fn reset(&mut self) {
        for line in &mut *self.screen {
            for sc in line {
                sc.ch = ' ' as u8;
                sc.palette = self.palette;
            }
        }
    }
}

impl fmt::Write for ScreenController<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

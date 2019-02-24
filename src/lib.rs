extern crate rgb24;
#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde;

pub use rgb24::Rgb24;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    RangeError(u8),
}
pub type Result<T> = ::std::result::Result<T, Error>;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NormalColour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Grey,
}

impl NormalColour {
    pub fn code(self) -> u8 {
        use self::raw::normal::*;
        use self::NormalColour::*;
        match self {
            Black => BLACK,
            Red => RED,
            Green => GREEN,
            Yellow => YELLOW,
            Blue => BLUE,
            Magenta => MAGENTA,
            Cyan => CYAN,
            Grey => GREY,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrightColour {
    DarkGrey,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl BrightColour {
    pub fn code(self) -> u8 {
        use self::raw::bright::*;
        use self::BrightColour::*;
        match self {
            DarkGrey => DARK_GREY,
            Red => RED,
            Green => GREEN,
            Yellow => YELLOW,
            Blue => BLUE,
            Magenta => MAGENTA,
            Cyan => CYAN,
            White => WHITE,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RgbColour {
    red: u8,
    green: u8,
    blue: u8,
}
const RGB_START: u8 = 16;
const RGB_MAX_FIELD: u8 = 5;
const RGB_FIELD_RANGE: u8 = RGB_MAX_FIELD + 1;
const RGB_COUNT: u8 = RGB_FIELD_RANGE * RGB_FIELD_RANGE * RGB_FIELD_RANGE;
const RGB_END: u8 = RGB_START + RGB_COUNT - 1;

const GREY_SCALE_START: u8 = RGB_END + 1;
const GREY_SCALE_MAX_LEVEL: u8 = 23;

impl RgbColour {
    pub const fn red(self) -> u8 {
        self.red
    }
    pub const fn green(self) -> u8 {
        self.green
    }
    pub const fn blue(self) -> u8 {
        self.blue
    }
    pub fn new(red: u8, green: u8, blue: u8) -> Result<Self> {
        if red > RGB_MAX_FIELD {
            return Err(Error::RangeError(red));
        }
        if green > RGB_MAX_FIELD {
            return Err(Error::RangeError(green));
        }
        if blue > RGB_MAX_FIELD {
            return Err(Error::RangeError(blue));
        }
        Ok(Self { red, green, blue })
    }
    pub const fn code(self) -> u8 {
        RGB_START
            + (RGB_FIELD_RANGE * RGB_FIELD_RANGE) * self.red
            + RGB_FIELD_RANGE * self.green
            + self.blue
    }
    pub const fn from_rgb24_components(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: ((red as u32 * RGB_MAX_FIELD as u32) / 255 as u32) as u8,
            green: ((green as u32 * RGB_MAX_FIELD as u32) / 255 as u32) as u8,
            blue: ((blue as u32 * RGB_MAX_FIELD as u32) / 255 as u32) as u8,
        }
    }
    pub const fn from_rgb24(rgb24: Rgb24) -> Self {
        Self::from_rgb24_components(rgb24.r, rgb24.g, rgb24.b)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GreyScaleColour(u8);

impl GreyScaleColour {
    pub const fn level(self) -> u8 {
        self.0
    }
    pub fn new(level: u8) -> Result<Self> {
        if level > GREY_SCALE_MAX_LEVEL {
            return Err(Error::RangeError(level));
        }
        Ok(GreyScaleColour(level))
    }
    pub const fn code(self) -> u8 {
        GREY_SCALE_START + self.0
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColourVariant {
    Normal(NormalColour),
    Bright(BrightColour),
    Rgb(RgbColour),
    GreyScale(GreyScaleColour),
}

impl ColourVariant {
    pub fn code(self) -> u8 {
        use ColourVariant::*;
        match self {
            Normal(c) => c.code(),
            Bright(c) => c.code(),
            Rgb(c) => c.code(),
            GreyScale(c) => c.code(),
        }
    }
    pub fn colour(self) -> Colour {
        Colour(self.code())
    }
    pub const fn from_rgb24_components(red: u8, green: u8, blue: u8) -> Self {
        ColourVariant::Rgb(RgbColour::from_rgb24_components(red, green, blue))
    }
    pub const fn from_rgb24(rgb24: Rgb24) -> Self {
        ColourVariant::Rgb(RgbColour::from_rgb24(rgb24))
    }
}

pub const fn encode_rgb24_components(red: u8, green: u8, blue: u8) -> u8 {
    RgbColour::from_rgb24_components(red, green, blue).code()
}

pub const fn encode_rgb24(rgb24: Rgb24) -> u8 {
    RgbColour::from_rgb24(rgb24).code()
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Colour(u8);

impl Colour {
    pub fn grey_scale(level: u8) -> Result<Self> {
        Ok(Colour(GreyScaleColour::new(level)?.code()))
    }
    pub fn rgb(red: u8, green: u8, blue: u8) -> Result<Self> {
        Ok(Colour(RgbColour::new(red, green, blue)?.code()))
    }
    pub fn normal(normal_colour: NormalColour) -> Self {
        Colour(normal_colour.code())
    }
    pub fn bright(bright_colour: BrightColour) -> Self {
        Colour(bright_colour.code())
    }
    pub const fn code(self) -> u8 {
        self.0
    }
    pub const fn from_code(code: u8) -> Self {
        Colour(code)
    }
    pub fn typ(self) -> ColourVariant {
        use self::raw::*;
        match self.0 {
            normal::BLACK => ColourVariant::Normal(NormalColour::Black),
            normal::RED => ColourVariant::Normal(NormalColour::Red),
            normal::GREEN => ColourVariant::Normal(NormalColour::Green),
            normal::YELLOW => ColourVariant::Normal(NormalColour::Yellow),
            normal::BLUE => ColourVariant::Normal(NormalColour::Blue),
            normal::MAGENTA => ColourVariant::Normal(NormalColour::Magenta),
            normal::CYAN => ColourVariant::Normal(NormalColour::Cyan),
            normal::GREY => ColourVariant::Normal(NormalColour::Grey),
            bright::DARK_GREY => ColourVariant::Bright(BrightColour::DarkGrey),
            bright::RED => ColourVariant::Bright(BrightColour::Red),
            bright::GREEN => ColourVariant::Bright(BrightColour::Green),
            bright::YELLOW => ColourVariant::Bright(BrightColour::Yellow),
            bright::BLUE => ColourVariant::Bright(BrightColour::Blue),
            bright::MAGENTA => ColourVariant::Bright(BrightColour::Magenta),
            bright::CYAN => ColourVariant::Bright(BrightColour::Cyan),
            bright::WHITE => ColourVariant::Bright(BrightColour::White),
            RGB_START...RGB_END => {
                let zero_based = self.0 - RGB_START;
                let blue = zero_based % RGB_FIELD_RANGE;
                let zero_based = zero_based / RGB_FIELD_RANGE;
                let green = zero_based % RGB_FIELD_RANGE;
                let zero_based = zero_based / RGB_FIELD_RANGE;
                let red = zero_based % RGB_FIELD_RANGE;
                ColourVariant::Rgb(RgbColour { red, green, blue })
            }
            GREY_SCALE_START...255 => {
                let zero_based = self.0 - GREY_SCALE_START;
                ColourVariant::GreyScale(GreyScaleColour(zero_based))
            }
            _ => unreachable!(),
        }
    }
    pub fn all() -> ColourIter {
        AllColours.into_iter()
    }
    pub const fn from_rgb24_components(red: u8, green: u8, blue: u8) -> Self {
        Colour(encode_rgb24_components(red, green, blue))
    }
    pub const fn from_rgb24(rgb24: Rgb24) -> Self {
        Colour(encode_rgb24(rgb24))
    }
}

impl From<ColourVariant> for Colour {
    fn from(t: ColourVariant) -> Self {
        t.colour()
    }
}

impl From<Colour> for ColourVariant {
    fn from(c: Colour) -> Self {
        c.typ()
    }
}

impl From<Colour> for u8 {
    fn from(c: Colour) -> Self {
        c.code()
    }
}

impl From<u8> for Colour {
    fn from(u: u8) -> Self {
        Colour(u)
    }
}

pub struct ColourIter(::std::ops::Range<u16>);

impl Iterator for ColourIter {
    type Item = Colour;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| Colour(i as u8))
    }
}

pub struct AllColours;

impl IntoIterator for AllColours {
    type Item = Colour;
    type IntoIter = ColourIter;
    fn into_iter(self) -> Self::IntoIter {
        ColourIter(0..256)
    }
}

pub const NUM_NORMAL_COLOURS: usize = 8;
pub const NUM_BRIGHT_COLOURS: usize = 8;
pub const NUM_RGB_COLOURS: usize = RGB_COUNT as usize;
pub const NUM_GREY_SCALE_COLOURS: usize = GREY_SCALE_MAX_LEVEL as usize + 1;
pub const NUM_RGB_COLOURS_PER_CHANNEL: usize = RGB_FIELD_RANGE as usize;
pub const MAX_RGB_CHANNEL: usize = RGB_MAX_FIELD as usize;
pub const MAX_GREY_SCALE: usize = GREY_SCALE_MAX_LEVEL as usize;

pub struct NormalColours;

impl IntoIterator for NormalColours {
    type Item = Colour;
    type IntoIter = ColourIter;
    fn into_iter(self) -> Self::IntoIter {
        ColourIter(0..(NUM_NORMAL_COLOURS as u16))
    }
}

pub struct BrightColours;

impl IntoIterator for BrightColours {
    type Item = Colour;
    type IntoIter = ColourIter;
    fn into_iter(self) -> Self::IntoIter {
        ColourIter((NUM_NORMAL_COLOURS as u16)..((NUM_NORMAL_COLOURS + NUM_BRIGHT_COLOURS) as u16))
    }
}

pub struct RgbColours;

impl IntoIterator for RgbColours {
    type Item = Colour;
    type IntoIter = ColourIter;
    fn into_iter(self) -> Self::IntoIter {
        ColourIter((RGB_START as u16)..(RGB_START as u16 + NUM_RGB_COLOURS as u16))
    }
}

pub struct GreyScaleColours;

impl IntoIterator for GreyScaleColours {
    type Item = Colour;
    type IntoIter = ColourIter;
    fn into_iter(self) -> Self::IntoIter {
        ColourIter(
            (GREY_SCALE_START as u16)..(GREY_SCALE_START as u16 + NUM_GREY_SCALE_COLOURS as u16),
        )
    }
}

mod raw {
    pub mod normal {
        pub const BLACK: u8 = 0;
        pub const RED: u8 = 1;
        pub const GREEN: u8 = 2;
        pub const YELLOW: u8 = 3;
        pub const BLUE: u8 = 4;
        pub const MAGENTA: u8 = 5;
        pub const CYAN: u8 = 6;
        pub const GREY: u8 = 7;
    }
    pub mod bright {
        pub const DARK_GREY: u8 = 8;
        pub const RED: u8 = 9;
        pub const GREEN: u8 = 10;
        pub const YELLOW: u8 = 11;
        pub const BLUE: u8 = 12;
        pub const MAGENTA: u8 = 13;
        pub const CYAN: u8 = 14;
        pub const WHITE: u8 = 15;
    }
}

pub mod colours {

    use raw;
    use Colour;

    pub const BLACK: Colour = Colour(self::raw::normal::BLACK);
    pub const RED: Colour = Colour(self::raw::normal::RED);
    pub const GREEN: Colour = Colour(self::raw::normal::GREEN);
    pub const YELLOW: Colour = Colour(self::raw::normal::YELLOW);
    pub const BLUE: Colour = Colour(self::raw::normal::BLUE);
    pub const MAGENTA: Colour = Colour(self::raw::normal::MAGENTA);
    pub const CYAN: Colour = Colour(self::raw::normal::CYAN);
    pub const GREY: Colour = Colour(self::raw::normal::GREY);

    pub const DARK_GREY: Colour = Colour(self::raw::bright::DARK_GREY);
    pub const BRIGHT_RED: Colour = Colour(self::raw::bright::RED);
    pub const BRIGHT_GREEN: Colour = Colour(self::raw::bright::GREEN);
    pub const BRIGHT_YELLOW: Colour = Colour(self::raw::bright::YELLOW);
    pub const BRIGHT_BLUE: Colour = Colour(self::raw::bright::BLUE);
    pub const BRIGHT_MAGENTA: Colour = Colour(self::raw::bright::MAGENTA);
    pub const BRIGHT_CYAN: Colour = Colour(self::raw::bright::CYAN);
    pub const WHITE: Colour = Colour(self::raw::bright::WHITE);
}

#[cfg(test)]
mod tests {
    use {
        AllColours, NUM_BRIGHT_COLOURS, NUM_GREY_SCALE_COLOURS, NUM_NORMAL_COLOURS, NUM_RGB_COLOURS,
    };

    #[test]
    fn from_colour() {
        for c in AllColours {
            c.typ();
        }
    }

    #[test]
    fn reflexive() {
        for orig_c in AllColours {
            let orig_t = orig_c.typ();
            let new_c = orig_t.colour();
            let new_t = new_c.typ();

            assert_eq!(orig_t, new_t);
            assert_eq!(orig_c, new_c);
        }
    }

    #[test]
    fn num_colours() {
        const NUM_COLOURS: usize = 256;
        assert_eq!(
            NUM_NORMAL_COLOURS + NUM_BRIGHT_COLOURS + NUM_RGB_COLOURS + NUM_GREY_SCALE_COLOURS,
            NUM_COLOURS
        );
    }
}

use crate::glyph::GlyphAttr;
use crate::x11_wrapper as x11;
use crate::Result;
use std::ffi::CString;
use std::os::raw::c_int;

/*
 * Printable characters in ASCII, used to estimate the advance width
 * of single wide characters.
 */
static ASCII_PRINTABLE: &[u8; 95] = b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

pub struct Font {
    xft: Xft,
    height: usize,
    width: usize,
    font: x11::XftFont,
    bfont: x11::XftFont,
    ifont: x11::XftFont,
    ibfont: x11::XftFont,
}

impl Font {
    pub fn new(dpy: x11::Display, scr: c_int, xft: Xft) -> Result<Self> {
        let pattern = x11::XftNameParse(&xft.serialize())?;

        let matched = x11::XftFontMatch(dpy, scr, pattern)?;
        let font = x11::XftFontOpenPattern(dpy, matched)?;
        x11::FcPatternDestroy(matched);
        let extents = x11::XftTextExtentsUtf8(dpy, font, ASCII_PRINTABLE);

        let height = x11::font_ascent(font) + x11::font_descent(font);
        let len = ASCII_PRINTABLE.len();

        // Divceil (round the width up).
        let width = (extents.xOff as usize + (len - 1)) / len;

        let slant = CString::new("slant").unwrap();
        let weight = CString::new("weight").unwrap();

        x11::FcPatternDel(pattern, &slant);
        x11::FcPatternAddInteger(pattern, &slant, x11::FC_SLANT_ITALIC);
        let matched = x11::XftFontMatch(dpy, scr, pattern)?;
        let ifont = x11::XftFontOpenPattern(dpy, matched)?;
        x11::FcPatternDestroy(matched);

        x11::FcPatternDel(pattern, &weight);
        x11::FcPatternAddInteger(pattern, &weight, x11::FC_WEIGHT_BOLD);
        let matched = x11::XftFontMatch(dpy, scr, pattern)?;
        let ibfont = x11::XftFontOpenPattern(dpy, matched)?;
        x11::FcPatternDestroy(matched);

        x11::FcPatternDel(pattern, &slant);
        x11::FcPatternAddInteger(pattern, &slant, x11::FC_SLANT_ROMAN);
        let matched = x11::XftFontMatch(dpy, scr, pattern)?;
        let bfont = x11::XftFontOpenPattern(dpy, matched)?;
        x11::FcPatternDestroy(matched);

        x11::FcPatternDestroy(pattern);
        Ok(Self {
            xft,
            width,
            height,
            font,
            bfont,
            ifont,
            ibfont,
        })
    }

    pub fn get(&self, attr: GlyphAttr) -> x11::XftFont {
        if attr.contains(GlyphAttr::BOLD | GlyphAttr::ITALIC) {
            return self.ibfont;
        }
        if attr.contains(GlyphAttr::BOLD) {
            return self.bfont;
        }
        if attr.contains(GlyphAttr::ITALIC) {
            return self.ifont;
        }
        self.font
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn ascent(&self) -> usize {
        x11::font_ascent(self.font)
    }

    pub fn get_resized(&self, dpy: x11::Display, scr: c_int, increment: i32) -> Result<Self> {
        let new_unsafe_size = self.xft.size as i32 + increment;
        let new_size = if new_unsafe_size > 0 {
            new_unsafe_size as usize
        } else {
            self.xft.size
        };

        Self::new(
            dpy,
            scr,
            Xft {
                name: self.xft.name.clone(),
                size: new_size,
            },
        )
    }
}

/// Utility for xft strings like droid:regular:size=12
pub struct Xft {
    name: String,
    pub size: usize,
}

impl Xft {
    pub fn new(name: &str) -> Self {
        Self {
            name: Self::remove_size(name),
            size: Self::parse_size(name),
        }
    }

    pub fn serialize(&self) -> String {
        self.name.clone() + ":size=" + &self.size.to_string()
    }

    /// Parse the xft string and gets the size, or a default of 12 if unspecified
    fn parse_size(name: &str) -> usize {
        let mut size_found = false;
        name.split(&['=', ':'])
            .filter(|section| {
                let is_it_size = size_found;
                if section.contains("size") {
                    size_found = true;
                }
                is_it_size
            })
            .next()
            .and_then(|size| size.parse::<usize>().ok())
            .unwrap_or(12)
    }

    /// Removes the :size=.. notation
    fn remove_size(name: &str) -> String {
        name.split(':')
            .filter(|section| !section.contains("size="))
            .collect::<Vec<_>>()
            .join(":")
    }
}

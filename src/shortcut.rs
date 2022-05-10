use crate::pty::Pty;
use crate::term::Term;
use crate::win::Win;
use std::os::raw::*;
use x11::keysym::*;
use x11::xlib::*;

#[derive(Clone, Copy)]
pub enum Function {
    Paste,
    ZoomIn,
    ZoomOut,
}

impl Function {
    pub fn execute(&self, win: &mut Win, term: &mut Term, pty: &mut Pty) {
        match self {
            Function::Paste => win.selection_paste(),
            Function::ZoomIn => win.zoom(term, pty, 1),
            Function::ZoomOut => win.zoom(term, pty, -1),
        }
    }
}

struct Shortcut {
    k: c_uint,
    mask: c_uint,
    function: Function,
}

macro_rules! make_shortcuts {
    {
        $({ $mask:expr, $k:expr, $function:path },)*
    } => {
        &[
            $(Shortcut {
                k: $k,
                mask: $mask,
                function: $function,
            },)*
        ]
    }
}

const SHORTCUTS: &[Shortcut] = make_shortcuts! {
    /* mask                  keysym          function */
    { ShiftMask,             XK_Insert,      Function::Paste },
    { ControlMask,           XK_Up,     Function::ZoomIn },
    { ControlMask,           XK_Down,     Function::ZoomOut },
};

pub fn find_shortcut(k: KeySym, state: c_uint) -> Option<Function> {
    let k = k as c_uint;
    if k & 0xFFFF < 0xFD00 {
        return None;
    }

    for shortcut in SHORTCUTS {
        if k == shortcut.k && state & shortcut.mask != 0 {
            return Some(shortcut.function);
        }
    }
    None
}

use core::ascii::Char;

use crate::arch::i686::kbd;
use crate::arch::i686::kbd::Key::{self, KpStar};
use crate::drivers::input;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

type CharBuffer = [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT];

#[repr(C)]
pub struct InputBuffer {
    buffer: CharBuffer,
    row_pos: usize,
    col_pos: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputAction {
    AddChar(Char),
    DelChar,
    Submit,
    Cancel,
}

pub struct Input {
    buffer: InputBuffer,
    cur_action: Option<InputAction>,
    cur_keypress: Key,
    is_shift: bool,
    is_ctrl: bool,
    is_alt: bool,
    caps_lock: bool,
    num_lock: bool,
    scrl_lock: bool,
}

impl Input {

    pub fn is_shift(&mut self, state: bool) { self.is_shift = state; }
    pub fn is_ctrl(&mut self, state: bool) { self.is_ctrl = state; }
    pub fn is_alt(&mut self, state: bool) { self.is_alt = state; }
    pub fn is_capslk(&mut self, state: bool) { self.caps_lock = state; }
    pub fn is_numlk(&mut self, state: bool) { self.num_lock = state; }
    pub fn is_scrllk(&mut self, state: bool) { self.scrl_lock = state; }

    pub fn update_action(&mut self) {
        if (self.is_shift | self.caps_lock) {

        }
    }

}

fn map_key_to_char(key: Key) -> Option<Char> {
    match key {
        //alphabets
        Key::A => Some(Char::SmallA),
        Key::B => Some(Char::SmallB),
        Key::C => Some(Char::SmallC),
        Key::D => Some(Char::SmallD),
        Key::E => Some(Char::SmallE),
        Key::F => Some(Char::SmallF),
        Key::G => Some(Char::SmallG),
        Key::H => Some(Char::SmallH),
        Key::I => Some(Char::SmallI),
        Key::J => Some(Char::SmallJ),
        Key::K => Some(Char::SmallK),
        Key::L => Some(Char::SmallL),
        Key::M => Some(Char::SmallM),
        Key::N => Some(Char::SmallN),
        Key::O => Some(Char::SmallO),
        Key::P => Some(Char::SmallP),
        Key::Q => Some(Char::SmallQ),
        Key::R => Some(Char::SmallR),
        Key::S => Some(Char::SmallS),
        Key::T => Some(Char::SmallT),
        Key::U => Some(Char::SmallU),
        Key::V => Some(Char::SmallV),
        Key::W => Some(Char::SmallW),
        Key::X => Some(Char::SmallX),
        Key::Y => Some(Char::SmallY),
        Key::Z => Some(Char::SmallZ),

        //numbers
        Key::Num0 => Some(Char::Digit0),
        Key::Num1 => Some(Char::Digit1),
        Key::Num2 => Some(Char::Digit2),
        Key::Num3 => Some(Char::Digit3),
        Key::Num4 => Some(Char::Digit4),
        Key::Num5 => Some(Char::Digit5),
        Key::Num6 => Some(Char::Digit6),
        Key::Num7 => Some(Char::Digit7),
        Key::Num8 => Some(Char::Digit8),
        Key::Num9 => Some(Char::Digit9),

        //symbols
        Key::Minus => Some(Char::HyphenMinus),
        Key::Equal => Some(Char::EqualsSign),
        Key::OpSqBk => Some(Char::LeftSquareBracket),
        Key::ClSqBk => Some(Char::RightSquareBracket),
        Key::Smcln => Some(Char::Semicolon),
        Key::Quote => Some(Char::Apostrophe),
        Key::BkTk => Some(Char::GraveAccent),
        Key::FSlash => Some(Char::Solidus),
        Key::BSlash => Some(Char::ReverseSolidus),
        Key::Comma => Some(Char::Comma),
        Key::Period => Some(Char::FullStop),
        
        //space
        Key::Space => Some(Char::Space),

        //keypad (change later for only number lock)
        Key::KpStar => Some(Char::Asterisk),
        Key::KpMinus => Some(Char::HyphenMinus),
        Key::KpPlus => Some(Char::PlusSign),
        Key::KpPeriod => Some(Char::FullStop),
        Key::Kp0 => Some(Char::Digit0),
        Key::Kp1 => Some(Char::Digit1),
        Key::Kp2 => Some(Char::Digit2),
        Key::Kp3 => Some(Char::Digit3),
        Key::Kp4 => Some(Char::Digit4),
        Key::Kp5 => Some(Char::Digit5),
        Key::Kp6 => Some(Char::Digit6),
        Key::Kp7 => Some(Char::Digit7),
        Key::Kp8 => Some(Char::Digit8),
        Key::Kp9 => Some(Char::Digit9),

        //unmapped keys
        //_ => None,
    }
}

fn map_shift(input: Char) -> Option<Char> {
    match input {
        //alphabets
        Char::SmallA => Some(Char::CapitalA),
        Char::SmallB => Some(Char::CapitalB),
        Char::SmallC => Some(Char::CapitalC),
        Char::SmallD => Some(Char::CapitalD),
        Char::SmallE => Some(Char::CapitalE),
        Char::SmallF => Some(Char::CapitalF),
        Char::SmallG => Some(Char::CapitalG),
        Char::SmallH => Some(Char::CapitalH),
        Char::SmallI => Some(Char::CapitalI),
        Char::SmallJ => Some(Char::CapitalJ),
        Char::SmallK => Some(Char::CapitalK),
        Char::SmallL => Some(Char::CapitalL),
        Char::SmallM => Some(Char::CapitalM),
        Char::SmallN => Some(Char::CapitalN),
        Char::SmallO => Some(Char::CapitalO),
        Char::SmallP => Some(Char::CapitalP),
        Char::SmallQ => Some(Char::CapitalQ),
        Char::SmallR => Some(Char::CapitalR),
        Char::SmallS => Some(Char::CapitalS),
        Char::SmallT => Some(Char::CapitalT),
        Char::SmallU => Some(Char::CapitalU),
        Char::SmallV => Some(Char::CapitalV),
        Char::SmallW => Some(Char::CapitalW),
        Char::SmallX => Some(Char::CapitalX),
        Char::SmallY => Some(Char::CapitalY),
        Char::SmallZ => Some(Char::CapitalZ),

        //numbers
        Char::Digit1 => Some(Char::ExclamationMark),
        Char::Digit2 => Some(Char::QuotationMark),
        Char::Digit3 => Some(Char::NumberSign),
        Char::Digit4 => Some(Char::DollarSign),
        Char::Digit5 => Some(Char::PercentSign),
        Char::Digit6 => Some(Char::CircumflexAccent),
        Char::Digit7 => Some(Char::Ampersand),
        Char::Digit8 => Some(Char::Asterisk),
        Char::Digit9 => Some(Char::LeftParenthesis),
        Char::Digit0 => Some(Char::RightParenthesis),

        //symbols
        Char::HyphenMinus => Some(Char::LowLine),
        Char::EqualsSign => Some(Char::PlusSign),
        Char::LeftSquareBracket => Some(Char::LeftCurlyBracket),
        Char::RightSquareBracket => Some(Char::RightCurlyBracket),
        Char::Semicolon => Some(Char::Colon),
        Char::Apostrophe => Some(Char::QuotationMark),
        Char::GraveAccent => Some(Char::Tilde),
        Char::Solidus => Some(Char::QuestionMark),
        Char::ReverseSolidus => Some(Char::VerticalLine),
        Char::Comma => Some(Char::LessThanSign),
        Char::FullStop => Some(Char::GreaterThanSign),
        
        //space
        Char::Space => Some(Char::Space),

        _ => None,
    }
}

fn map_caps(input: Char) -> Option<Char> {
    match input {
        //alphabets
        Char::SmallA => Some(Char::CapitalA),
        Char::SmallB => Some(Char::CapitalB),
        Char::SmallC => Some(Char::CapitalC),
        Char::SmallD => Some(Char::CapitalD),
        Char::SmallE => Some(Char::CapitalE),
        Char::SmallF => Some(Char::CapitalF),
        Char::SmallG => Some(Char::CapitalG),
        Char::SmallH => Some(Char::CapitalH),
        Char::SmallI => Some(Char::CapitalI),
        Char::SmallJ => Some(Char::CapitalJ),
        Char::SmallK => Some(Char::CapitalK),
        Char::SmallL => Some(Char::CapitalL),
        Char::SmallM => Some(Char::CapitalM),
        Char::SmallN => Some(Char::CapitalN),
        Char::SmallO => Some(Char::CapitalO),
        Char::SmallP => Some(Char::CapitalP),
        Char::SmallQ => Some(Char::CapitalQ),
        Char::SmallR => Some(Char::CapitalR),
        Char::SmallS => Some(Char::CapitalS),
        Char::SmallT => Some(Char::CapitalT),
        Char::SmallU => Some(Char::CapitalU),
        Char::SmallV => Some(Char::CapitalV),
        Char::SmallW => Some(Char::CapitalW),
        Char::SmallX => Some(Char::CapitalX),
        Char::SmallY => Some(Char::CapitalY),
        Char::SmallZ => Some(Char::CapitalZ),
    }
}

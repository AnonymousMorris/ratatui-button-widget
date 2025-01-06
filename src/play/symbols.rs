
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PlaySet {
    pub top_corner: &'static str,
    pub bottom_corner: &'static str,
    pub right_corner: &'static str,
    pub center: &'static str,
    pub single_char_symbol: &'static str,
}

impl Default for PlaySet {
    fn default() -> PlaySet {
        PLAIN
    }
}

pub const PLAIN: PlaySet = PlaySet {
    top_corner: "🬿",
    bottom_corner: "🭚",
    right_corner: "🭬",
    center: "█",
    single_char_symbol: "▶",
};

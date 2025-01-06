use ratatui::prelude::*;
use crate::play::symbols::PlaySet;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Play {
    pub(crate) style: Style,
    pub(crate) char_set: PlaySet,
}
impl Play {
    pub fn new() -> Play {
        Play {
            style: Style::default(),
            char_set: PlaySet::default(),
        }
    }
}

// use ratatui::prelude::*;
// use ratatui::{buffer::Buffer, widgets::Widget};
//
// use crate::hex::Hexagon;
//
//
// enum PlayState {
//     Play,
//     Pause,
// }
//
// enum PlayButtonState {
//     Selected(PlayState),
//     Normal(PlayState),
//     Pressed(PlayState),
// }
//
// struct PlayButton <W: Widget> {
//     hexagon: Hexagon,
//     state: PlayButtonState,
//     content: Option<W>
// }
//
// impl PlayButton {
//     fn new() -> Self {
//         PlayButton {
//             hexagon: Hexagon::new(),
//             state: PlayButtonState::Normal(PlayState::Play),
//             content: None,
//         }
//     }
// }
//
// impl StatefulWidget for PlayButton {
//     type State = PlayState;
//
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         inner = self.hexagon.inner(area);
//     }
// }

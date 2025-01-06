use std::cmp::min;
use ratatui::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PlaySet {
    pub top_corner: &'static str,
    pub bottom_corner: &'static str,
    pub right_corner: &'static str,
    pub center: &'static str,
    pub single_char_symbol: &'static str,
}

pub const PLAIN: PlaySet = PlaySet {
    top_corner: "🬿",
    bottom_corner: "🭚",
    right_corner: "🭬",
    center: "█",
    single_char_symbol: "▶",
};

struct Play {
    style: Style,
    char_set: PlaySet,
}


impl Widget for Play {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.render_play(area, buf);
    }
}

impl Play {
    fn new() -> Play {
        Play {
            style: Style::default(),
            char_set: PLAIN,
        }
    }

    fn render_play(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if area.height == 1 {
            buf[(area.left(), area.top())]
                .set_symbol(self.char_set.single_char_symbol)
                .set_style(self.style);
        }
        else {
            let max_inner_width = min(area.width, (area.height + 1) / 2);
            let max_inner_height = min(area.height, max_inner_width * 2 - 1);
            
            let inner_size = Size::new(max_inner_width, max_inner_height);
            let inner_x_offset = (area.width - inner_size.width) / 2;
            let inner_y_offset = (area.height - inner_size.height) / 2;
            let inner_pos = Position::new(area.left() + inner_x_offset, area.top() + inner_y_offset);

            let inner_area = Rect::from((inner_pos, inner_size));
            self.render_edges(inner_area, buf);
            self.render_center(inner_area, buf);
        }
    }

    fn render_edges(&mut self, area: Rect, buf: &mut Buffer) {
        let mid_y = area.height / 2;
        // draw top half
        for x_offset in 0..area.width{
            let x = area.left() + x_offset;
            let y = area.top() + x_offset;

                buf[(x, y)]
                    .set_symbol(self.char_set.top_corner)
                    .set_style(self.style);
        }
        // draw bottom half
        for x_offset in 0..area.width{
            let x = area.left() + x_offset;
            let y = area.bottom() - x_offset - 1;
                buf[(x, y)]
                    .set_symbol(self.char_set.bottom_corner)
                    .set_style(self.style);
        }
        // draw middle line
        if area.height % 2 == 1 {
            let x = area.right();
            let y = area.top() + mid_y;
            buf[(x, y)]
                .set_symbol(self.char_set.right_corner)
                .set_style(self.style);
        }
    }

    fn render_center(&mut self, area: Rect, buf: &mut Buffer) {
        for x_offset in 0..area.width {
            let x = area.left() + x_offset;
            for y in (area.top() + x_offset + 1) .. (area.bottom() - x_offset - 1) {
                buf[(x, y)]
                    .set_symbol(self.char_set.center)
                    .set_style(self.style);
            }
        }
    }
    
}

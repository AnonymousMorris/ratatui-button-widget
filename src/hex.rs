use std::cmp::min;
use ratatui::prelude::*;
use ratatui::widgets::Widget;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct HexSet {
    pub top_left_corner: &'static str,
    pub top_right_corner: &'static str,
    pub bottom_left_corner: &'static str,
    pub bottom_right_corner: &'static str,
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub vertical_left: &'static str,
    pub vertical_right: &'static str,
    pub horizontal_top: &'static str,
    pub horizontal_bottom: &'static str,
}

pub const PLAIN: HexSet = HexSet {
    top_left_corner: "⸝",
    top_right_corner: "⸜",
    bottom_left_corner: "⸌",
    bottom_right_corner: "⸍",
    top_left: "╱",
    top_right: "╲",
    bottom_left: "╲",
    bottom_right: "╱",
    vertical_left: "▏",
    vertical_right: "▕",
    horizontal_top: "─",
    horizontal_bottom: "─",
};

#[derive(Clone)]
pub struct Hexagon {
    // /// Visibile borders
    // borders: Borders,
    /// Border style
    border_style: Style,
    border_set: HexSet,
    // /// Widget style
    // style: Style,
    // /// Block padding
    // padding: Padding
}

impl Hexagon {
    pub const fn new() -> Self {
        Self {
            border_style: Style::new(),
            border_set: PLAIN,
            // style: Style::new(),
            // padding: Padding::Zero,
        }
    }

    // #[must_use = "method moves the value of self and returns the modified value"]
    // pub fn border_style<S: Into<Style>>(mut self, style: S) -> Self {
    //     self.border_style = style.into();
    //     self
    // }
    //
    // #[must_use = "method moves the value of self and returns the modified value"]
    // pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
    //     // self.style = style.into();
    //     self
    // }

    // #[must_use = "method moves the value of self and returns the modified value"]
    // pub const fn padding(mut self, padding: Padding) -> Self {
    //     self.padding = padding;
    //     self
    // }

    // pub fn inner(&self, area: Rect) -> Rect {
    //     let mut inner = area;
    // }
}

impl Widget for Hexagon {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.render_borders(area, buf);
    }
}

impl Hexagon {
    fn render_borders(&mut self, area: Rect, buf: &mut Buffer) {
        // if area less than 3 lines, there would not be space for anything inside
        if area.width >= 3 && area.height >= 3 {
            let slanted_lines = min(area.width / 2, area.height / 2);
            let corner_size = Size::new(slanted_lines, slanted_lines);

            let top_left_pos = Position::new(area.left(), area.top());
            let top_right_pos = Position::new(area.right() - slanted_lines, area.top());
            let bottom_left_pos = Position::new(area.left(), area.bottom() - slanted_lines);
            let bottom_right_pos = Position::new(area.right() - slanted_lines, area.bottom() - slanted_lines);

            let top_left = Rect::from((top_left_pos, corner_size));
            let top_right = Rect::from((top_right_pos, corner_size));
            let bottom_left = Rect::from((bottom_left_pos, corner_size));
            let bottom_right = Rect::from((bottom_right_pos, corner_size));


            // render slanted corners
            self.render_upper_left(top_left, buf);
            self.render_upper_right(top_right, buf);
            self.render_lower_left(bottom_left, buf);
            self.render_lower_right(bottom_right, buf);

            // render vertical lines
            let vertical_size = Size::new(1, area.height - (2 * slanted_lines));
            let vertical_left_pos = Position::new(area.left(), area.top() + slanted_lines);
            let vertical_right_pos = Position::new(area.right() - 1, area.top() + slanted_lines);

            let vertical_left = Rect::from((vertical_left_pos, vertical_size));
            let vertical_right = Rect::from((vertical_right_pos, vertical_size));
            self.render_vertical_left(vertical_left, buf);
            self.render_vertical_right(vertical_right, buf);

            // render horizontal lines
            let horizontal_size = Size::new(area.width - (2 * slanted_lines), 1);
            let horizontal_top_pos = Position::new(area.left() + slanted_lines, area.top());
            let horizontal_bottom_pos = Position::new(area.left() + slanted_lines, area.bottom() - 1);

            let horizontal_top = Rect::from((horizontal_top_pos, horizontal_size));
            let horizontal_bottom = Rect::from((horizontal_bottom_pos, horizontal_size));
            self.render_horizontal_top(horizontal_top, buf);
            self.render_horizontal_bottom(horizontal_bottom, buf);
        }
    }

    // area passed in is the top left corner where the slanted lines are to be drawn
    fn render_upper_left(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.width == area.height);
        buf[(area.right() - 1, area.top())]
            .set_symbol(self.border_set.top_left_corner)
            .set_style(self.border_style);
        for i in 1..area.width {
            let x = area.right() - i - 1;
            let y = area.top() + i;

            buf[(x, y)]
                .set_symbol(self.border_set.top_left)
                .set_style(self.border_style);
        }
    }

    // area passed in is the top right corner where the slanted lines are to be drawn
    fn render_upper_right(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.width == area.height);
        buf[(area.left(), area.top())]
            .set_symbol(self.border_set.top_right_corner)
            .set_style(self.border_style);
        for i in 1..area.width {
            let x = area.left() + i;
            let y = area.top() + i;

            buf[(x, y)]
                .set_symbol(self.border_set.top_right)
                .set_style(self.border_style);
        }
    }

    fn render_lower_left(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.width == area.height);
        buf[(area.right() - 1, area.bottom() - 1)]
            .set_symbol(self.border_set.bottom_left_corner)
            .set_style(self.border_style);
        for i in 1..area.width {
            let x = area.right() - i - 1;
            let y = area.bottom() - i - 1;

            buf[(x, y)]
                .set_symbol(self.border_set.bottom_left)
                .set_style(self.border_style);
        }
    }

    fn render_lower_right(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.width == area.height);
        buf[(area.left(), area.bottom() - 1)]
            .set_symbol(self.border_set.bottom_right_corner)
            .set_style(self.border_style);
        for i in 1..area.width {
            let x = area.left() + i;
            let y = area.bottom() - i - 1;

            buf[(x, y)]
                .set_symbol(self.border_set.bottom_right)
                .set_style(self.border_style);
        }
    }

    fn render_vertical_left(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.width == 1);
        for y in area.top()..area.bottom() {
            buf[(area.left(), y)]
                .set_symbol(self.border_set.vertical_left)
                .set_style(self.border_style);
        }
    }
    
    fn render_vertical_right(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.width == 1);
        for y in area.top()..area.bottom() {
            buf[(area.right() - 1, y)]
                .set_symbol(self.border_set.vertical_right)
                .set_style(self.border_style);
        }
    }

    fn render_horizontal_top(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.height == 1);
        for x in area.left()..area.right() {
            buf[(x, area.top())]
                .set_symbol(self.border_set.horizontal_top)
                .set_style(self.border_style);
        }
    }
    
    fn render_horizontal_bottom(&mut self, area: Rect, buf: &mut Buffer) {
        assert!(area.height == 1);
        for x in area.left()..area.right() {
            buf[(x, area.bottom() - 1)]
                .set_symbol(self.border_set.horizontal_bottom)
                .set_style(self.border_style);
        }
    }

    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner = area;
        inner.x = inner.x.saturating_add(1).min(inner.right());
        inner.width = inner.width.saturating_sub(2);
        inner.y = inner.y.saturating_add(1).min(inner.bottom());
        inner.height = inner.height.saturating_sub(2);
        inner
    }
}


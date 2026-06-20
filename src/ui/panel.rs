use macroquad::prelude::*;
use super::draw_button;

pub enum PanelItem<'a> {
    Title  { text: &'a str, size: f32, color: Color },
    Body   { text: &'a str, size: f32, color: Color },
    Button { label: &'a str, size: f32 },
    Gap    { px: f32 },
}

pub struct Panel<'a> {
    items:  Vec<PanelItem<'a>>,
    w:      f32,
    h:      f32,
    pad:    f32,
    dpr:    f32,
    bg:     Color,
    border: Color,
    x:      f32,
    y:      f32,
}

impl<'a> Panel<'a> {
    pub fn build(
        items:  Vec<PanelItem<'a>>,
        max_w:  f32,
        pad:    f32,
        dpr:    f32,
        bg:     Color,
        border: Color,
    ) -> Self {
        let inner_w = max_w - 2.0 * pad;
        let h = Self::measure(&items, inner_w, pad, dpr);
        Panel { items, w: max_w, h, pad, dpr, bg, border, x: 0.0, y: 0.0 }
    }

    pub fn centered_on(mut self, cx: f32, cy: f32) -> Self {
        self.x = cx - self.w / 2.0;
        self.y = cy - self.h / 2.0;
        self
    }

    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Draws panel background, border, and all items.
    /// Returns the index of the button that was clicked, or None.
    pub fn draw(&self) -> Option<usize> {
        draw_rectangle(self.x, self.y, self.w, self.h, self.bg);
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 2.0, self.border);

        let text_x  = self.x + self.pad;
        let inner_w = self.w - 2.0 * self.pad;
        let mut cy  = self.y + self.pad;
        let mut btn_n   = 0usize;
        let mut clicked = None;

        for item in &self.items {
            match item {
                PanelItem::Title { text, size, color } => {
                    cy += size;
                    draw_text(text, text_x, cy, *size, *color);
                    cy += size * 0.4 + 8.0 * self.dpr;
                }
                PanelItem::Body { text, size, color } => {
                    let lh = size * 1.5;
                    for line in wrap_words(text, inner_w, *size) {
                        cy += lh;
                        draw_text(&line, text_x, cy, *size, *color);
                    }
                    cy += size * 0.3;
                }
                PanelItem::Button { label, size } => {
                    let btn_w = (label.len() as f32 * size * 0.65 + 40.0 * self.dpr).max(140.0 * self.dpr);
                    let btn_h = size + 18.0 * self.dpr;
                    let btn_x = self.x + self.w / 2.0 - btn_w / 2.0;
                    cy += self.pad * 0.5;
                    if draw_button(btn_x, cy, btn_w, btn_h, label, false, *size) {
                        clicked = Some(btn_n);
                    }
                    cy += btn_h + self.pad * 0.5;
                    btn_n += 1;
                }
                PanelItem::Gap { px } => {
                    cy += px;
                }
            }
        }

        clicked
    }

    fn measure(items: &[PanelItem], inner_w: f32, pad: f32, dpr: f32) -> f32 {
        let mut h = pad * 2.0;
        for item in items {
            match item {
                PanelItem::Title { size, .. } => {
                    h += size + size * 0.4 + 8.0 * dpr;
                }
                PanelItem::Body { text, size, .. } => {
                    let n = wrap_words(text, inner_w, *size).len() as f32;
                    h += n * size * 1.5 + size * 0.3;
                }
                PanelItem::Button { size, .. } => {
                    h += size + 18.0 * dpr + pad;
                }
                PanelItem::Gap { px } => {
                    h += px;
                }
            }
        }
        h
    }
}

fn wrap_words(text: &str, max_w: f32, font_size: f32) -> Vec<String> {
    let char_w  = font_size * 0.5;
    let mut lines: Vec<String> = Vec::new();
    let mut cur   = String::new();
    let mut cur_w = 0.0f32;

    for word in text.split_whitespace() {
        let word_w = word.len() as f32 * char_w;
        if !cur.is_empty() && cur_w + char_w + word_w > max_w {
            lines.push(cur.clone());
            cur   = word.to_string();
            cur_w = word_w;
        } else {
            if !cur.is_empty() {
                cur.push(' ');
                cur_w += char_w;
            }
            cur.push_str(word);
            cur_w += word_w;
        }
    }
    if !cur.is_empty() { lines.push(cur); }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

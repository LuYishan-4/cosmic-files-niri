// Copyright 2026 LuYishan-4
// SPDX-License-Identifier: GPL-3.0-only

//! NyxNiri visual language for the standalone COSMIC Files shell.
//!
//! This module intentionally keeps all appearance decisions in one place so
//! the file-management backend can continue to track upstream COSMIC Files.

use cosmic::iced::{Background, Border, Color};
use cosmic::{theme, widget};

pub const SIDEBAR_MAX_WIDTH: u16 = 244;
pub const OUTER_RADIUS: f32 = 18.0;
pub const CARD_RADIUS: f32 = 16.0;
pub const TOOLBAR_RADIUS: f32 = 18.0;

fn alpha(mut color: Color, amount: f32) -> Color {
    color.a *= amount;
    color
}

/// Floating glass-like sidebar used instead of the stock COSMIC nav surface.
pub fn sidebar() -> theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        let background = alpha(Color::from(cosmic.primary(theme.transparent).base), 0.82);
        let border = alpha(Color::from(cosmic.accent_color()), 0.18);

        widget::container::Style {
            icon_color: Some(Color::from(cosmic.on_bg_color())),
            text_color: Some(Color::from(cosmic.on_bg_color())),
            background: Some(Background::Color(background)),
            border: Border {
                width: 1.0,
                color: border,
                radius: OUTER_RADIUS.into(),
            },
            shadow: Default::default(),
            snap: true,
        }
    })
}

/// Main content surface. Niri supplies the real compositor blur behind this.
pub fn content_panel() -> theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        let background = alpha(
            Color::from(cosmic.background(theme.transparent).base),
            0.88,
        );
        let border = alpha(Color::from(cosmic.accent_color()), 0.12);

        widget::container::Style {
            icon_color: Some(Color::from(cosmic.on_bg_color())),
            text_color: Some(Color::from(cosmic.on_bg_color())),
            background: Some(Background::Color(background)),
            border: Border {
                width: 1.0,
                color: border,
                radius: OUTER_RADIUS.into(),
            },
            shadow: Default::default(),
            snap: true,
        }
    })
}

/// Capsule containing history controls and the breadcrumb path.
pub fn toolbar() -> theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        let background = alpha(Color::from(cosmic.bg_component_color()), 0.76);
        let border = alpha(Color::from(cosmic.accent_color()), 0.16);

        widget::container::Style {
            icon_color: Some(Color::from(cosmic.on_bg_component_color())),
            text_color: Some(Color::from(cosmic.on_bg_component_color())),
            background: Some(Background::Color(background)),
            border: Border {
                width: 1.0,
                color: border,
                radius: TOOLBAR_RADIUS.into(),
            },
            shadow: Default::default(),
            snap: true,
        }
    })
}

/// A real file card: the complete icon + label tile owns the hover/selection
/// surface rather than styling icon and text independently.
pub fn file_card(selected: bool, highlighted: bool) -> theme::Container<'static> {
    theme::Container::custom(move |theme| {
        let cosmic = theme.cosmic();

        let mut background = Color::from(cosmic.bg_component_color());
        let mut border = Color::from(cosmic.on_bg_component_color());

        if selected {
            background = alpha(background, 0.92);
            border = Color::from(cosmic.accent_color());
        } else if highlighted {
            background = alpha(background, 0.72);
            border = alpha(Color::from(cosmic.accent_color()), 0.38);
        } else {
            background = alpha(background, 0.28);
            border = alpha(border, 0.10);
        }

        widget::container::Style {
            icon_color: Some(if selected {
                Color::from(cosmic.accent_color())
            } else {
                Color::from(cosmic.on_bg_component_color())
            }),
            text_color: Some(if selected {
                Color::from(cosmic.accent_text_color())
            } else {
                Color::from(cosmic.on_bg_component_color())
            }),
            background: Some(Background::Color(background)),
            border: Border {
                width: if selected { 1.5 } else { 1.0 },
                color: border,
                radius: CARD_RADIUS.into(),
            },
            shadow: Default::default(),
            snap: true,
        }
    })
}

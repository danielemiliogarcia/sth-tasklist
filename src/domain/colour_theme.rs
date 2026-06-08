//! Colour theme value object. Pure domain — no IO, no ratatui dependency.
//! Adapters convert NamedColor to their concrete colour types at the boundary.

/// All named ratatui Color variants the theme supports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
    Reset,
}

impl Default for NamedColor {
    fn default() -> Self {
        Self::Reset
    }
}

/// User-configurable colour theme for the TUI.
#[derive(Clone, Debug, PartialEq)]
pub struct ColourTheme {
    /// Border and title of the active (focused) panel.
    pub active_panel_border: NamedColor,
    /// Border and title of the inactive panel.
    pub inactive_panel_border: NamedColor,
    /// Foreground of the selected/highlighted row (used when selected_item_reverse is false).
    pub selected_item_fg: NamedColor,
    /// Background of the selected/highlighted row (used when selected_item_reverse is false).
    pub selected_item_bg: NamedColor,
    /// When true, the selected row uses terminal REVERSED modifier instead of explicit fg/bg.
    pub selected_item_reverse: bool,
    /// Foreground of normal (non-selected) rows.
    pub normal_item_fg: NamedColor,
}

impl Default for ColourTheme {
    fn default() -> Self {
        Self {
            active_panel_border: NamedColor::LightCyan,
            inactive_panel_border: NamedColor::Reset,
            selected_item_fg: NamedColor::Reset,
            selected_item_bg: NamedColor::Reset,
            selected_item_reverse: true,
            normal_item_fg: NamedColor::Reset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AT-1 covers REQ-1: default ColourTheme matches pre-feature hardcoded values
    #[test]
    fn default_theme_matches_pre_feature_hardcoded_values() {
        let theme = ColourTheme::default();
        assert_eq!(theme.active_panel_border, NamedColor::LightCyan);
        assert_eq!(theme.inactive_panel_border, NamedColor::Reset);
        assert!(theme.selected_item_reverse, "default uses REVERSED modifier");
    }
}

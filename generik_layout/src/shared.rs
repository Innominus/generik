/// Breakpoint px values and their Tailwind-style names, smallest first.
pub const BREAKPOINTS: [(u32, &str); 5] = [
    (640, "sm"),
    (768, "md"),
    (1024, "lg"),
    (1280, "xl"),
    (1536, "2xl"),
];

/// Gap size used by grid/stack/cluster/sidebar/masonry via the `gl-gap-*` classes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gutter {
    Sm,
    Md,
    Lg,
}

impl Gutter {
    /// Returns the CSS class suffix applied to the gap class, e.g. `Md` -> `"-md"`.
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Gutter::Sm => "-sm",
            Gutter::Md => "-md",
            Gutter::Lg => "-lg",
        }
    }
}

impl Default for Gutter {
    fn default() -> Self {
        Gutter::Md
    }
}

/// Which side a narrow element sits on (Sidebar media, MediaObject media).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Side::Left
    }
}

/// Flexbox `align-items` values for Cluster.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl AlignItems {
    pub fn css_value(&self) -> &'static str {
        match self {
            AlignItems::Start => "flex-start",
            AlignItems::Center => "center",
            AlignItems::End => "flex-end",
            AlignItems::Stretch => "stretch",
            AlignItems::Baseline => "baseline",
        }
    }
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Center
    }
}

/// Flexbox `justify-content` values for Cluster.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

impl JustifyContent {
    pub fn css_value(&self) -> &'static str {
        match self {
            JustifyContent::Start => "flex-start",
            JustifyContent::Center => "center",
            JustifyContent::End => "flex-end",
            JustifyContent::Between => "space-between",
            JustifyContent::Around => "space-around",
            JustifyContent::Evenly => "space-evenly",
        }
    }
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::Start
    }
}

/// Joins a base class string with an extra passthrough class, skipping empty extras.
pub fn merge_class(base: &str, extra: &str) -> String {
    if extra.is_empty() {
        base.to_string()
    } else if base.is_empty() {
        extra.to_string()
    } else {
        format!("{base} {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_class_empty_extra() {
        assert_eq!(merge_class("base", ""), "base");
    }

    #[test]
    fn merge_class_empty_base() {
        assert_eq!(merge_class("", "extra"), "extra");
    }

    #[test]
    fn merge_class_both() {
        assert_eq!(merge_class("base", "extra"), "base extra");
    }

    #[test]
    fn gutter_suffix() {
        assert_eq!(Gutter::Sm.class_suffix(), "-sm");
        assert_eq!(Gutter::Md.class_suffix(), "-md");
        assert_eq!(Gutter::Lg.class_suffix(), "-lg");
    }

    #[test]
    fn gutter_default() {
        assert_eq!(Gutter::default(), Gutter::Md);
    }

    #[test]
    fn side_default() {
        assert_eq!(Side::default(), Side::Left);
    }

    #[test]
    fn align_items_default() {
        assert_eq!(AlignItems::default(), AlignItems::Center);
    }

    #[test]
    fn align_items_css_value() {
        assert_eq!(AlignItems::Start.css_value(), "flex-start");
        assert_eq!(AlignItems::Center.css_value(), "center");
        assert_eq!(AlignItems::End.css_value(), "flex-end");
        assert_eq!(AlignItems::Stretch.css_value(), "stretch");
        assert_eq!(AlignItems::Baseline.css_value(), "baseline");
    }

    #[test]
    fn justify_content_default() {
        assert_eq!(JustifyContent::default(), JustifyContent::Start);
    }

    #[test]
    fn justify_content_css_value() {
        assert_eq!(JustifyContent::Start.css_value(), "flex-start");
        assert_eq!(JustifyContent::Center.css_value(), "center");
        assert_eq!(JustifyContent::End.css_value(), "flex-end");
        assert_eq!(JustifyContent::Between.css_value(), "space-between");
        assert_eq!(JustifyContent::Around.css_value(), "space-around");
        assert_eq!(JustifyContent::Evenly.css_value(), "space-evenly");
    }
}
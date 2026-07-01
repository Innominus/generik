use leptos::prelude::*;

use crate::shared::merge_class;
pub use crate::shared::RuleVariant;

/// A horizontal rule with thickness variants and an optional inline label.
///
/// Renders a `<hr>`-equivalent `<div>` (so it can carry a label child) styled
/// via the `gl-rule` class. The label, if any, sits inline on the rule with
/// the line broken around it — the classic editorial section-break with a
/// centered eyebrow ("FEATURE", "ISSUE 14").
///
/// Props: `variant` (default `Thin`), `label` (optional `&'static str` — if
/// present, rendered as a small mono eyebrow on the rule), `class`/`style`
/// (passthrough).
#[component]
pub fn Rule(
    #[prop(optional)] variant: RuleVariant,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let variant_class = rule_variant_class(variant);
    let class_str = merge_class("gl-rule", class);
    let line_class = format!("gl-rule-line {variant_class}");

    view! {
        <div class=class_str style=style>
            <span class=line_class.clone()></span>
            {label.map(|l| view! { <span class="gl-rule-label">{l}</span> <span class=line_class.clone()></span> })}
        </div>
    }
}

/// Returns the CSS modifier class for a `RuleVariant` (`Thin` -> `gl-rule-thin`, etc.).
fn rule_variant_class(v: RuleVariant) -> &'static str {
    match v {
        RuleVariant::Thin => "gl-rule-thin",
        RuleVariant::Thick => "gl-rule-thick",
        RuleVariant::Dotted => "gl-rule-dotted",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_mapping() {
        assert_eq!(rule_variant_class(RuleVariant::Thin), "gl-rule-thin");
        assert_eq!(rule_variant_class(RuleVariant::Thick), "gl-rule-thick");
        assert_eq!(rule_variant_class(RuleVariant::Dotted), "gl-rule-dotted");
    }
}
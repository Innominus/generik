use leptos::{
    prelude::{document, GetUntracked},
};
use leptos_router::hooks::use_location;
use web_sys::Element;

#[inline]
pub fn get_fragment_element() -> Option<Element> {
    let hash = use_location().hash.get_untracked();

    document().get_element_by_id(&hash.trim_start_matches("#"))
}

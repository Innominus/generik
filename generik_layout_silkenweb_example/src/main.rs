//! Minimal Trunk-served example for `generik_layout::silkenweb`.

use generik_layout::silkenweb::{col, container, grid, mount_styles, stack};
use generik_layout::Gutter;
use silkenweb::dom::DefaultDom;
use silkenweb::elements::html::{div, h1, p};
use silkenweb::mount;
use silkenweb::node::element::TextParentElement;
use silkenweb::node::Node;

fn main() {
    mount_styles();

    let columns: Vec<Node<DefaultDom>> = vec![
        col::<DefaultDom>(
            12,
            None,
            Some(6),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            [div().text("Main")],
        )
        .into(),
        col::<DefaultDom>(
            12,
            None,
            Some(6),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "",
            "",
            [div().text("Aside")],
        )
        .into(),
    ];
    let sections: Vec<Node<DefaultDom>> = vec![
        h1().text("generik_layout · Silkenweb").into(),
        p().text("A minimal Trunk-served layout adapter example.")
            .into(),
        grid::<DefaultDom>(Gutter::Md, "", "", "", columns).into(),
    ];
    let app = container::<DefaultDom>(
        "960px",
        "",
        "margin: 0 auto; padding: 2rem;",
        [stack::<DefaultDom>(Gutter::Lg, "", "", sections)],
    );

    mount("app", app);
}

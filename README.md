# generik

This workspace contains browser/WASM utilities and the `generik_layout` Swiss/editorial layout library.

## Layout adapters

`generik_layout` provides two feature-selected adapters over the same global `gl-*` CSS contract:

| Feature | Module | Example |
| --- | --- | --- |
| `leptos` (default) | `generik_layout::components` | `generik_layout_examples` |
| `silkenweb` | `generik_layout::silkenweb` | `generik_layout_silkenweb_example` |

Run the Silkenweb example with:

```sh
cd generik_layout_silkenweb_example
trunk serve
```

Call `generik_layout::silkenweb::mount_styles()` once during startup, then mount the adapter elements with `silkenweb::mount`.

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

const TOAST_DURATION_MS: u32 = 4500;
const TOAST_EXIT_MS: u32 = 300;

#[derive(Clone, Copy)]
pub enum ToastKind {
    Default,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
struct Toast {
    id: u64,
    message: String,
    kind: ToastKind,
    leaving: RwSignal<bool>,
}

#[derive(Clone)]
pub struct ToastManager {
    toasts: RwSignal<Vec<Toast>>,
    next_id: RwSignal<u64>,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(0),
        }
    }

    fn next_toast_id(&self) -> u64 {
        let id = self.next_id.get_untracked();
        self.next_id.set(id.wrapping_add(1));
        id
    }

    pub fn push(&self, message: impl Into<String>) {
        self.push_with(message, ToastKind::Default);
    }

    pub fn push_with(&self, message: impl Into<String>, kind: ToastKind) {
        let id = self.next_toast_id();
        let toast = Toast {
            id,
            message: message.into(),
            kind,
            leaving: RwSignal::new(false),
        };

        self.toasts.update(|toasts| toasts.push(toast.clone()));

        let toasts = self.toasts;
        let leaving = toast.leaving;
        spawn_local(async move {
            TimeoutFuture::new(TOAST_DURATION_MS).await;
            leaving.set(true);
            TimeoutFuture::new(TOAST_EXIT_MS).await;
            toasts.update(|toasts| toasts.retain(|toast| toast.id != id));
        });
    }

    pub fn dismiss(&self, id: u64) {
        let toast = self
            .toasts
            .get_untracked()
            .into_iter()
            .find(|toast| toast.id == id);

        if let Some(toast) = toast {
            let toasts = self.toasts;
            let leaving = toast.leaving;
            spawn_local(async move {
                leaving.set(true);
                TimeoutFuture::new(TOAST_EXIT_MS).await;
                toasts.update(|toasts| toasts.retain(|toast| toast.id != id));
            });
        }
    }

    pub fn success(&self, message: impl Into<String>) {
        self.push_with(message, ToastKind::Success);
    }

    pub fn warning(&self, message: impl Into<String>) {
        self.push_with(message, ToastKind::Warning);
    }

    pub fn error(&self, message: impl Into<String>) {
        self.push_with(message, ToastKind::Error);
    }
}

pub fn use_toast() -> ToastManager {
    use_context::<ToastManager>().expect("ToastProvider is missing")
}

#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let manager = ToastManager::new();
    provide_context(manager.clone());

    view! {
        <style>
            r#"@keyframes toastIn{0%{opacity:0;transform:translateY(-16px) scale(.98)}100%{opacity:1;transform:translateY(0) scale(1)}}.toast-viewport{pointer-events:none;position:fixed;top:1rem;left:0;right:0;z-index:50;display:flex;justify-content:center;padding:0 1rem}.toast-stack{width:100%;max-width:28rem;display:flex;flex-direction:column;margin-top:-.5rem}.toast-shell{margin-top:.5rem;transition:margin .3s ease,opacity .24s ease;border-radius:.75rem;box-shadow:0 12px 24px rgba(15,23,42,.16)}.toast-shell-exit{margin-top:0;opacity:0}.toast-clip{max-height:120px;overflow:hidden;transition:max-height .3s ease;border-radius:inherit}.toast-clip-exit{max-height:0}.toast-card{pointer-events:auto;border-radius:inherit;border:1px solid rgba(229,231,235,1);background:rgba(255,255,255,.95);backdrop-filter:blur(10px);background-clip:padding-box;padding:.75rem 1rem}.toast-text{font-size:.875rem;font-weight:600;color:#111827}.toast-success{border-color:#a7f3d0;background:#ecfdf5}.toast-success .toast-text{color:#047857}.toast-warning{border-color:#fde68a;background:#fffbeb}.toast-warning .toast-text{color:#92400e}.toast-error{border-color:#fecaca;background:#fef2f2}.toast-error .toast-text{color:#b91c1c}.toast-enter{animation:toastIn 220ms ease-out}"#
        </style>
        {children()}
        <ToastViewport manager=manager />
    }
}

#[component]
fn ToastViewport(manager: ToastManager) -> impl IntoView {
    let toasts = manager.toasts;

    view! {
        <div class="toast-viewport">
            <div class="toast-stack">
                <For each=move || toasts.get() key=|toast| toast.id let:toast>
                    <ToastItem toast=toast manager=manager.clone() />
                </For>
            </div>
        </div>
    }
}

#[component]
fn ToastItem(toast: Toast, manager: ToastManager) -> impl IntoView {
    let class = move || {
        if toast.leaving.get() {
            "toast-shell toast-shell-exit".to_string()
        } else {
            "toast-shell toast-enter".to_string()
        }
    };

    let dismiss = { move |_| manager.dismiss(toast.id) };

    view! {
        <div class=class>
            <div class=move || {
                if toast.leaving.get() { "toast-clip toast-clip-exit" } else { "toast-clip" }
            }>
                <div
                    class=move || {
                        let kind_class = match toast.kind {
                            ToastKind::Default => "",
                            ToastKind::Success => "toast-success",
                            ToastKind::Warning => "toast-warning",
                            ToastKind::Error => "toast-error",
                        };
                        format!("toast-card {kind_class}")
                    }
                    on:click=dismiss
                >
                    <span class="toast-text">{toast.message}</span>
                </div>
            </div>
        </div>
    }
}

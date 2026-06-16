use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FormStatus {
    pub submitting: RwSignal<bool>,
}

impl FormStatus {
    pub fn new() -> Self {
        Self {
            submitting: RwSignal::new(false),
        }
    }
}

#[derive(Clone)]
pub struct FieldErrors {
    map: RwSignal<HashMap<String, String>>,
}

impl FieldErrors {
    pub fn new() -> Self {
        Self {
            map: RwSignal::new(HashMap::new()),
        }
    }

    pub fn clear(&self) {
        self.map.set(HashMap::new());
    }

    pub fn set(&self, field: &str, message: impl Into<String>) {
        self.map.update(|map| {
            map.insert(field.to_string(), message.into());
        });
    }

    pub fn message_signal_owned(&self, field: String) -> Signal<Option<String>> {
        let map = self.map;
        Signal::derive(move || map.get().get(&field).cloned())
    }
}

#[component]
pub fn FormCard(
    #[prop(into)] title: String,
    #[prop(optional)] field_errors: Option<FieldErrors>,
    children: Children,
) -> impl IntoView {
    if let Some(field_errors) = field_errors {
        provide_context(field_errors);
    }

    view! {
        <div class="p-6 mx-auto w-full bg-white rounded-lg border shadow-lg sm:p-8 lg:w-4/5 xl:w-1/3 border-base-200">
            <h2 class="mb-6 text-2xl font-bold text-center text-gray-800">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
pub fn FormField(
    #[prop(into)] label: String,
    #[prop(into)] for_id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <label for=for_id class="block text-sm font-medium text-gray-700">
                {label}
            </label>
            {children()}
        </div>
    }
}

#[component]
pub fn FormFieldError(message: Signal<Option<String>>) -> impl IntoView {
    view! {
        <Show when=move || message.get().is_some()>
            <p class="text-xs text-red-600">{move || message.get().unwrap_or_default()}</p>
        </Show>
    }
}

#[component]
pub fn FormInput(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(into, default = "text".to_string())] input_type: String,
    #[prop(default = 100usize)] max_length: usize,
    #[prop(default = true)] required: bool,
) -> impl IntoView {
    let on_input = move |event| {
        value.set(event_target_value(&event));
    };
    let field_errors = use_context::<FieldErrors>();
    let error = field_errors.map(|errors| errors.message_signal_owned(id.clone()));

    view! {
        <FormField label=label for_id=id.clone()>
            <input
                maxlength=max_length
                type=input_type
                id=id
                name=name
                required=required
                prop:value=move || value.get()
                on:input=on_input
                class="py-2 px-3 w-full rounded-md border border-gray-300 shadow-sm transition-colors focus:border-purple-500 focus:ring-2 focus:ring-purple-500 focus:outline-none"
            />
            {error.map(|message| view! { <FormFieldError message=message /> })}
        </FormField>
    }
}

#[component]
pub fn FormTextarea(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(default = 4u16)] rows: u16,
    #[prop(default = 400usize)] max_length: usize,
    #[prop(default = true)] required: bool,
    #[prop(optional)] footer: Option<AnyView>,
) -> impl IntoView {
    let on_input = move |event| {
        value.set(event_target_value(&event));
    };
    let field_errors = use_context::<FieldErrors>();
    let error = field_errors.map(|errors| errors.message_signal_owned(id.clone()));

    view! {
        <FormField label=label for_id=id.clone()>
            <textarea
                maxlength=max_length
                id=id
                name=name
                rows=rows
                required=required
                prop:value=move || value.get()
                on:input=on_input
                class="py-2 px-3 w-full rounded-md border border-gray-300 shadow-sm transition-colors focus:border-purple-500 focus:ring-2 focus:ring-purple-500 focus:outline-none min-h-64 resize-vertical"
            ></textarea>
            {error.map(|message| view! { <FormFieldError message=message /> })}
            {footer.map(|footer| view! { <div class="flex justify-end">{footer}</div> })}
        </FormField>
    }
}

#[component]
pub fn InlineSpinner() -> impl IntoView {
    view! {
        <span class="inline-block w-4 h-4 rounded-full border-2 animate-spin border-white/60 border-t-white"></span>
    }
}

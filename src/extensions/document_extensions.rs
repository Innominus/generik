use web_sys::{wasm_bindgen::JsCast, Document};

pub trait DocumentExtensions {
    fn get_element_by_id_unchecked<T>(&self, element_id: &str) -> Option<T>
    where
        T: JsCast;
}

impl DocumentExtensions for Document {
    fn get_element_by_id_unchecked<T>(&self, element_id: &str) -> Option<T>
    where
        T: JsCast,
    {
        self.get_element_by_id(element_id)
            .map(|el| el.unchecked_into::<T>())
    }
}

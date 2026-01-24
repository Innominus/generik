pub trait StrToggler {
    fn toggle_class<S: AsRef<str>>(self, class: S, when: &dyn Fn() -> bool) -> String;
    fn push_class<S: AsRef<str>>(self, class: S) -> String;
}

impl StrToggler for &str {
    fn toggle_class<S: AsRef<str>>(self, class: S, when: &dyn Fn() -> bool) -> String {
        let class = class.as_ref();

        if when() {
            self.to_string() + " " + class
        } else {
            self.to_string()
        }
    }

    fn push_class<S: AsRef<str>>(self, class: S) -> String {
        self.to_string() + " " + class.as_ref()
    }
}

impl StrToggler for String {
    fn toggle_class<S: AsRef<str>>(self, class: S, when: &dyn Fn() -> bool) -> String {
        let class = class.as_ref();

        if when() {
            self + " " + class
        } else {
            self
        }
    }

    fn push_class<S: AsRef<str>>(self, class: S) -> String {
        self + " " + class.as_ref()
    }
}

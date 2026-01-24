use web_sys::{wasm_bindgen::JsCast, window, HtmlDocument};
// TODO: SPLIT UTILS INTO SEPARATE CRATE AND USE

// Function to set a cookie securely
pub fn set_secure_cookie(name: &str, value: &str, path: Option<&str>) {
    let document = window()
        .unwrap()
        .document()
        .unwrap()
        .unchecked_into::<HtmlDocument>();
    // Build the cookie string
    let mut cookie_string = name.to_string() + "=" + value;

    if let Some(path) = path {
        cookie_string.push_str(&("; Path={}".to_string() + path));
    }

    cookie_string.push_str("; Secure");

    cookie_string.push_str("; SameSite=Strict");

    document.set_cookie(&cookie_string).unwrap()
}

pub fn get_cookie(name: &str) -> Option<String> {
    let document = window()
        .unwrap()
        .document()
        .unwrap()
        .unchecked_into::<HtmlDocument>();

    let cookies = document.cookie().unwrap();

    cookies.split(';').map(|s| s.trim()).find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let key = parts.next()?.trim();
        let value = parts.next()?.trim();
        if key == name {
            Some(value.to_string())
        } else {
            None
        }
    })
}

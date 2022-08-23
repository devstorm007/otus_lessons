/*
cargo build (run, check, ...) запускаются всех крейтов. Есть параметры --lib и --bin name.
pub элементы из lib.rs могут быть использованы при подключении данного пакета в качестве зависимости.
*/

//https://doc.rust-lang.org/reference/conditional-compilation.html
//Аттрибут #[cfg(condition)]
#[cfg(target_os = "windows")]
fn win_foo() {
    todo!()
}

//Аттрибут #[cfg_attr(condition, attribute)]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
//#[cfg_attr(macos, path = "macos.rs")]
//mod os;

//Макрос cfg!(condition)
fn sample() {
    let os = if cfg!(unix) { "windows" } else { "linux" };
    todo!()
}

//all(), any(), not()
#[cfg(any(cond1, cond2))]
fn foo_or_bar() {
    todo!()
}

#[cfg(all(unix, target_pointer_width = "32"))]
fn for_32bit_unix() {
    todo!()
}

#[cfg(not(cond1))]
fn not_foo() {
    todo!()
}

/*
features

[dependencies]
winapi = { version = "0.3.9", optional = true }
winapi-util = { version = "0.1.5", optional = true }

[features]
win = ["winapi", "winapi-util"]

#[cfg(feature = "win")]
fn use_winapi() {
    let void = winapi::ctypes::c_int::from(5);
    let f_typ = winapi_util::console::Color::Blue;
}

cargo <command> --features “feature1 feature2 ...”

Фичи, включенные по умолчанию перечисляются в default фиче. Отключить
их можно через флаг --no-default-features.
*/

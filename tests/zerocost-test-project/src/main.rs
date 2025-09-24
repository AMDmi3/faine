#[allow(unused_imports)]
use faine::inject_return;

fn test() -> &'static str {
    #[cfg(feature = "faine")]
    {
        inject_return!("injected");
    }
    ""
}

fn main() {
    let size = std::env::current_exe()
        .ok()
        .and_then(|path| std::fs::metadata(path).ok())
        .map(|attr| attr.len())
        .unwrap_or_default();
    println!("{}{}", test(), size);
}

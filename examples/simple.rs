use system_theme::SystemTheme;

fn main() {
    let theme = SystemTheme::new().unwrap();
    println!("Theme kind: {:?}", theme.get_kind());
    println!("Theme scheme: {:?}", theme.get_scheme());
    println!("Theme contrast: {:?}", theme.get_contrast());
    println!("Theme accent: {:?}", theme.get_accent());

    // Full theme (with fallbacks for unsupported methods)
    println!("Theme: {:?}", theme.get_theme());
}

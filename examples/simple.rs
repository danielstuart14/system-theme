use system_theme::SystemTheme;

fn main() {
    let theme = SystemTheme::new().unwrap();
    println!("Theme scheme: {:?}", theme.theme_scheme());
    println!("Theme contrast: {:?}", theme.theme_contrast());
    println!("Theme accent: {:?}", theme.theme_accent());
}

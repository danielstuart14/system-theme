use futures_util::{pin_mut, StreamExt};
use system_theme::SystemTheme;

#[tokio::main]
async fn main() {
    let theme = SystemTheme::new().unwrap();

    println!("Theme: {:?}", theme.get_theme());

    let subscription = theme.subscribe();
    pin_mut!(subscription);

    loop {
        println!("Waiting for theme change...");

        if let None = subscription.next().await {
            println!("No more theme changes");
            break;
        }

        println!("Theme: {:?}", theme.get_theme());
    }
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
struct Cookies(pub i32);

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
struct ClickPower(pub i32);

#[component]
fn UpgradeButton(amount: i32) -> Element {
    let mut click_power = use_context::<Signal<ClickPower>>();
    let mut cookies = use_context::<Signal<Cookies>>();
    rsx!(
        Button {
            onclick: move |_| {
                if cookies.read().0 >= amount * 100 {
                    cookies.write().0 -= amount * 100;
                    click_power.write().0 += amount;
                }
            },
            label {
                "Increase Click Power +{amount} ({amount * 100} cookies)"
            }
        }
    )
}

fn app() -> Element {
    let mut click_power = use_signal(|| ClickPower(1));
    let mut cookies = use_signal(|| Cookies(0));

    use_context_provider(|| click_power);
    use_context_provider(|| cookies);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "black",
            color: "white",
            main_align: "center",
            cross_align: "center",
            label {
                font_size: "75",
                font_weight: "bold",
                onclick: move |_| {
                    let cur_cookies = cookies.read().0;
                    *cookies.write() = Cookies(cur_cookies + click_power.read().0);
                },
                "{cookies.read().0}"
            }
            UpgradeButton {
                amount: 1,
            }
            UpgradeButton {
                amount: 10,
            }
            UpgradeButton {
                amount: 100,
            }
            UpgradeButton {
                amount: 1000,
            }
        }
    )
}

fn main() {
    launch(app);
}

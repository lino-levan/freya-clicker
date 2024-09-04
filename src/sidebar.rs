use freya::prelude::*;
use std::time::Duration;
use tokio::time::interval;

use crate::types::{ClickPower, Crabs, Upgrades};

#[component]
pub fn Sidebar() -> Element {
    let click_power = use_context::<Signal<ClickPower>>();
    let upgrades = use_context::<Signal<Upgrades>>();
    let mut crabs = use_context::<Signal<Crabs>>();
    let mut temp_crabs = use_signal(|| 0);

    use_future(move || async move {
        let mut count = 0;
        let mut int = interval(Duration::from_millis(100));
        loop {
            int.tick().await;
            temp_crabs += upgrades.read().rate() / 10;
            count += 1;
            if count % 10 == 0 {
                count = 0;
                crabs.write().0 += upgrades.read().rate();
                *temp_crabs.write() = 0;
            }
        }
    });

    rsx!(
        rect {
            height: "100%",
            width: "300",
            background: "blue",
            onclick: move |_| {
                let cur_crabs = crabs.read().0;
                *crabs.write() = Crabs(cur_crabs + click_power.read().0);
            },
            label {
                width: "100%",
                font_size: "40",
                font_weight: "bold",
                text_align: "center",
                "Your Crab House"
            }
            label {
                width: "100%",
                font_size: "30",
                font_weight: "bold",
                text_align: "center",
                "{crabs.read().0 + *temp_crabs.read()} crabs"
            }
            label {
                width: "100%",
                font_size: "200",
                text_align: "center",
                "🦀"
            }
        }
    )
}

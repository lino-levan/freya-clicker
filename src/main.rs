#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod sidebar;
mod types;

use freya::prelude::*;
use sidebar::Sidebar;
use types::{ClickPower, Crabs, Upgrades};

fn app() -> Element {
    let click_power = use_signal(|| ClickPower(1));
    let mut upgrades = use_signal(|| Upgrades::init());
    let mut crabs = use_signal(|| Crabs(100000));

    use_context_provider(|| click_power);
    use_context_provider(|| upgrades);
    use_context_provider(|| crabs);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "black",
            color: "white",
            direction: "horizontal",
            Sidebar {}
            rect {
                height: "100%",
                width: "fill",
                label {
                    font_size: "40",
                    font_weight: "bold",
                    text_align: "center",
                    "Store"
                }
                rect {
                    width: "100%",
                    padding: "10",
                    direction: "horizontal",
                    cross_align: "center",
                    background: "gray",
                    border: "2 solid white",
                    border_align: "inner",
                    onclick: move |_| {
                        if crabs.read().0 >= 100 {
                            crabs.write().0 -= 100;
                            upgrades.write().claws += 1;
                        }
                    },
                    rect {
                        label {
                            font_size: "20",
                            font_weight: "bold",
                            "Claws"
                        }
                        label {
                            font_size: "20",
                            "{100} crabs"
                        }
                    }
                    label {
                        font_size: "20",
                        font_weight: "bold",
                        text_align: "right",
                        "{upgrades.read().claws}"
                    }
                }
                rect {
                    width: "100%",
                    padding: "10",
                    direction: "horizontal",
                    cross_align: "center",
                    background: "gray",
                    border: "2 solid white",
                    border_align: "inner",
                    onclick: move |_| {
                        if crabs.read().0 >= 1000 {
                            crabs.write().0 -= 1000;
                            upgrades.write().nets += 1;
                        }
                    },
                    rect {
                        label {
                            font_size: "20",
                            font_weight: "bold",
                            "Nets"
                        }
                        label {
                            font_size: "20",
                            "{1000} crabs"
                        }
                    }
                    label {
                        font_size: "20",
                        font_weight: "bold",
                        text_align: "right",
                        "{upgrades.read().nets}"
                    }
                }
                rect {
                    width: "100%",
                    padding: "10",
                    direction: "horizontal",
                    cross_align: "center",
                    background: "gray",
                    border: "2 solid white",
                    border_align: "inner",
                    onclick: move |_| {
                        if crabs.read().0 >= 10000 {
                            crabs.write().0 -= 10000;
                            upgrades.write().buckets += 1;
                        }
                    },
                    rect {
                        label {
                            font_size: "20",
                            font_weight: "bold",
                            "Buckets"
                        }
                        label {
                            font_size: "20",
                            "{10000} buckets"
                        }
                    }
                    label {
                        font_size: "20",
                        font_weight: "bold",
                        text_align: "right",
                        "{upgrades.read().buckets}"
                    }
                }
            }
        }
    )
}

fn main() {
    launch(app);
}

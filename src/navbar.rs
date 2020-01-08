use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html, clone, events, link};
use futures_signals::signal::{SignalExt, Mutable};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::{utils};
use crate::routing::Route;


pub struct NavBar {
  navbar_isopen: Mutable<bool>,
  hidden: Mutable<bool>,
}

impl NavBar {
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            navbar_isopen: Mutable::new(false),
            hidden: Mutable::new(true),
        })
    }

    pub fn render(state: Rc<Self>) -> Dom {
        lazy_static! {
            /*
            static ref FIXED_TOP: String = class! {
                .style("position", "fixed")
                .style("max-height", "60px")
                .style("top", "0px")
                .style("right", "0px")
                .style("left", "0px")
                .style("z-index", "1030")
            };
            */
        }

        html!("div", {
            .class_signal("navbar-transparent", state.hidden.signal())
            .global_event(clone!(state => move |_: events::Scroll| {
                let window = utils::window();

                if window.page_y_offset().unwrap() > 299.0 {
                    state.hidden.set_neq(false);
                } else {
                    state.hidden.set_neq(true);
                }
            }))
            .visible_signal(Route::signal().map(move |x| x!=Route::Signup))
            .style("max-height", "60px")
            .style("padding-top", "12px")
            .class(["fixed-top", "navbar", "navbar-expand-lg"])
            .class_signal("nav-open", state.navbar_isopen.signal().map(|x| x))
            .children(&mut [

                html!("div", {
                    .class("navbar-translate")
                    .children(&mut [


                        link!("#/", {
                            .class("navbar-brand")
                            .attribute("data-placement", "bottom")
                            .attribute("title", "Coded by ZPOW")
                            .children(&mut [
                                html!("img", {
                                    //.style("color", "#dc3545")
                                    .style("width", "32px")
                                    .style("height", "32px")
                                    .class("logo-rust-smaller")
                                    .attribute("src", "assets/img/logo.png")
                                }),
                                html!("span", {
                                    .text("  RustCloud")
                                })
                            ])
                        }),


                        html!("button", {
                            .event(clone!(state => move |_: events::Click| {
                                state.navbar_isopen.replace_with(|x| !*x);
                            }))
                            .class("navbar-toggler")
                            .children(&mut [
                                html!("span", { .class(["navbar-toggler-bar", "bar1"]) }),
                                html!("span", { .class(["navbar-toggler-bar", "bar2"]) }),
                                html!("span", { .class(["navbar-toggler-bar", "bar3"]) })
                            ])
                        })

                        
                    ])
                }),
                
                //Menu Section
                html!("div", {
                    .class(["navbar-collapse", "collapse", "justify-content-end"])
                    .children(&mut [
                        html!("ul", {
                            .class("navbar-nav")
                            .children(&mut [
                                

                                html!("li", {
                                    .class("nav-item")
                                    .children(&mut [
                                        link!("#/docs", {
                                            .class("nav-link")
                                            .children(&mut [
                                                html!("i", {
                                                    .class(["nc-icon", "nc-book-bookmark"])
                                                }),
                                                html!("span", {
                                                    .text("Docs")
                                                })
                                            ])
                                        })
                                    ])
                                }),
                                

                                html!("li", {
                                    .class("nav-item")
                                    .children(&mut [

                                        link!("#/pricing", {
                                            .class("nav-link")
                                            .children(&mut [
                                                html!("i", {
                                                    .class(["nc-icon", "nc-money-coins"])
                                                }),
                                                html!("span", {
                                                    .text("Pricing")
                                                })
                                            ])
                                        })

                                    ])
                                }),


                                html!("li", {
                                    .class("nav-item")
                                    .children(&mut [

                                        html!("a", {
                                            .class("nav-link")
                                            .attribute("href", "https://discord.gg/YArPaqT")
                                            .attribute("target", "_blank")
                                            .children(&mut [
                                                html!("i", {
                                                    .class(["fab", "fa-discord", "fa-2x"])
                                                }),
                                                html!("span", {
                                                    .text("Discord")
                                                })
                                            ])
                                        })

                                    ])
                                }),


                                html!("li", {
                                    .class("nav-item")
                                    .children(&mut [

                                        html!("a", {
                                            .class("nav-link")
                                            .attribute("href", "https://github.com/thienpow")
                                            .attribute("target", "_blank")
                                            .children(&mut [
                                                html!("i", {
                                                    .class(["fab", "fa-github", "fa-2x"])
                                                }),
                                                html!("span", {
                                                    .text("Github")
                                                })
                                            ])
                                        })

                                    ])
                                }),


                                html!("li", {
                                    .style("padding-top", "3px")
                                    .class("nav-item")
                                    .children(&mut [

                                        link!("#/signup", {
                                            .class(["btn", "btn-round", "btn-danger"])
                                            .text("Sign Up")
                                        })

                                    ])
                                })
                                
                                
                            ])
                        })
                    ])
                    
                })


            ])
        })
    }
}
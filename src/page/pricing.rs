use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;


pub struct PricingPage {
  //navbar_isopen: Mutable<bool>,
}

impl PricingPage {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            //navbar_isopen: Mutable::new(false),
            
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
            .class(["page-header", "page-header-medium", "bg-dark"])
            .visible_signal(Route::signal().map(move |x| x==Route::Pricing))
            .style("height", "850px")
            .children(&mut [

                html!("div", {
                    .class(["content-center", "container-autoscale"])
                    .children(&mut [
                        html!("div", {
                            .class("title-brand")
                            .children(&mut [
                                html!("h1", {
                                    .class("presentation-title")
                                    .text("Pricing")
                                }),
                            ])
                        }),
                        html!("h2", {
                            .class(["presentation-subtitle", "text-center"])
                            .text("Cheap cheap no good. Good good not cheap.")
                        })
                    ])
                }),


            ])
        })
    }
}
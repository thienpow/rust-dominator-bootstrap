use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;


pub struct SignupPage {
  //navbar_isopen: Mutable<bool>,
}

impl SignupPage {
  
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
            .visible_signal(Route::signal().map(move |x| x==Route::Pricing))
            .style("height", "1200px")
            .style("color", "white")
            .text("signup here.")
            //.children(&mut [


            //])
        })
    }
}
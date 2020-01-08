use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;


pub struct SignupPage {
    
}

impl SignupPage {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .class(["page-header", "page-header-medium", "bg-dark"])
            .visible_signal(Route::signal().map(move |x| x==Route::Signup))
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
                                    .text("Signup")
                                }),
                            ])
                        }),
                        html!("h2", {
                            .class(["presentation-subtitle", "text-center"])
                            .text("Just a bootstrap demo. Might add in extra demo feature here.")
                        })
                    ])
                }),


            ])
        })
    }
}
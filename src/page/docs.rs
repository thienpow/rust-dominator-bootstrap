use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;


pub struct DocsPage {
    
}

impl DocsPage {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {

        html!("div", {
            .class(["page-header", "page-header-medium", "bg-dark"])
            .visible_signal(Route::signal().map(move |x| x==Route::Docs))
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
                                    .text("Docs")
                                }),
                            ])
                        }),
                        html!("h2", {
                            .class(["presentation-subtitle", "text-center"])
                            .text("Knowledge begin from hungry.")
                        })
                    ])
                }),


            ])
        })
    }
}
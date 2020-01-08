use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;

pub struct IndexHeader {
}

impl IndexHeader {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {

        html!("div", {
            //.style("background-image", "url('assets/img/bg-bottom.svg')")
            .class(["page-header", "page-header-medium", "bg-dark"])
            
            .children(&mut [

                /*
                html!("div", {
                    .class("welcome-bg")
                    .children(&mut [
                        html!("img", {
                            .attribute("src", "assets/img/bg-bottom.svg")
                        })
                    ])
                }),
                */
                
                html!("div", {
                    .class("video-container")
                    .children(&mut [


                        html!("div", {
                            .class("filter")
                        }),
                        html!("video", {
                            .attribute("playsinline", "playsinline")
                            .attribute("autoplay", "autoplay")
                            .attribute("muted", "muted")
                            .attribute("loop", "loop")
                            .children(&mut [
                                html!("source", {
                                    .attribute("src", "assets/video/iheader.mp4")
                                    .attribute("type", "video/mp4")
                                })
                            ])
                        })


                    ])
                }),
                

                html!("div", {
                    .class(["content-center", "container-autoscale"])
                    .children(&mut [
                        html!("div", {
                            .class("title-brand")
                            .children(&mut [
                                html!("h1", {
                                    .class("presentation-title")
                                    .text("RustCloud")
                                }),
                            ])
                        }),
                        html!("h2", {
                            .class(["presentation-subtitle", "text-center"])
                            .text("Decentralized Ochestration for Makers")
                        })
                    ])
                }),


                html!("div", {
                    .class("index-header-bottom")
                    .style("padding-bottom", "7px")
                    .children(&mut [

                        html!("a", {
                            .text("Powered by")
                        }),
                        html!("img", {
                            .class("logo-rust-small")
                            .attribute("src", "assets/img/logo_white_s.png")
                        })

                    ])
                })


            ])
        })
    }
}
use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html, clone, events, link};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;

pub struct Footer {
}

impl Footer {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {

        html!("div", {
            .class(["footer"])
            .children(&mut [

                html!("div", {
                    .class(["footer-container", "row"])
                    .children(&mut [
                        

                        html!("nav", {
                            .class("footer-nav")
                            .children(&mut [

                                html!("ul", {
                                    .children(&mut [
                                        html!("li", {
                                            .children(&mut [
                                                link!("#/about", {
                                                    .text("ABOUT")
                                                })
                                            ])
                                        }),
                                        html!("li", {
                                            .children(&mut [
                                                link!("https://thienpow.gitbook.io/rustcloud", {
                                                    .text("BLOG")
                                                })
                                            ])
                                        }),
                                        html!("li", {
                                            .children(&mut [
                                                link!("https://thienpow.gitbook.io/rustcloud/license", {
                                                    .text("LICENSE")
                                                })
                                            ])
                                        }),
                                    ])
                                })
                            ])
                        }),


                        html!("div", {
                            .class(["credits", "ml-auto"])
                            .children(&mut [
                                html!("span", {
                                    .class("copyright")
                                    .children(&mut [
                                        html!("span", {
                                            .text("©2019 made with ")
                                        }),
                                        html!("i", {
                                            .class(["fas", "fa-heart", "heart"])
                                        }),
                                        html!("span", {
                                            .text(" by ZPOW")
                                        }),
                                    ])
                                })
                            ])
                        })


                    ])//footer-container
                })//footer
            ])
        })
    }
}
use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::{video, index_section, column_one, column_two, label};
use crate::routing::Route;

pub struct IndexSec1 {
}

impl IndexSec1 {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {

        index_section!("D-ID", {
            .children(&mut [

                html!("div", {
                    .style("width", "3%")
                }),
                html!("img", {
                    .style("width", "38px")
                    .attribute("src", "assets/img/divider.svg")
                }),
                column_one!("D-ID", "Decentralized Identification", {
                    .children(&mut [
                        label!("Nobody should store your Password or Wallet Keys.  It simply doesn't make sense to let someone else holding your House and Safe keys."),
                        label!("D-ID make use of existing Blockchain technologies for authentication and autorization, and a D-ID registrar would merely keep an alias/username and a pointer to the user."),
                        label!("No password, no profile, no contacts would be kept on any server."),
                    ])
                }),
                column_two!({
                    .children(&mut [
                        video!("assets/video/iheader.mp4", { 
                            .attribute("controls", "controls")
                        })
                    ])
                }),

            ])
        })
      
    }
}
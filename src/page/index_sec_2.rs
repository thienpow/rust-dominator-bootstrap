use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::{video, index_section, column_one, column_two, label};
use crate::routing::Route;

pub struct IndexSec2 {
}

impl IndexSec2 {
  
    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {

        index_section!("D-FS", {
            .children(&mut [

                html!("div", {
                    .style("width", "3%")
                }),
                html!("img", {
                    .style("width", "38px")
                    .attribute("src", "assets/img/divider.svg")
                }),
                column_one!("D-FS", "Decentralized File Storage", {
                    .children(&mut [
                        label!("D-FS is using IPFS as it's core infrastructure, enhanced with developer access control plane and infra ochestration UX."),
                        label!("-- 'IPFS is a peer-to-peer (p2p) storage network. Content is accessible through peers that might relay information or store it (or do both), and those peers can be located anywhere in the world. IPFS knows how to find what you ask for by its content address, rather than where it is.'... learn more from https://docs.ipfs.io/introduction/ ")
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
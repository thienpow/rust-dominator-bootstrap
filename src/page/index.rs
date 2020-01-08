use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};
use crate::routing::Route;

use crate::page::{IndexHeader, IndexSec1, IndexSec2};

pub struct IndexPage {
    index_header: Rc<IndexHeader>,
    index_sec_1: Rc<IndexSec1>,
    index_sec_2: Rc<IndexSec2>,
}

impl IndexPage {

    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            index_header: IndexHeader::new(),
            index_sec_1: IndexSec1::new(),
            index_sec_2: IndexSec2::new(),
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
            .visible_signal(Route::signal().map(move |x| x==Route::Index))
            .children(&mut [
                IndexHeader::render(state.index_header.clone()),
                IndexSec1::render(state.index_sec_1.clone()),
                IndexSec2::render(state.index_sec_2.clone()),
            ])
        })
    }
}
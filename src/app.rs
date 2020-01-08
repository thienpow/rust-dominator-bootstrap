use std::rc::Rc;
use lazy_static::lazy_static;
use futures_signals::signal::{Signal, SignalExt, Mutable};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec};
use dominator::{Dom, clone, class, html, stylesheet};

use crate::navbar::{NavBar};
use crate::footer::{Footer};
use crate::routing::Route;
use crate::page::{IndexPage, DocsPage, PricingPage, SignupPage};

pub struct App {
  navbar: Rc<NavBar>,
  footer: Rc<Footer>,
  index_page: Rc<IndexPage>,
  docs_page: Rc<DocsPage>,
  pricing_page: Rc<PricingPage>,
  signup_page: Rc<SignupPage>,
}

impl App {

    pub fn new() -> Rc<Self> {

        Rc::new(Self {
            navbar: NavBar::new(),
            footer: Footer::new(),
            index_page: IndexPage::new(),
            docs_page: DocsPage::new(),
            pricing_page: PricingPage::new(),
            signup_page: SignupPage::new(),
        })
    }


    pub fn render(state: Rc<Self>) -> Dom {
        
        /*
        stylesheet!("html, body", {
            .style("width", "100%")
            .style("height", "100%")
            .style("margin", "0px")
            .style("padding", "0px")
        });
        */

        lazy_static! {
            static ref ROOT_CLASS: String = class! {
                .style("overflow-x", "hidden")
                .style("background-color", "#323334")
            };
        }
        
        let navbar = state.navbar.clone();
        let footer = state.footer.clone();

        html!("div", {
            .class(&*ROOT_CLASS)

            .children(&mut [

                NavBar::render(navbar),
                
                html!("div", {
                    .children_signal_vec( 
                        Route::signal().map( move |x| 
                            match x {
                                Route::Index => vec![IndexPage::render(state.index_page.clone())],
                                Route::Docs => vec![DocsPage::render(state.docs_page.clone())],
                                Route::Pricing => vec![PricingPage::render(state.pricing_page.clone())],
                                Route::Signup => vec![
                                    SignupPage::render(state.signup_page.clone()),
                                    html!("div", {
                                        .text("Signup!")
                                    })
                                ]
                            }
                        ).to_signal_vec()
                    )
                }),

                Footer::render(footer),
                
            ])
        })
    }
}

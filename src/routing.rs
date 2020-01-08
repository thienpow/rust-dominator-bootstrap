use wasm_bindgen::prelude::*;
use web_sys::Url;
use futures_signals::signal::{Signal, SignalExt};
use dominator::routing;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Index,
    Docs,
    Pricing,
    Signup,
}

impl Route {

    pub fn signal() -> impl Signal<Item = Self> {
        routing::url()
            .signal_ref(|url| Url::new(&url).unwrap_throw())
            .map(|url| {
                match url.hash().as_str() {
                    "#/" => Route::Index,
                    "#/docs" => Route::Docs,
                    "#/pricing" => Route::Pricing,
                    "#/signup" => Route::Signup,
                    _ => Route::Index,
                }
            })
    }

    pub fn url(&self) -> &'static str {
        match self {
            Route::Index => "#/",
            Route::Docs => "#/about",
            Route::Pricing => "#/pricing",
            Route::Signup => "#/signup"
        }
    }
    
}
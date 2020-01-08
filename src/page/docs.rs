use std::rc::Rc;
use lazy_static::lazy_static;
use dominator::{Dom, html};
use futures_signals::signal::{Signal, SignalExt, Mutable};

use crate::routing::Route;


pub struct DocsPage {
  //navbar_isopen: Mutable<bool>,
}

impl DocsPage {
  
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
            //.style("height", "1200px")
            //.style("color", "white")
            //.text("docs here.")
            //.children(&mut [

                /*
                <div class="container-fluid">
                    <div class="flex-xl-nowrap row">
                        <div class="bd-sidebar col-12 col-md-3 col-xl-2">
                            <nav class="collapse bd-links" id="bd-docs-nav">
                                <div class="bd-toc-item active">
                                    <a class="bd-toc-link" href="#/documentation/introduction">Getting started</a>
                                    <ul class="bd-sidenav nav">
                                        <li class="active bd-sidenav-active"><a href="#/documentation/introduction">Introduction</a></li>
                                        <li class=""><a href="#/documentation/build-tools">BuildTools</a></li>
                                        <li class=""><a href="#/documentation/license">License</a></li>
                                        <li class=""><a href="#/documentation/file-structure">File Structure</a></li>
                                    </ul>
                                </div>
                                <div class="bd-toc-item active">
                                    <a class="bd-toc-link" href="#/documentation/footers">Core Components</a>
                                    <ul class="bd-sidenav nav">
                                        <li class=""><a href="#/documentation/footers">Footers</a></li>
                                        <li class=""><a href="#/documentation/headers">Headers</a></li>
                                        <li class=""><a href="#/documentation/navbars">Navbars</a></li>
                                        <li class=""><a href="#/documentation/parallax">Parallax</a></li>
                                    </ul>
                                </div>
                                <div class="bd-toc-item active">
                                    <a class="bd-toc-link" href="#/documentation/alerts">Components</a>
                                    <ul class="bd-sidenav nav">
                                        <li class=""><a href="#/documentation/alerts">Alerts</a></li>
                                        <li class=""><a href="#/documentation/badge">Badge</a></li>
                                        <li class=""><a href="#/documentation/breadcrumb">Breadcrumb</a></li>
                                        <li class=""><a href="#/documentation/buttons">Buttons</a></li>
                                        <li class=""><a href="#/documentation/card">Card</a></li>
                                        <li class=""><a href="#/documentation/carousel">Carousel</a></li>
                                        <li class=""><a href="#/documentation/colors">Colors</a></li>
                                        <li class=""><a href="#/documentation/dropdowns">Dropdowns</a></li>
                                        <li class=""><a href="#/documentation/forms">Forms</a></li>
                                        <li class=""><a href="#/documentation/Modal">Modal</a></li>
                                        <li class=""><a href="#/documentation/navbar">Navbar</a></li>
                                        <li class=""><a href="#/documentation/pagination">Pagination</a></li>
                                        <li class=""><a href="#/documentation/popovers">Popovers</a></li>
                                        <li class=""><a href="#/documentation/progress">Progress</a></li>
                                        <li class=""><a href="#/documentation/tooltips">Tooltips</a></li>
                                        <li class=""><a href="#/documentation/typography">Typography</a></li>
                                    </ul>
                                </div>
                                <div class="bd-toc-item active">
                                    <a class="bd-toc-link" href="#/documentation/alerts">Plugins</a>
                                    <ul class="bd-sidenav nav">
                                        <li class=""><a href="#/documentation/nouislider">Nouislider</a></li>
                                        <li class=""><a href="#/documentation/nucleo-icons">Nucleo Icons</a></li>
                                        <li class=""><a href="#/documentation/react-bootstrap-switch">React Bootstrap Switch</a></li>
                                        <li class=""><a href="#/documentation/react-datetime">React Datetime</a></li>
                                    </ul>
                                </div>
                            </nav>
                        </div>
                        <div class="d-none d-xl-block bd-toc col-12 col-xl-2"></div>


                        <main class="py-md-3 pl-md-5 bd-content col-12 col-md-9 col-xl-8">
                            <h1 class="bd-title" id="content">Introduction</h1>
                            <p class="bd-lead">Paper Kit React is a freeby Bootstrap 4, React and Reactstrap UI Kit.</p>
                            <h2 id="quick-start">Quick start</h2>
                            <p>To start using the UI Kit you will need to import some files in your current project or start from scratch using our template (scroll down in this page to view it).</p>
                            <h3 id="css">CSS</h3>
                            <p>Copy-paste the stylesheet into your <code class="highlighter-rouge">index.js</code> file before all other stylesheets to load our CSS.</p>
                            <pre style="color: black; background-color: rgb(245, 242, 240); text-shadow: white 0px 1px; font-family: Consolas, Monaco, &quot;Andale Mono&quot;, &quot;Ubuntu Mono&quot;, monospace; text-align: left; white-space: pre; word-spacing: normal; word-break: normal; word-wrap: normal; line-height: 1.5; tab-size: 4; -webkit-hyphens: none; padding: 1em; margin: 0.5em 0px; overflow: auto; background-position: initial initial; background-repeat: initial initial;">
                                
                                <code style="color: black; background-image: none; text-shadow: white 0px 1px; font-family: Consolas, Monaco, &quot;Andale Mono&quot;, &quot;Ubuntu Mono&quot;, monospace; text-align: left; white-space: pre; word-spacing: normal; word-break: normal; word-wrap: normal; line-height: 1.5; tab-size: 4; -webkit-hyphens: none; background-position: initial initial; background-repeat: initial initial;">
                                    <span class="token" style="color: slategray;">// styles</span>
                                    <span class="token module" style="color: rgb(0, 119, 170);">import</span> 
                                    <span class="token" style="color: rgb(102, 153, 0);">"assets/css/bootstrap.min.css"</span>
                                    <span class="token" style="color: rgb(153, 153, 153);">;</span>
                                    <span class="token module" style="color: rgb(0, 119, 170);">import</span> 
                                    <span class="token" style="color: rgb(102, 153, 0);">"assets/css/paper-kit.css"</span>
                                    <span class="token" style="color: rgb(153, 153, 153);">;</span>
                                    <span class="token" style="color: slategray;">// import "assets/css/paper-kit.min.css";</span>
                                    <span class="token" style="color: slategray;">// import "assets/css/paper-kit.css.map";</span>
                                    <span class="token module" style="color: rgb(0, 119, 170);">import</span> 
                                    <span class="token" style="color: rgb(102, 153, 0);">"assets/demo/demo.css"</span>
                                    <span class="token" style="color: rgb(153, 153, 153);">;</span>

                                </code>
                            
                            </pre>
                        </main>
                    </div>
                </div>
                */
                
            //])
        })
    }
}
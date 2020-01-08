
#[macro_export]
macro_rules! video {
  ($src:expr, { $($rest:tt)* }) => {

      html!("div", {
          .class(["embed-responsive", "embed-responsive-16by9", "mobile-min"])
          .children(&mut [
              
              html!("video", {
                  .class("embed-responsive-item")
                  .attribute("width", "560px")
                  .attribute("height", "315px")
                  .children(&mut [
                      html!("source", {
                          .attribute("src", $src)
                          .attribute("type", "video/mp4")
                          
                      })
                  ])
                  $($rest)*
              })
          ])
      })
  }
}


#[macro_export]
macro_rules! index_section {
    ($title:expr, { $($rest:tt)* }) => {
        html!("div", {
            .class(["section", "section-dark"])
            .children(&mut [

                html!("div", {
                    .class("row")
                    .children(&mut [

                        html!("div", {
                            .class("d-id")
                            .children(&mut [
                                html!("span", {.text($title) }),
                                html!("img", { .attribute("src", "assets/img/title_block.svg") })
                            ])
                        })
                        
                    ])
                }),

                html!("div", { .style("height", "10px") }),

                html!("div", {
                    .class("row")
                    .style("padding-left", "7%")
                    .style("padding-right", "7%")
                    $($rest)*
                })
            ])
        })
    }
}


#[macro_export]
macro_rules! column_one {
    ($title:expr, $desc:expr, { $($rest:tt)* }) => {

        html!("div", {
            .class(["col", "col-md", "title"])
            .children(&mut [

                html!("div", {
                    .class("d-id-big")
                    .text($title)
                }),
                html!("div", {
                    .class("title-big")
                    .text($desc)
                }),
                html!("div", {
                    .class("title-medium")
                    $($rest)*
                }),
            ])
        })

    }
}

#[macro_export]
macro_rules! column_two {
    ({ $($rest:tt)* }) => {
        html!("div", {
            .class(["col", "col-md"])
            .style("padding-left", "2%")
            .style("padding-right", "2%")
            $($rest)*
        })
    }
}

#[macro_export]
macro_rules! label {
    ($text:expr) => {
        html!("label", {
            .text($text)
        })
    }
}

//! Gallery

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct GalleryProperties {
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
}

/// Gallery layout
///
/// > Use a **Gallery** layout to arrange content in a responsive grid. Content will wrap responsively to create uniform rows and columns.
///
/// See: <https://www.patternfly.org/v4/layouts/gallery>
///
/// ## Properties
///
/// Defined by [`GalleryProperties`].
#[function_component(Gallery)]
pub fn gallery(props: &GalleryProperties) -> Html {
    let mut classes = classes!("pf-l-gallery");

    if props.gutter {
        classes.push(classes!("pf-m-gutter"));
    }

    html! (
        <div class={classes}>
        { for props.children.iter().map(|child|{
            html!{
                <div class="pf-l-gallery__item">
                    { child }
                </div>
            }
        }) }
        </div>
    )
}
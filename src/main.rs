mod block;
mod chain;

use crate::chain::Chain;
use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(APP)
}

static APP: Component<()> = |cx| {
    use_context_provider(&cx, || Chain::new());

    let chain = use_context::<Chain>(&cx)?;
    let mut draft = use_state(&cx, || "".to_string());

    let submit_block = move || {
        chain.write().add_block(draft.to_string());
        draft.set("".to_string());
    };

    rsx!(cx, div {
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { [include_str!("./style.css")] }
        header {
            i { class: "material-icons inventory", "inventory" }
            h1 {"Blocks: " [format!("{}", chain.read().get_total() )]}
        }
        main {
            section {
                input {
                    class:"new-block",
                    placeholder:"Add block data",
                    autofocus:"true",
                    value: "{draft}",
                    oninput: move |evt| draft.set(evt.value.clone()),
                    onkeydown : move |evt| {
                        if evt.key == "Enter" {
                            submit_block();
                        }
                    }
                }
            }
            section {
                ul {
                    class:"block-list",
                    chain.read().get_blocks().iter().map(|block| rsx!(
                        BlockItem {
                            key:"{block.index}",
                            id: block.index
                        }
                    ))
                }
            }
        }
    })
};

#[derive(PartialEq, Props)]
pub struct BlockItemProps {
    id: usize,
}

pub fn BlockItem(cx: Scope<BlockItemProps>) -> Element {
    let chain = use_context::<Chain>(&cx)?;
    let block = chain.read().get_block(cx.props.id)?;
    let hash = block.get_hash();
    cx.render(rsx! {
        li{
            strong {
                "{block.data}"
            }
            p {
                "{hash}"
            }
            em {
                "previous: {block.previous_hash}"
            }
        }
    })
}

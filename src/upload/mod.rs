mod theme;
mod upload_dragger;

use crate::mount_style;
use leptos::*;
pub use theme::UploadTheme;
pub use upload_dragger::UploadDragger;
pub use web_sys::FileList;

#[component]
pub fn Upload(
    #[prop(optional, into)] accept: MaybeSignal<String>,
    #[prop(optional, into)] multiple: MaybeSignal<bool>,
    #[prop(optional, into)] custom_request: Option<Callback<FileList, ()>>,
    children: Children,
) -> impl IntoView {
    mount_style("upload", include_str!("./upload.css"));

    let on_file_addition = move |files: FileList| {
        if let Some(custom_request) = custom_request {
            custom_request.call(files);
        }
    };
    let input_ref = create_node_ref::<html::Input>();
    let on_change = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            if let Some(files) = input_ref.files() {
                on_file_addition(files);
            }
        }
    };
    let on_click = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            input_ref.click();
        }
    };

    let is_trigger_dargover = create_rw_signal(false);
    let on_trigger_drop = move |event: ev::DragEvent| {
        event.prevent_default();
        if let Some(data) = event.data_transfer() {
            if let Some(files) = data.files() {
                on_file_addition(files);
            }
        }
        is_trigger_dargover.set(false);
    };
    let on_trigger_dragover = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dargover.set(true);
    };
    let on_trigger_dragenter = move |event: ev::DragEvent| {
        event.prevent_default();
    };
    let on_trigger_dragleave = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dargover.set(false);
    };

    view! {
        <div class="thaw-upload">
            <input
                class="thaw-upload__input"
                class=("thaw-upload--drag-over", move || is_trigger_dargover.get())
                ref=input_ref
                type="file"
                accept=move || accept.get()
                multiple=move || multiple.get()
                on:change=on_change
            />
            <div
                class="thaw-upload__trigger"
                on:click=on_click
                on:drop=on_trigger_drop
                on:dragover=on_trigger_dragover
                on:dragenter=on_trigger_dragenter
                on:dragleave=on_trigger_dragleave
            >
                {children()}
            </div>
        </div>
    }
}
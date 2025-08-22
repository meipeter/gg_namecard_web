
use std::io::Cursor;

use image::RgbImage;
use leptos::mount::mount_to_body;
use base64::{ write::EncoderWriter};
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
use leptos::prelude::*;
use leptos::control_flow::Show;
use puddle_farm_api_client_openapi_client::apis::configuration::Configuration;
use puddle_farm_api_client_openapi_client::apis::default_api::{avatar_player_id_get, player_id_get};
use send_wrapper::SendWrapper;
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(1);
    let (if_valid,set_if_valid)=signal(true);
    let (id,set_id) = signal(String::new());
    let img_gen = Action::new(|id:&i64|{
        let id = id.to_owned();
        SendWrapper::new(async move {
            let a = avatar_player_id_get(&Configuration { base_path: "http://127.0.0.1:8080/api".to_string(), ..Default::default() }, id).await.unwrap().bytes().await.unwrap();
            let a = image::ImageReader::new(std::io::Cursor::new(a))
                    .with_guessed_format().unwrap()
                    .decode().unwrap()
                    .to_rgba8();
            let p = player_id_get(&Configuration { base_path: "http://127.0.0.1:8080/api".to_string(), ..Default::default() }.clone(), id).await.unwrap();
            let i = gg_namecard_gen::generate_gg_namecard(p, a).unwrap();
            let mut img = Cursor::new(Vec::with_capacity(65536));
            i.write_to(&mut img, image::ImageFormat::Png);
            let res_base64 = base64::encode(&img.into_inner());
            res_base64


       })
    });
    let handle_submit = move |_| {
            let i = id.get().parse::<i64>().unwrap();
            img_gen.dispatch(i);};
    view! {
        <div class="flex items-center justify-center h-screen flex-col">

            <input type="text" class="input"

                    on:input:target=move |ev| {

                        set_id.set(ev.target().value());
                        if let Ok(v)= ev.target().value().parse::<i64>(){
                            set_if_valid.set(true);
                        }else {
                            set_if_valid.set(false);
                        };
                    }


                    prop:value=id
                />

                <p>
                    {move || if !if_valid.get() {
                       view!{<div class="alert alert-error">
                                  <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                  </svg>
                                  <span>Error! Task failed successfully.</span>
                                </div>}.into_any()
                    } else {
                        view!{}.into_any()
                    }}
                    </p>
                <button class="btn btn-success"  on:click=handle_submit class:btn-disabled= move || !if_valid.get()>"生成"</button>





        </div>
        {move || {
            if let Some(im) = img_gen.value().get(){
            view! {

                    <img
                        src=format!("data:image/png;base64,{}", im)

                    />
        }.into_any()}else{
            view! {}.into_any()
        }}}
    }
}

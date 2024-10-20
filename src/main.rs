#![allow(dead_code, unused_variables, unused_imports, unreachable_code)]
mod uuid;
mod network;
mod helpers;
mod protocols;

use esp_idf_svc::hal::delay::FreeRtos;
use core::cmp::Ordering;

use embedded_svc::{
    http::Method,
    io::Write,
    ws::FrameType,
};

use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    systime::EspSystemTime,
};

use esp_idf_svc::sys::{EspError, ESP_ERR_INVALID_SIZE};

use log::*;

use std::{borrow::Cow, collections::BTreeMap, str, sync::Mutex};
use esp_idf_sys::sleep;
use crate::network::access_point::{AccessPoint, AccessPointConfig};

static INDEX_HTML: &str = include_str!("http_ws_server_page.html");

#[derive(Debug, Copy, Clone)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    ap_ssid: &'static str,
    #[default("")]
    ap_password: &'static str,
    #[default(8)]
    ap_max_payload_len: usize,
    #[default(1024)]
    ap_stack_size: usize,
    #[default(11)]
    ap_wifi_channel: u8,
}

pub struct RelicClient {
    
}

fn main() -> anyhow::Result<()> {
    prepare();
    const CFG: Config = CONFIG;
    info!("App config: {:?}", CFG);
    
    let ap_config = AccessPointConfig {
        ssid: CFG.ap_ssid,
        password: CFG.ap_password,
        modem: Peripherals::take()?.modem,
        sys_loop: EspSystemEventLoop::take()?,
        nvs: EspDefaultNvsPartition::take()?,
        stack_size: CFG.ap_stack_size,
        wifi_channel: CFG.ap_wifi_channel,
    };
    let mut ap = AccessPoint::new(ap_config)?;
    
    ap.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?
            .write_all(INDEX_HTML.as_bytes())
            .map(|_| ())
    })?;
    let relics_map = Mutex::new(BTreeMap::<i32, RelicClient>::new());
    ap.ws_handler("/ws/guess", move |ws| {
        if ws.is_new() {
            ws.send(
                FrameType::Text(false),
                "Hi".as_bytes(),
            )?;
            return Ok(());
        } else if ws.is_closed() {
            return Ok(());
        }

        let (_frame_type, len) = match ws.recv(&mut []) {
            Ok(frame) => frame,
            Err(e) => return Err(e),
        };

        const MAX_LEN: usize = CFG.ap_max_payload_len;
        if len > MAX_LEN {
            ws.send(FrameType::Text(false), "Request too big".as_bytes())?;
            ws.send(FrameType::Close, &[])?;
            return Err(EspError::from_infallible::<ESP_ERR_INVALID_SIZE>());
        }
        
        let mut buf = [0; MAX_LEN]; // Small digit buffer can go on the stack
        ws.recv(buf.as_mut())?;
        let Ok(user_string) = str::from_utf8(&buf[..len]) else {
            ws.send(FrameType::Text(false), "[UTF-8 Error]".as_bytes())?;
            return Ok(());
        };
        FreeRtos::delay_ms(1000);
        ws.send(
            FrameType::Text(false),
            format!("Please enter a number between 1 and 100 ({})", user_string).as_bytes(),
        )?;
        return Ok(());
        
        // let mut relics = relics_map.lock().unwrap();
        // if ws.is_new() {
        //     relics.insert(ws.session(), RelicClient {});
        //     info!("New WebSocket session with relic. ({} open)", relics.len());
        //     ws.send(
        //         FrameType::Text(false),
        //         "Hi".as_bytes(),
        //     )?;
        //     return Ok(());
        // } else if ws.is_closed() {
        //     relics.remove(&ws.session());
        //     info!("Closed WebSocket session session with relic. ({} open)", relics.len());
        //     return Ok(());
        // }
        // let relic = relics.get_mut(&ws.session()).unwrap();
        // 
        // // NOTE: Due to the way the underlying C implementation works, ws.recv()
        // // may only be called with an empty buffer exactly once to receive the
        // // incoming buffer size, then must be called exactly once to receive the
        // // actual payload.
        // let (_frame_type, len) = match ws.recv(&mut []) {
        //     Ok(frame) => frame,
        //     Err(e) => return Err(e),
        // };
        // 
        // const MAX_LEN: usize = CFG.ap_max_payload_len;
        // if len > MAX_LEN {
        //     ws.send(FrameType::Text(false), "Request too big".as_bytes())?;
        //     ws.send(FrameType::Close, &[])?;
        //     return Err(EspError::from_infallible::<ESP_ERR_INVALID_SIZE>());
        // }
        // 
        // let mut buf = [0; MAX_LEN]; // Small digit buffer can go on the stack
        // ws.recv(buf.as_mut())?;
        // let Ok(user_string) = str::from_utf8(&buf[..len]) else {
        //     ws.send(FrameType::Text(false), "[UTF-8 Error]".as_bytes())?;
        //     return Ok(());
        // };
        // 
        // let Some(user_guess) = GuessingGame::parse_guess(user_string) else {
        //     ws.send(
        //         FrameType::Text(false),
        //         "Please enter a number between 1 and 100".as_bytes(),
        //     )?;
        //     return Ok(());
        // };
        // 
        // match relic.guess(user_guess) {
        //     (Ordering::Greater, n) => {
        //         let reply = format!("Your {} guess was too high", nth(n));
        //         ws.send(FrameType::Text(false), reply.as_ref())?;
        //     }
        //     (Ordering::Less, n) => {
        //         let reply = format!("Your {} guess was too low", nth(n));
        //         ws.send(FrameType::Text(false), reply.as_ref())?;
        //     }
        //     (Ordering::Equal, n) => {
        //         let reply = format!(
        //             "You guessed {} on your {} try! Refresh to play again",
        //             relic.secret,
        //             nth(n)
        //         );
        //         ws.send(FrameType::Text(false), reply.as_ref())?;
        //         ws.send(FrameType::Close, &[])?;
        //     }
        // }
        Ok::<(), EspError>(())
    })?;
    
    // Keep server running beyond when main() returns (forever)
    // Do not call this if you ever want to stop or access it later.
    // Otherwise you can either add an infinite loop so the main task
    // never returns, or you can move it to another thread.
    // https://doc.rust-lang.org/stable/core/mem/fn.forget.html
    core::mem::forget(ap);
    
    loop {
        unsafe { sleep(1); }
    }
    Ok(())
}
fn prepare() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_hal::sys::link_patches();
    info!("Ready");
}
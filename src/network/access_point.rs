use core::convert::TryInto;
use log::*;
use esp_idf_sys::EspError;

use anyhow::{
    Result,
    Error
};
use std::{
    fmt::Debug,
    str,
};

use embedded_svc::{
    http::Method,
    http::server::Request,
    wifi::{self, AccessPointConfiguration, AuthMethod},
};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::EspHttpServer,
    http::server::EspHttpConnection,
    http::server::ws::EspHttpWsConnection,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};

use esp_idf_hal::{
    modem::Modem,
    peripheral::Peripheral,
};

pub struct AccessPointConfig<'a, M: Peripheral<P=Modem>> {
    pub ssid: &'a str,
    pub password: &'a str,
    pub modem: M,
    pub sys_loop: EspSystemEventLoop,
    pub nvs: EspDefaultNvsPartition,
    pub stack_size: usize,
    pub wifi_channel: u8,
}

#[allow(dead_code)]
pub struct AccessPoint<'a> {
    wifi: BlockingWifi<EspWifi<'a>>,
    server: EspHttpServer<'a>,
}

impl<'a> AccessPoint<'a> {
    pub fn new<M>(
        config: AccessPointConfig<'a, M>
    ) -> Result<Self, Error>
    where
        M: Peripheral<P=Modem> + 'static
    {
        let wifi = wifi(
            config.ssid,
            config.password,
            config.modem,
            config.sys_loop,
            config.nvs,
            config.wifi_channel,
        )?;
        Ok(AccessPoint {
            wifi,
            server: server(config.stack_size)?,
        })
    }

    pub fn fn_handler<E, F>(
        &mut self,
        uri: &str,
        method: Method,
        f: F,
    ) -> std::result::Result<&mut Self, EspError>
    where
        F: for<'r> Fn(Request<&mut EspHttpConnection<'r>>) -> Result<(), E> + Send + 'static,
        E: Debug,
    {
        self.server.fn_handler(uri, method, f)?;
        Ok(self)
    }
    
    pub fn ws_handler<H, E>(&mut self, uri: &str, handler: H) -> Result<&mut Self, EspError>
    where
        H: for<'r> Fn(&'r mut EspHttpWsConnection) -> Result<(), E> + Send + Sync + 'a,
        E: Debug,
    {
        self.server.ws_handler(uri, handler)?;
        Ok(self)
    }
}

fn wifi<'w>(
    ssid: &str,
    password: &str,
    modem: impl Peripheral<P = Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
    channel: u8,
) -> Result<BlockingWifi<EspWifi<'w>>, Error> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;
    let wifi_configuration = wifi::Configuration::AccessPoint(AccessPointConfiguration {
        ssid: ssid.try_into().unwrap(),
        ssid_hidden: true,
        auth_method: AuthMethod::WPA2Personal,
        password: password.try_into().unwrap(),
        channel,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;
    wifi.start()?;
    wifi.wait_netif_up()?;
    info!(
        "Created Wi-Fi with WIFI_SSID `{}` and WIFI_PASS `{}`",
        ssid, password,
    );
    Ok(wifi)
}

fn server(stack_site: usize) -> Result<EspHttpServer<'static>, Error> {
    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: stack_site,
        ..Default::default()
    };
    let server = EspHttpServer::new(&server_configuration)?;
    info!("Created HTTP server with configuration {:?}", server_configuration);
    Ok(server)
}
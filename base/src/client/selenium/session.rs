use thirtyfour::{DesiredCapabilities, WebDriver};
use crate::client::conn::Conn;
use crate::error::InternalResult;

pub struct OrcaWebDriver;

impl OrcaWebDriver {
    pub async fn default() -> InternalResult<WebDriver> {
        Ok(Self::firefox().await?)
    }
    pub async fn chrome() -> InternalResult<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--headless").expect("TODO: panic message");
        caps.add_chrome_arg("--disable-gpu").expect("TODO: panic message");
        Ok(Conn::webdriver(caps).await?)
    }
    pub async fn firefox() -> InternalResult<WebDriver> {
        let mut caps = DesiredCapabilities::firefox();
        caps.add_firefox_arg("--headless").expect("TODO: panic message");
        caps.add_firefox_arg("--disable-gpu").expect("TODO: panic message");
        Ok(Conn::webdriver(caps).await?)
    }
    pub async fn edge() -> InternalResult<WebDriver> {
        let caps = DesiredCapabilities::edge();
        Ok(Conn::webdriver(caps).await?)
    }
}
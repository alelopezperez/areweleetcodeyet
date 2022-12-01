use std::{thread, time::Duration};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().unwrap();
    let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();

    driver
        .goto("https://leetcode.com/problemset/all/")
        .await
        .unwrap();

    let stack = driver
        .find(By::Id("headlessui-listbox-button-15"))
        .await
        .unwrap();

    stack.click().await.unwrap();

    driver
        .find(By::Id("headlessui-listbox-option-28"))
        .await
        .unwrap()
        .click()
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    let rowgroup = driver
        .find(By::XPath("//div[@role = 'rowgroup']"))
        .await
        .unwrap();

    let children = rowgroup
        .find_all(By::XPath("//div[@role = 'row']"))
        .await
        .unwrap();

    let url = driver.current_url().await.unwrap();

    for i in children {
        println!("{:?}", i.text().await.unwrap());
    }
}

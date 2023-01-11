use diesel::prelude::*;
use std::{thread, time::Duration, time::SystemTime};
use thirtyfour::{extensions::query::conditions, prelude::*};
use tokio::task::JoinHandle;
use web_scraper::{create_problem, establish_connection, models};

#[tokio::main]
async fn main() {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().unwrap();
    let mut driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();

    driver
        .goto("https://leetcode.com/problemset/all/")
        .await
        .unwrap();

    let nav;

    loop {
        match driver.find(By::XPath("//nav[@role = 'navigation']")).await {
            Ok(elem) => {
                nav = elem;
                break;
            }
            _ => {}
        }
    }

    let buttons; //= nav.find_all(By::Tag("button")).await.unwrap();

    loop {
        match nav.find_all(By::Tag("button")).await {
            Ok(elem) => {
                if elem.len() > 2 {
                    println!("{}", elem.len());
                    buttons = elem;
                    break;
                }
            }
            _ => {}
        }
    }

    for b in &buttons {
        println!("{}", b.text().await.unwrap())
    }
    let mut last_page = buttons.iter().rev().take(2);
    last_page.next();

    let last_page = last_page
        .next()
        .unwrap()
        .inner_html()
        .await
        .unwrap()
        .parse::<u32>()
        .unwrap();

    println!("Last Page {}", last_page);

    for i in 1..=last_page {
        let page_next = format!(
            "{}{}",
            "https://leetcode.com/problemset/all/?page=",
            i.to_string()
        );
        driver.goto(page_next).await.unwrap();

        let rowgroup;

        loop {
            match driver.find(By::XPath("//div[@role = 'rowgroup']")).await {
                Ok(elem) => {
                    rowgroup = elem;
                    break;
                }
                _ => {}
            }
        }
        let children = rowgroup
            .find_all(By::XPath("//div[@role = 'row']"))
            .await
            .unwrap();

        // let mut join_handle: Vec<JoinHandle<()>> = Vec::new();
        for child in children {
            if let Ok(a_href) = child.find(By::Tag("a")).await {
                if let Some(premium) = a_href.class_name().await.unwrap() {
                    if premium.contains("opacity") {
                        println!("is Premium");
                        continue;
                    }
                }

                match a_href.prop("href").await.unwrap() {
                    Some(link) => {
                        let name = a_href.text().await.unwrap();
                        if !name.is_empty() {
                            println!("len:{} Name:{}  Link:{}", name.len(), name, &link);

                            has_rust(name, link).await;
                        }
                    }
                    None => {}
                }
            }
        }

        // for join in join_handle {
        //     tokio::join!(join).0.unwrap();
        // }
    }
    driver.quit().await.unwrap();
}

async fn has_rust(name: String, link: String) {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().unwrap();

    let mut driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();

    driver.goto(&link).await.unwrap();

    let language_selector: WebElement;

    let start_query = SystemTime::now();
    loop {
        match driver.find(By::Css("button[id*='listbox']")).await {
            Ok(element) => {
                language_selector = element;
                break;
            }
            _ => {
                match start_query.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_secs() == 120 {
                            println!("SE HARA UN REFRESH");
                            match driver.refresh().await {
                                Ok(_) => {}
                                _ => {
                                    driver.quit().await.unwrap();
                                    let mut caps = DesiredCapabilities::chrome();
                                    caps.set_headless().unwrap();
                                    driver = WebDriver::new("http://localhost:9515", caps)
                                        .await
                                        .unwrap();
                                    driver.goto(&link).await.unwrap();
                                }
                            }
                        }
                    }
                    _ => {
                        // println!("Aun no?");
                    }
                }
            }
        }
    }

    language_selector.click().await.unwrap();

    let available_lang = driver.find_all(By::Css("li")).await.unwrap();

    println!("{}", available_lang.len());

    let mut yes = false;
    for lang in available_lang {
        if lang.text().await.unwrap() == "Rust" {
            println!("has rust");
            yes = true;
            break;
        }
    }

    use web_scraper::schema::problems::dsl::*;

    let connection = &mut establish_connection();

    let _ = create_problem(connection, &name, &link, &yes);
    driver.quit().await;
}

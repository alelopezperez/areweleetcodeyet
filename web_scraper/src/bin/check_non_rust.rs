use diesel::prelude::*;
use std::{thread, time::Duration, time::SystemTime};
use thirtyfour::{extensions::query::conditions, prelude::*};
use tokio::task::JoinHandle;
use web_scraper::{create_problem, get_last_problem, models};
use web_scraper::{establish_connection, get_false_problems, update_problem};

#[tokio::main]
async fn main() {
    let connection = &mut establish_connection();

    let problems = get_false_problems(connection);
    println!("Amount of false {}", problems.len());
    for problem in problems {
        has_rust(problem.id, problem.url).await;
    }
}

async fn has_rust(ids: i32, link: String) {
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

    if yes {
        update_problem(connection, ids);
    }
    driver.quit().await.unwrap();
}

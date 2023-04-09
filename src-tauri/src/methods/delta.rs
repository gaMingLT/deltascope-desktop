use std::time::Instant;

use crate::db::conn::db_con;
use crate::db::delta::{get_events_delta, get_events_json};
use crate::db::events::{get_events_image_values_neariest_date, input_values_events};
use crate::db::file_db::get_path_image_from_name;
use crate::db::files::input_values_files;
use crate::db::tables::{create_events_table, create_files_table};
use crate::tools::mactime::{execute_mactime, parse_mactime_lines, execute_mactime_wls ,MacTimeLine};
use crate::tools::rsfls::{execute_fls, parse_fls_lines, execute_fls_wsl};

pub async fn delta_images(
    out_path: String,
    images: Vec<String>,
    directory_name: String,
    use_wls: bool
) -> Result<(), ()> {
    log::info!("Delating two images!");
    // println!("Images: {:?}", images);
    // println!("Directory: {}", directory_name);

    let start = Instant::now();

    for name in images.into_iter() {
      let temp_path = out_path.clone();
      retrieve_info_image(temp_path.clone(), name.split('.').next().unwrap().to_string(), use_wls).await.unwrap();
    }

    // let mut handles = Vec::new();

    // for name in images.into_iter() {
    //     let temp_path = out_path.clone();

    //     // tokio::task::spawn(async move {
    //     //   retrieve_info_image(temp_path.clone(), name.split('.').next().unwrap().to_string()).await.unwrap();
    //     // });

    //     handles.push(tokio::task::spawn(async move {
    //         retrieve_info_image(
    //             temp_path.clone(),
    //             name.split('.').next().unwrap().to_string(),
    //         )
    //         .await
    //         .unwrap();
    //     }));
    // }

    // for handle in handles {
    //     let output = handle.await.expect("Execution failed!");
    // }

    // Before 32s
    // Now 28.XXs when using scn-1-nginx-chance - but database locked issue!
    log::info!("Elapsed: {:?}", start.elapsed());

    Ok(())
}

async fn retrieve_info_image(out_path: String, name: String, use_wls: bool) -> Result<(), ()> {
    log::info!("Deltaing images with name: {}", name);

    // Connect to database
    let conn = db_con(out_path.clone()).await.unwrap();

    sqlx::query("pragma temp_store = memory;")
        .execute(&conn)
        .await.unwrap();
    sqlx::query("pragma mmap_size = 30000000000;")
        .execute(&conn)
        .await.unwrap();
    sqlx::query("pragma page_size = 4096;")
        .execute(&conn)
        .await.unwrap();

    // Create tables
    // - Create files table
    let conn = create_files_table(name.clone(), conn).await.unwrap();

    // - Create events table
    let conn = create_events_table(name.clone(), conn).await.unwrap();

    // FLS
    // - Execute FLS
    let image_path = get_path_image_from_name(name.clone()).unwrap();

    let lines;

    if use_wls {
        lines = execute_fls_wsl(image_path, out_path.clone(), name.clone()).await.unwrap();
    }
    else {
        lines = execute_fls(image_path.clone(), out_path.clone(), name.clone())
            .await
            .unwrap();
    }

    // - Parse fls lines
    let parsed_files_data = parse_fls_lines(lines).unwrap();

    // - FLS Info into database
    let conn = input_values_files(name.clone(), parsed_files_data, &conn)
        .await
        .unwrap();

    // Mactime
    let mactime_lines;
    // - Execute Mactime
    if use_wls {
        mactime_lines = execute_mactime_wls(out_path, name.clone()).await.unwrap();
    } else {
        mactime_lines = execute_mactime(out_path.clone(), name.clone()).unwrap();
    }
    
    // - Parse Mactime lines
    let parsed_events_data = parse_mactime_lines(mactime_lines).unwrap();

    // - Mactime into database
    input_values_events(name.clone(), parsed_events_data, &conn)
        .await
        .unwrap();

    conn.close().await;

    log::info!("Finishing retrieving info from image: {}", name);

    Ok(())
}

#[derive(Debug)]
pub struct InfoEvents {
    image: String,
    date: String,
}

pub async fn get_events_images(
    images: Vec<String>,
    directory_path: String,
) -> Result<(Vec<MacTimeLine>, Vec<MacTimeLine>, Vec<MacTimeLine>), String> {
    let conn = db_con(directory_path.clone()).await.unwrap();
    let mut base_image_info: InfoEvents = InfoEvents {
        image: String::from(""),
        date: String::from(""),
    };
    let mut next_image_info: InfoEvents = InfoEvents {
        image: String::from(""),
        date: String::from(""),
    };

    for image in images.into_iter() {
        let new_image_name = image.split('.').next().unwrap().to_string();

        let date = get_events_image_values_neariest_date(new_image_name.clone(), conn.clone())
            .await
            .unwrap();

        if base_image_info.date.is_empty() && base_image_info.image.is_empty() {
            base_image_info = InfoEvents {
                image: new_image_name,
                date: date,
            };
        } else if date > base_image_info.date {
            next_image_info = InfoEvents {
                image: new_image_name,
                date: date,
            };
        } else if date < base_image_info.date {
            next_image_info = base_image_info;
            base_image_info = InfoEvents {
                image: new_image_name,
                date: date,
            };
        } else {
            continue;
        }
    }

    conn.close().await;

    let conn = db_con(directory_path.clone()).await.unwrap();

    let base_events = get_events_json(base_image_info.image.clone(), conn.clone())
        .await
        .unwrap();
    let next_events = get_events_json(next_image_info.image.clone(), conn.clone())
        .await
        .unwrap();
    let delta_events = get_events_delta(base_image_info.image, next_image_info.image, conn)
        .await
        .unwrap();

    Ok((base_events, next_events, delta_events))
}

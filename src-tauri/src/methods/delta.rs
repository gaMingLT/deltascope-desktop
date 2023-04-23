use std::sync::Arc;
use std::time::Instant;

use tokio::sync::Mutex;

use crate::db::conn::db_con;
use crate::db::delta::{get_events_delta, get_events_json};
use crate::db::events::{get_events_image_values_neariest_date, input_values_events};
use crate::db::file_db::get_path_image_from_name;
use crate::db::files::{get_files, input_values_files, Bodyfile3Line2};
use crate::db::tables::{create_events_table, create_files_table};
use crate::tools::files::retrieve_files_image;
use crate::tools::fls::{execute_fls, execute_fls_wsl, parse_fls_lines};
use crate::tools::mactime::{
    execute_mactime, execute_mactime_wls, parse_mactime_lines, MacTimeLine,
};

pub async fn delta_images(
    out_path: String,
    images: Vec<String>,
    directory_name: String,
    use_wls: bool,
) -> Result<(), ()> {
    log::info!("Delating two images!");

    let start = Instant::now();

    // let mut info: Vec<(String, Vec<Bodyfile3Line2>)> = vec![];

    // for name in images.clone().into_iter() {
    //     let temp_path = out_path.clone();
    //     let res = retrieve_info_image(
    //         temp_path.clone(),
    //         name.split('.').next().unwrap().to_string(),
    //         use_wls,
    //     )
    //     .await
    //     .unwrap();
    //     info.push((name, res));
    // }

    let mut handles = Vec::new();
    let mut info2 = Arc::new(Mutex::new(vec![]));

    for name in images.clone().into_iter() {
        let temp_path = out_path.clone();
        let info2 = info2.clone();

        handles.push(tokio::task::spawn(async move {
            // let temp_path = out_path.clone();
            let res = retrieve_info_image(
                temp_path.clone(),
                name.split('.').next().unwrap().to_string(),
                use_wls,
            )
            .await
            .unwrap();

            info2.lock().await.push((name, res));
        }));
    }

    for handle in handles {
        let output = handle.await.expect("Execution failed!");
    }

    log::debug!("Getting results (threading)");
    let mut new_info = vec![];

    let res = info2.lock().await;
    // let test = &res.get(0).unwrap().0;
    // let test2 = &res.get(1).unwrap().0;
    // println!("Res (threading): {:?} {:?}", test, test2);

    new_info.push(res.get(0).unwrap().clone());
    new_info.push(res.get(1).unwrap().clone());

    // let conn = Arc::new(Mutex::new(db_con(out_path.clone())));
    let conn = db_con(out_path.clone()).await.unwrap();
    let mut base_image_info: InfoEvents = InfoEvents {
        image: String::from(""),
        date: String::from(""),
    };
    let mut next_image_info: InfoEvents = InfoEvents {
        image: String::from(""),
        date: String::from(""),
    };

    for image in images.clone().into_iter() {
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

    log::debug!("Next info: {:?}", next_image_info);
    log::debug!("Base info: {:?}", base_image_info);

    let mut next_result = vec![];
    let mut base_result = vec![];

    for results in new_info.into_iter() {
        let new_image_name = results.0.split('.').next().unwrap().to_string();
        if new_image_name == next_image_info.image {
            next_result = results.1;
        } else if new_image_name == base_image_info.image {
            base_result = results.1;
        }
    }

    log::debug!("Next Result: {:?}", next_result.len());
    log::debug!("Base result: {:?}", base_result.len());

    let result_files = compare_hash_path(base_result, next_result).unwrap();

    retrieve_files_image(images, out_path, result_files);

    // Before 32s
    // Now 28.XXs when using scn-1-nginx-chance - but database locked issue!
    log::info!("Elapsed: {:?}", start.elapsed());

    Ok(())
}

async fn retrieve_info_image(
    out_path: String,
    name: String,
    use_wls: bool,
) -> Result<Vec<Bodyfile3Line2>, ()> {
    log::info!("Deltaing images with name: {}", name);

    // Connect to database
    let conn = db_con(out_path.clone()).await.unwrap();

    sqlx::query("pragma temp_store = memory;")
        .execute(&conn)
        .await
        .unwrap();
    sqlx::query("pragma mmap_size = 30000000000;")
        .execute(&conn)
        .await
        .unwrap();
    sqlx::query("pragma page_size = 4096;")
        .execute(&conn)
        .await
        .unwrap();

    sqlx::query("PRAGMA journal_mode=WAL;")
        .execute(&conn)
        .await
        .unwrap();

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
        lines = execute_fls_wsl(image_path, out_path.clone(), name.clone())
            .await
            .unwrap();
    } else {
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

    //Execute Mactime
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

    let res = get_files(name.clone(), &conn).await.unwrap();

    conn.close().await;

    log::info!("Finishing retrieving info from image: {}", name);

    Ok(res)
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
    let start = Instant::now();

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

    log::info!("Elapsed: {:?}", start.elapsed());

    Ok((base_events, next_events, delta_events))
}

#[derive(Debug)]
pub struct MappingFiles {
    pub same: Vec<Bodyfile3Line2>,
    pub deleted: Vec<Bodyfile3Line2>,
    pub modified: Vec<Bodyfile3Line2>,
    pub moved: Vec<Bodyfile3Line2>,
    pub new: Vec<Bodyfile3Line2>,
    pub swap: Vec<Bodyfile3Line2>,
    pub ignore: Vec<Bodyfile3Line2>,
}

pub fn compare_hash_path(
    base_files: Vec<Bodyfile3Line2>,
    next_files: Vec<Bodyfile3Line2>,
) -> Result<MappingFiles, ()> {
    log::info!("Comparing hash and path");

    let mut differences = MappingFiles {
        same: vec![],
        deleted: vec![],
        modified: vec![],
        moved: vec![],
        new: vec![],
        swap: vec![],
        ignore: vec![],
    };

    log::debug!("Next files (len): {}", next_files.len());
    log::debug!("Base files (len): {}", base_files.len());

    let start = Instant::now();

    let filtered_next = next_files
        .clone()
        .into_iter()
        .filter(|f| f.md5 != String::from("0"))
        .filter(|f| {
            f.mode_as_string
                .chars()
                .collect::<Vec<char>>()
                .get(0)
                .unwrap()
                .clone()
                == "r".chars().next().unwrap()
        })
        .filter(|f| !f.name.contains("deleted"))
        .filter(|f| !f.name.contains("/usr/lib"))
        .filter(|f| !f.name.contains("/usr/bin"))
        .collect::<Vec<Bodyfile3Line2>>();

    let filtered_base = base_files
        .clone()
        .into_iter()
        .filter(|f| f.md5 != String::from("0"))
        .filter(|f| {
            f.mode_as_string
                .chars()
                .collect::<Vec<char>>()
                .get(0)
                .unwrap()
                .clone()
                == "r".chars().next().unwrap()
        })
        .filter(|f| !f.name.contains("deleted"))
        .filter(|f| !f.name.contains("/usr/lib"))
        .filter(|f| !f.name.contains("/usr/bin"))
        .collect::<Vec<Bodyfile3Line2>>();

    log::info!("Elapsed (filter): {:?}", start.elapsed());

    let start = Instant::now();

    for next_row in filtered_next.iter() {
        for base_row in filtered_base.iter() {
            if next_row.name == base_row.name && next_row.name.contains("deleted") {
                if next_row.name.contains("deleted") {
                    differences.deleted.push(next_row.clone());
                } else if next_row.name.contains("/usr/lib/") {
                    differences.ignore.push(next_row.clone());
                }
            } else if next_row.md5 == base_row.md5 && next_row.name == base_row.name {
                differences.same.push(next_row.clone());
            } else if next_row.md5 != base_row.md5 && next_row.name == base_row.name {
                differences.modified.push(next_row.clone());
            }
        }
    }

    log::info!("Elapsed (iterate): {:?}", start.elapsed());

    log::debug!("Differences (same): {:?}", differences.same.len());
    log::debug!("Differences (modified): {:?}", differences.modified);
    log::debug!("Differences (moved): {:?}", differences.moved.len());

    Ok(differences)
}

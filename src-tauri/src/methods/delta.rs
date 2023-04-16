use std::time::Instant;

use crate::db::conn::db_con;
use crate::db::delta::{get_events_delta, get_events_json};
use crate::db::events::{get_events_image_values_neariest_date, input_values_events};
use crate::db::file_db::get_path_image_from_name;
use crate::db::files::{get_files, input_values_files, Bodyfile3Line2};
use crate::db::tables::{create_events_table, create_files_table};
use crate::tools::mactime::{
    execute_mactime, execute_mactime_wls, parse_mactime_lines, MacTimeLine,
};
use crate::tools::rsfls::{execute_fls, execute_fls_wsl, parse_fls_lines};
use crate::tools::files::{retrieve_files_image};

pub async fn delta_images(
    out_path: String,
    images: Vec<String>,
    directory_name: String,
    use_wls: bool,
) -> Result<(), ()> {
    log::info!("Delating two images!");
    // println!("Images: {:?}", images);
    // println!("Directory: {}", directory_name);

    let start = Instant::now();

    let mut info: Vec<(String, Vec<Bodyfile3Line2>)> = vec![];

    for name in images.clone().into_iter() {
        let temp_path = out_path.clone();
        let res = retrieve_info_image(
            temp_path.clone(),
            name.split('.').next().unwrap().to_string(),
            use_wls,
        )
        .await
        .unwrap();
        info.push((name, res));
    }

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

    let mut next_result = vec![];
    let mut base_result = vec![];

    for results in info.into_iter() {
        let new_image_name = results.0.split('.').next().unwrap().to_string();
        if new_image_name == next_image_info.image {
            next_result = results.1;
        } else if new_image_name == base_image_info.image {
            base_result = results.1;
        }
    }

    let result_files = compare_hash_path(base_result, next_result).unwrap();

    retrieve_files_image(images,  out_path ,result_files);

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

    let res = get_files(name.clone(), &conn).await.unwrap();

    println!("Length: {}", res.len());

    conn.close().await;

    log::info!("Finishing retrieving info from image: {}", name);

    // Ok(())
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
    };

    println!("Len next: {:?}", next_files.len());
    println!("Base next: {:?}", base_files.len());

    let filtered_next = next_files
        .clone()
        .into_iter()
        .filter(|f| f.md5 != String::from("0"))
        .filter(|f| f.mode_as_string.chars().collect::<Vec<char>>().get(0).unwrap().clone() == "r".chars().next().unwrap())
        .collect::<Vec<Bodyfile3Line2>>();
    println!("Len next new: {:?}", filtered_next.len());

    let filtered_base = base_files
        .clone()
        .into_iter()
        .filter(|f| f.md5 != String::from("0"))
        .filter(|f| f.mode_as_string.chars().collect::<Vec<char>>().get(0).unwrap().clone() == "r".chars().next().unwrap())
        .collect::<Vec<Bodyfile3Line2>>();
    println!("Len next new: {:?}", filtered_base.len());

    for next_row in filtered_next.iter() {
        for base_row in filtered_base.iter() {
            if next_row.md5 == base_row.md5 && next_row.name == base_row.name {
                differences.same.push(next_row.clone());
            }
            else if next_row.md5 != base_row.md5 &&  next_row.name == base_row.name {
                differences.modified.push(next_row.clone());
            }
        }
    }

    println!("Differences (same): {:?}", differences.same.len());
    println!("Differences (modified): {:?}", differences.modified);
    println!("Differences (moved): {:?}", differences.moved.len());

    // for next_row in next_files.iter() {

    //     for base_row in base_files.iter() {
    //         // println!("Row: {:?} {:?}    {:?} {:?}", base_row.clone().md5 ,base_row.clone().name, next_row.clone().md5 , next_row.clone().name);
    //         // println!("Same: {:?}",  next_row.md5 == base_row.md5 && next_row.name == base_row.name);
    //         // println!("Modified: {:?}",  next_row.md5 != base_row.md5 &&  next_row.name == base_row.name);
    //         if next_row.md5 == base_row.md5 && next_row.name == base_row.name {
    //             differences.same.push(next_row.clone());
    //         }
    //         else if next_row.md5 == base_row.md5 && next_row.name != base_row.name {
    //             differences.moved.push(next_row.clone());
    //         }
    //         else if next_row.md5 != base_row.md5 &&  next_row.name == base_row.name {
    //             differences.modified.push(next_row.clone());
    //         }
    //     }
    // }

    // println!("Differences: {:?}", differences.modified);

    Ok(differences)
}

// def compare_hash_path(data, con):
//   main_logger.info('[METHODS] - Comparing hash and path')
//   hashAndPathImages = []

//   for img in data:
//     hashPathMode = []
//     for row in img[1]:
//       hashPathMode.append((row[0],row[1], row[2] ,row[3]))
//     hashAndPathImages.append((img[0], hashPathMode))

//   deltas = []
//   deltaImage = { 'delta': hashAndPathImages[0][0], 'next': hashAndPathImages[1][0], 'differences': {} }
//   differences = { 'same': [] ,'deleted': [], 'modified': [], 'moved': [] , 'new': [], 'swap': [] }

//   baseImage = hashAndPathImages[0][1]
//   nextImage =  hashAndPathImages[1][1]
//   basePaths = []

//   for nextRow in nextImage:
//     nextHash = nextRow[0]
//     nextPath = nextRow[1]

//     if 'deleted' in nextPath:
//       differences['swap'].append(nextRow)

//     for baseRow in baseImage:
//       baseHash = baseRow[0]
//       basePath = baseRow[1]

//       if baseHash == nextHash and basePath == nextPath:
//         differences['same'].append(nextRow)

//       if baseHash != nextHash and basePath == nextPath:
//         basePaths.append(basePath)
//         differences['modified'].append(nextRow)

//     if nextRow not in baseImage and 'deleted' not in nextPath and nextRow[1] not in basePaths:
//       # TODO: Moved files are shown as new here - hash als changes :(
//       differences['new'].append(nextRow)

//   deltaImage['differences'] = differences
//   return deltaImage

// def retrieve_files_from_image(deltas, out: str):
//   main_logger.info('[METHODS] - Retrieving ``modified`` files from image')
//   print('New files: ', deltas['differences']['new'])
//   modified = deltas['differences']['modified']
//   modifiedFilePaths = []

//   print('Modified: ', modified)

//   for mod in modified:
//     fileOrDir = mod[3].split('/')[0]

//     if fileOrDir == 'd':
//       pass
//     elif fileOrDir == 'r':
//       modifiedFilePaths.append(mod)
//     else:
//       pass

//   imageNames = [deltas['delta'], deltas['next']]

//   if not path.exists('{0}/{1}'.format(out,'icat')):
//     mkdir('{0}/{1}'.format(out,'icat'))

//   differentPathNames = {}

//   for name in imageNames:
//     for mod in modifiedFilePaths:
//       pathName = mod[1].split('/')[-1].split('.')[0]
//       fileName = mod[1].split('/')[-1]
//       if fileName in differentPathNames:
//         differentPathNames[fileName].append("{0}-{2}-{1}.txt".format(name.replace('_','-'), mod[2] ,pathName))
//       else:
//         differentPathNames[fileName] = ["{0}-{2}-{1}.txt".format(name.replace('_','-'), mod[2] ,pathName)]

//       srcPath = "/mnt/img-store/scn-1/images"
//       cmd = "icat {4}/{0}.img {1} > {2}/icat/{0}-{3}-{1}.txt".format(name.replace('_','-'), mod[2], out, pathName, srcPath)

//       res = system(cmd)

//       if res == 0:
//         main_logger.debug('[METHODS] - Retrieving file succesfull!')

//   if not path.exists('{0}/{1}'.format(out,'diff')):
//     mkdir('{0}/{1}'.format(out,'diff'))

//   for key in differentPathNames.keys():
//     paths = differentPathNames[key]
//     diffFileName = "{0}.txt".format(key.split('.')[0])
//     cmd = "diff -u {0}/icat/{1} {0}/icat/{2} > {0}/diff/{3}".format(out, paths[0], paths[1], diffFileName)
//     res = system(cmd)
//     if res == 0:
//       main_logger.debug('[METHODS] - Diffing of files succesfull!')

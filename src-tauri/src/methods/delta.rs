use std::time::Instant;

use crate::db::conn::{db_con};
use crate::db::file_db::{get_path_image_from_name};
use crate::db::tables::{create_events_table, create_files_table};
use crate::db::files::{input_values_files};
use crate::db::events::{input_values_events, get_events_image_values_neariest_date};
use crate::tools::rsfls::{execute_fls, parse_fls_file, parse_fls_lines};
use crate::tools::mactime::{execute_mactime, parse_mactime_lines, MacTimeLine};
use crate::db::delta::{get_events_json, get_events_delta};

pub async fn delta_images(out_path: String, images: Vec<String>, directory_name: String) -> Result<(), ()> {
  println!("Delating two images!");
  // println!("Images: {:?}", images);
  // println!("Directory: {}", directory_name);

  let start = Instant::now();

  for name in images.into_iter() {
    let temp_path = out_path.clone();
    retrieve_info_image(temp_path.clone(), name.split('.').next().unwrap().to_string()).await.unwrap(); 
  }

  // let mut handles = Vec::new();

  // for name in images.into_iter() {
  //   let temp_path = out_path.clone();
    
  //   // tokio::task::spawn(async move {
  //   //   retrieve_info_image(temp_path.clone(), name.split('.').next().unwrap().to_string()).await.unwrap();
  //   // });

  //   handles.push(
  //        tokio::task::spawn(async move {
  //     retrieve_info_image(temp_path.clone(), name.split('.').next().unwrap().to_string()).await.unwrap();
  //   }) 
  //   );

  // };

  // for handle in handles {
  //   let output = handle.await.expect("Execution failed!");
  // }

  // Before 32s
  // Now 28.XXs when using scn-1-nginx-chance - but database locked issue!
  println!("Elapsed: {:?}", start.elapsed());

  Ok(())
}

async fn retrieve_info_image(out_path: String, name: String) -> Result<(), ()> {
  println!("Deltaing images with name: {}", name);

  // Connect to database
  let conn = db_con(out_path.clone()).await.unwrap();
  
  // Create tables 
  // - Create files table
  let conn = create_files_table(name.clone(), conn).await.unwrap();

  // - Create events table
  let conn = create_events_table(name.clone(), conn).await.unwrap();


  // FLS
  // - Execute FLS
  let image_path = get_path_image_from_name(name.clone()).unwrap();
  let lines = execute_fls(image_path, out_path.clone(), name.clone()).await.unwrap();

  // - Parse Body File FLS 
  // let parsed_body_file = parse_fls_file(out_path, name.clone()).unwrap();

  // - Parse fls lines
  let parsed_files_data = parse_fls_lines(lines).unwrap();

  // - FLS Info into database
  let conn = input_values_files(name.clone(), parsed_files_data, conn.clone()).await.unwrap();

  // Mactime
  // - Execute Mactime
  let mactime_lines = execute_mactime(out_path.clone(), name.clone()).unwrap();

  // - Parse Mactime lines
  let parsed_events_data = parse_mactime_lines(mactime_lines).unwrap(); 

  // - Mactime into database
  input_values_events(name.clone(), parsed_events_data, conn.clone()).await.unwrap();

  conn.close().await;

  println!("Finishing retrieving info from image: {}", name);

  Ok(())
}

#[derive(Debug)]
pub struct InfoEvents {
  image: String,
  date: String,
}

pub async fn get_events_images(images: Vec<String>, directory_path: String) -> Result<(Vec<MacTimeLine>, Vec<MacTimeLine>, Vec<MacTimeLine>), String> {

  let conn = db_con(directory_path.clone()).await.unwrap();
  let mut base_image_info: InfoEvents = InfoEvents { image: String::from(""), date: String::from("") };
  let mut next_image_info : InfoEvents =  InfoEvents { image: String::from(""), date: String::from("") };

  for image in images.into_iter() {
    let new_image_name = image.split('.').next().unwrap().to_string();

    let date = get_events_image_values_neariest_date(new_image_name.clone(), conn.clone()).await.unwrap();

    if base_image_info.date.is_empty() && base_image_info.image.is_empty() {
      base_image_info = InfoEvents { image: new_image_name, date: date };
    }
    else if date > base_image_info.date {
      next_image_info = InfoEvents { image: new_image_name, date: date };
    }
    else if date < base_image_info.date {
      next_image_info = base_image_info;
      base_image_info = InfoEvents { image: new_image_name, date: date };
    }
    else {
      continue;
    }
  }

  conn.close().await;

  let conn = db_con(directory_path.clone()).await.unwrap();

  let base_events = get_events_json(base_image_info.image.clone(), conn.clone()).await.unwrap();
  let next_events = get_events_json(next_image_info.image.clone(), conn.clone()).await.unwrap(); 
  let delta_events = get_events_delta(base_image_info.image, next_image_info.image, conn).await.unwrap();


  Ok((base_events, next_events, delta_events))
}

// def retrieve_info_image(outPath, iterable: str):
//   path = iterable
  
//   dbCon = database_con(outPath)
//   image_info(path=path)
//   bodyFilePath = execute_fls(imagePath=path, out=outPath)
//   name = (bodyFilePath.split('/')[-1].split('.')[0]).replace('-','_')


//   create_files_table(name=name, con=dbCon)
//   fileData = parse_fls_body_file(filePath=bodyFilePath, out=outPath)
//   input_values_files(name=name, values=fileData, con=dbCon)
  
//   # Timeline creation
//   create_timeline_image_table_2(name=name, con=dbCon)

//   # Create timeline files
//   execute_mactime(name=name, out=outPath)
  
//   # Filtering mac timeline file:
//   filter_mactime_file(name=name, out=outPath)
  
//   # Parse timelines file
//   timelineData = parse_mactime_file(name=name, out=outPath)
  
//   # Add data to database file
//   input_values_events_2(name=name, values=timelineData, con=dbCon)
  
//   # dbCon.close()
  
  
//   # Events 2.0? -
//   # load_database_from_image(imagePath=path, out=outPath)
//   # dbConLoaded = database_con_loaddb(filePath=path, outPath=outPath)
  
//   # create_loaddb_events_table(name=name, con=dbCon)
  
//   # eventsLoadeddb = get_events_loaddb(name='', con=dbConLoaded)
//   # input_values_contentdb(name=name, values=eventsLoadeddb, con=dbCon)

//   # dbCon.close
    
//   return name
  
  
// def delta_image_web(paths: list[str], images: list[str]):
//   main_logger.info('Initiating Delta images trough WEB')
//   start_time = time.time()
  
//   outPath = prepare_filesystem(paths, out='./output')
//   tablesNames = []
  
//   with Pool(2) as p:
//     res = p.map(partial(retrieve_info_image, outPath), paths)
  
//   tablesNames = res
//   print('Finished preprocessing images')

//   print('Execution time', time.time() - start_time, ' Seconds')


//   # Image Differences
//   dbCon = database_con(outPath)
//   dataImages = []
//   for tableName in tablesNames:
//     fileData = get_files_values_path(name=tableName, path='/etc', con=dbCon)
//     dataImages.append((tableName, fileData))
  
//   fileDelta  = compare_hash_path(data=dataImages,con=dbCon)
//   retrieve_files_from_image(deltas=fileDelta, out=outPath)
  
//   dbCon.close()
  
  
//   return { 'images': images, 'directoryName': outPath }


// def get_events_images(tablesNames: list[str], directoryPath: str):
//   main_logger.info('[DELTASCOPE] - Retrieving events from images')
//   dbCon = database_con(path=directoryPath)
//   baseImageTableName = ()
//   nextImageTableName = ()
  
//   newNames = []
//   for name in tablesNames:
//     newName = name.replace('.img','').replace('-','_')
//     newNames.append(newName)
//     date = get_events_image_values_neariest_date(name=newName, con=dbCon)[0][0]
    
//     if len(baseImageTableName) == 0:
//       baseImageTableName = (date, newName)
//     elif date > baseImageTableName[0]:
//       nextImageTableName = (date, newName)
//     elif date < baseImageTableName[0]:
//       nextImageTableName = baseImageTableName
//       baseImageTableName = (date, newName)
//     else:
//       pass
  
//   baseEvents = json.loads(get_events_json(baseImageTableName[1], 2023, dbCon)[0][0])[:100]
//   nextEvents = json.loads(get_events_json(nextImageTableName[1], 2023, dbCon)[0][0])[:100]
//   deltaEvents = json.loads(get_events_delta(base=baseImageTableName[1], next=nextImageTableName[1], year=2023 ,con=dbCon)[0][0])
  
//   events = { 'delta': deltaEvents, 'base': baseEvents, 'next': nextEvents }
  
//   return events

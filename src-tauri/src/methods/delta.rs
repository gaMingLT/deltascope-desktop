use std::time::Instant;

use crate::db::conn::{db_con};
use crate::db::file_db::{get_path_image_from_name};
use crate::db::tables::{create_events_table, create_files_table};
use crate::db::files::{input_values_files};
use crate::tools::rsfls::{execute_fls, parse_fls_file, parse_fls_lines};

pub async fn delta_images(out_path: String, images: Vec<String>, directory_name: String) {
  println!("Delating two images!");
  println!("Images: {:?}", images);
  println!("Directory: {}", directory_name);

  let start = Instant::now();

  for name in images.into_iter() {
    retrieve_info_image(out_path.clone(), name.split('.').next().unwrap().to_string()).await.unwrap();
  }

  println!("Elapsed: {:?}", start.elapsed());

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
  let parsed_data = parse_fls_lines(lines);

  // - FLS Info into database
  // input_values_files(name)

  // Mactime
  // - Execute Mactime

  // - Parse Mactime

  // - Mactime into database


  conn.close().await;

  Ok(())
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

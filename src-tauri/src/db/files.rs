


pub fn input_values_files() {
  // cur.executemany("INSERT INTO {0}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)".format(name.replace('-','_')), values)
}

pub fn get_files_values() {
  // res = cur.execute("SELECT * FROM {0}_files LIMIT 100".format(name))
}

pub fn get_files_values_path() {
  // res = cur.execute("SELECT * FROM {0}_files where name like '{1}%' ORDER BY mtime DESC".format(name, path))
}

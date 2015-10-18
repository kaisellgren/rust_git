/*
let path = "C:/Projects/RustGit/.git/objects/37/eaf372e4a1a5638706b757f8a086b78a5b490b";
let compressed_contents = File::open(&Path::new(path)).read_to_end().unwrap();
let contents = inflate_bytes_zlib(compressed_contents.as_slice()).unwrap();
let s = str::from_utf8(contents.as_slice()).unwrap();
println!("{}", s);

TODO: tag encode/deocde
TODO: tree encode/decode (TREE!)
TODO: packfile
TODO: packindex
TODO: index
TODO: hardcoded DECODE IDs!!
TODO: remote branches
TODO: config class

*/
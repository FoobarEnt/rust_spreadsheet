//spreadsheet::...
// pub mod Spreadsheet;
pub mod reader;

// use std::string::String;
// use std::vec;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use reader::*;

pub fn read(file_path : &'static str){
	// println!("{:?}", file_path);

	//read the extension
	let path = Path::new(file_path);
	let extension = path.extension().unwrap();
	match extension.to_str() {
		Some("xls")  => xls::read(file_path), // all reads should return a Spreadsheet object 
							                  // and inherit the Spreadsheet object
		Some("xlsx") => xlsx::read(file_path),
		Some("csv")  => csv::read(file_path),
		Some("ods")  => ods::read(file_path),
		Some("xsc")  => xsc::read(file_path),
		Some(_)      |
		None         => panic!("incompatible extension"),
	}
	let display = path.display();
	println!("{:?}", display);
	let mut f = File::open(file_path).unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s).unwrap();
	println!("{:?}", s);

	//ss = spreadsheet::Spreadsheet::new()->(Spreadsheet);
	//ss.read("/asdasd/asdasd/");
	//ss.table(1);
	//ss.get_cell(0 , 0);
	//ss.get_table_next();
	//ss.get_row_next();
	//ss.get_cell_next();
	//ss.to_json();
	//ss.from_json();
	//ss.write("path/to/write/to");
}
pub struct Spreadsheet{
	tables     : Table, //should be a vec of tables
	curr_cell  : u64,
	curr_row   : u64,
	curr_table : u8,
}

pub struct Table{
	rows : Row, //should be a vec of rows
}

pub struct Row{
	cells : &'static str, //should be a vec of cells
}

impl Spreadsheet{
	// pub fn get_cell(){}

	// pub fn get_sheet_next(){}

	// pub fn get_row_next(){}

	// pub fn get_cell_next(){}

	// pub fn get_sheet_count(){}

	// pub fn get_row_count(){}

	// pub fn get_cell_count(){}
}

/*
internal struct
{
	[ //Tables
		[ //Rows
			[ //Cells

			]
		]
	]
}

*/



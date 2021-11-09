use spreadsheet_ods::{WorkBook, Sheet, Value};
use chrono::NaiveDate;
use spreadsheet_ods::format;
use spreadsheet_ods::formula;
use spreadsheet_ods::{cm, mm};
use spreadsheet_ods::style::{CellStyle};
use color::Rgb;
use spreadsheet_ods::style::units::{TextRelief, Border, Length};

fn main() {
 let path = std::path::Path::new("testsheet.ods");
let mut wb = if path.exists() {
    spreadsheet_ods::read_ods(path).unwrap()
} else {
    WorkBook::new()
};


if wb.num_sheets() == 0 {
    let mut sheet = Sheet::new();
    sheet.set_value(0, 0, true);
    wb.push_sheet(sheet);
}

let sheet = wb.sheet(0);
let n = sheet.value(0,0).as_f64_or(0f64);
if let Value::Boolean(v) = sheet.value(1,1) {
    if *v {
        println!("was true");
    }
}

if wb.num_sheets() == 1 {
    wb.push_sheet(Sheet::new());
}

let date_format = format::create_date_dmy_format("date_format");
let date_format = wb.add_format(date_format);

let mut date_style = CellStyle::new("nice_date_style", &date_format);
date_style.set_font_bold();
date_style.set_font_relief(TextRelief::Engraved);
date_style.set_border(mm!(0.2), Border::Dashed, Rgb::new(192, 72, 72));
let date_style_ref = wb.add_cellstyle(date_style);

let mut sheet = wb.sheet_mut(1);
sheet.set_value(0, 0, 21.4f32);
sheet.set_value(0, 1, "foo");
sheet.set_styled_value(0, 2, NaiveDate::from_ymd(2020, 03, 01), &date_style_ref);
sheet.set_formula(0, 3, format!("of:={}+1", formula::fcellref(0,0)));

let mut sheet = Sheet::new_with_name("sample");
sheet.set_value(5,5, "sample");
wb.push_sheet(sheet);


spreadsheet_ods::write_ods(&mut wb, "test_out/tryout.ods");
   
}

extern crate chrono;
extern crate daybits4r;

use daybits4r::*;
use chrono::Utc;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::naive::*;
use chrono::Datelike;
use std::time::{Duration, Instant};
use std::cell::RefCell;

use std::collections::*;


pub struct Mabi{
	pub mabi : i32
}

pub struct MoreMabis<'c> {
	mabis : &'c mut Vec<Mabi>
}


impl<'c> Mabi {
	pub fn new(newMabi : i32) -> Mabi {
		Mabi{mabi: newMabi}
	}	
}

impl<'c> MoreMabis<'c> {
	pub fn new(newMabis : &'c mut Vec<Mabi>) -> MoreMabis<'c>{
		MoreMabis{mabis: newMabis}
	}
	
	pub fn sort(mut self, mut moreMabis :& 'c MoreMabis) ->Vec<Mabi> {
		let mut i = 0; 
		
		let mut newMabis : Vec<Mabi> = Vec::new();
		let moreMabiCell = RefCell::new(moreMabis);
		let moreMabisTaken = moreMabiCell.into_inner();
		
		while i < self.mabis.len() {
			let newMabi = &mut self.mabis[i];
			let moreMabi = &moreMabisTaken.mabis[i];
			
			if newMabi.mabi < moreMabi.mabi {
				newMabi.mabi += moreMabi.mabi;
				
				
//				newMabis.push(x);
//				self.mabis.insert(i, *moreMabi);
			}
			
			i += 1;
		}
		
		newMabis
	}		
}

pub struct Quarter {
    pub byte: u8
}

impl Quarter {
	pub fn new(vec : u8) ->Quarter{
		Quarter{byte: vec}
	}
	
	pub fn change(mut self, vec: u8) {
		self.byte = vec;
	}
}

pub struct Year {
	pub spring : Vec<Quarter>
}

impl Year {
	pub fn new(newSpring : Vec<Quarter>) -> Year {
		Year{spring: newSpring}
	}
	
	pub fn sort(self, moreSpring : Vec<Quarter>) {
		let mut i = 0;
//		let moreSpringCell = RefCell::new(moreSpring);
//		let moreSpringTaken = moreSpringCell.into_inner();
		
		
		for thisQuarter in self.spring.into_iter() {
			let thatQuarter = &moreSpring[i];
			
			if thisQuarter.byte < thatQuarter.byte {
				thisQuarter.change(thatQuarter.byte);	
			}
			
			i += 1;
		}
	}
}

fn main() {
//	let mabi1 = Mabi::new();
//	println!("mabi1 {}", mabi1.mabi);
//	
//	let mut mabi2 = mabi1.clone();
//	mabi2.mabi = 2;
//	
//	println!("mabi1 {} mabi2 {}", mabi1.mabi, mabi2.mabi);
//
//	let mut vec = Vec::new();
//	vec.push(2);
//	
//	let mut vec2 = Vec::new();
//	vec2.push(3);
//	
//	let mut quarter1 = Quarter::new(2);
//	
//	let mut quarter2 = quarter1.clone();
//	quarter2.change(3);
//	
//	println!("quarter1 {}; quater2 {}", quarter1.bytes, quarter2.bytes);
//	
//    println!("Hello, world!");
//    
//    let text = "ab";
//    let mut textString = String::from(text);
//    textString.push((44 as u8) as char);
//    
//    println!("textString {}", textString.as_str());
//    
//	let utc = Utc.ymd(2010, 2, 1).and_hms(0, 0, 0);
//	
//	println!("elapsed {}", utc.timestamp());
//	
//	let dt = NaiveDateTime::from_timestamp(utc.timestamp(), 0);
//	
//	//println!("elapsed {}, {}, {}", dt.date().year(), dt.date().month(), dt.date().day());
//	
//    let year = 2012; 
//    let month = 10; 
//    let dayOfMonth = 1;
//    let valueIndex = (year - 2010) as i32 * (12 * 31) + ((month - 1) * 31) as i32 + (dayOfMonth - 1) as i32;
//
//	let mut i = 0;
//	while i < 366 * 10 {
//		let millis : i64 = dt.timestamp() + i * 24 * 3600;
//		
//		let time = NaiveDateTime::from_timestamp(millis, 0);
//		
//		//println!("[main] year{} month{} day{} ", time.year(), time.month(), time.day());
//		
//		i += 1;
//	}
//
//	daybits4r::initCalendar();
//	daybits4r::ansiToReserved();
//    
//    i = 0;
//    while i < 256 {
//    	unsafe {
//    		println!("[ansi] {} {}", i, daybits4r::ANSI_RESERVED[i as usize]);
//    	}
//    	
//    	i += 1;
//    }
//    
//    
//	let mut daybits = daybits4r::parse(";AAAAAChCywMgAg==");
//	let daybitsFirst = daybits.first();
//	let daybitsLast = daybits.last();
//	
//	println!("daybitsFirst {} daybitsLast {} count {}", daybitsFirst, daybitsLast, daybits.count());
//
//	let mut daybits2 = daybits4r::parse(";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//		
//	let daybitsFirst = daybits2.first();
//	let daybitsLast = daybits2.last();
//	
//	println!("daybits2 daybitsFirst {} daybitsLast {} count {}", daybitsFirst, daybitsLast, daybits2.count());
//	
//	daybits = daybits4r::parse(";;;,,AAAAAAAAAAAAAEAI,AIIgBYA=");
//	
//	println!("daybitsFirst {} daybitsLast {} count {}", daybits.first(), daybits.last(), daybits.count());
//	
//	daybits = daybits4r::parse(";;;,,,EA==");
//	
//	println!("daybitsFirst {} daybitsLast {} count {}", daybits.first(), daybits.last(), daybits.count());
//	
//	println!("before anded");
//	
//	let anded = daybits4r::and(";;;,,AAAAAAAAAAAAAEAI,AIIgBYA=", ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	
//	println!("anded {}", anded);
//	
//	let or = daybits4r::or(";;;,,AAAAAAAAAAAAAEAI,AIIgBYA=", ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	
//	println!("or {}", or);	
//
//	let mut merged = daybits4r::or(";AAAAAChCywMgAg==", ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	daybits = daybits4r::parse(merged.as_str());
//	
//	println!("merged {} first {} last {}", merged.as_str(), daybits.first(), daybits.last());	
//	
//	merged = daybits4r::or(";;;,,AAAAAAAAAAAAAEAI,AIIgBYA=", ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	daybits = daybits4r::parse(merged.as_str());
//	
//	println!("merged {} first {} last {}", merged.as_str(), daybits.first(), daybits.last());
//	
//	let text = ",,,AAAAAAAAAAAAAAAI;AQ==";
//	daybits = daybits4r::parse(text);
//	
//	println!("wenshao.first {} last {} count {}", daybits.first(), daybits.last(), daybits.count());
//	
//	let merge1 = "IIGAEAAAECEgAw==";
//	let merge2 = "IICAEAAAECEAAw==";
//	let mut daybitsMerge1 = daybits4r::parse(merge1);
//	let daybitsMerge2 = daybits4r::parse(merge2);
//	daybitsMerge1.and(daybitsMerge2);
//	
//	println!("wenshao.and {}", daybitsMerge1.toStr());
//	
//	let and1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
//	let and2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
//	
//	//let and1Raw = "ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==;";
//	//let and2Raw = "gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
//	let mut and1 = daybits4r::parse(and1Raw);
//	let mut and2 = daybits4r::parse(and2Raw);
	
	
//	
//	let mut i = 0; 
//	while i < and1.years.len() {
//		//println!("and1 i {} year {}", i, and1.years[i].toStr().as_str());
//		
//		i += 1;
//	}
//
//	let mut i = 0; 
//	while i < and2.years.len() {
//		let slices = and1.years.as_ptr();
//		
//		//let year = *slices[i];
//		//and2.years.as_ref();
//		println!("and2 i {} year {} {}", i, and2.years[i].toStr().as_str(), year.toStr().as_str());
//		
//		i += 1;
//	}	
	
//	let mut i = 0; 
//	let mut yearsPoped : Vec<daybits4r::Year> = Vec::new();
//	while let Some(year) = and2.years.pop() {
//		yearsPoped.push(year);
//	}
//	
//	while let Some(year) = yearsPoped.pop() {
//		println!("and2 i {} year {}", i, year.toStr().as_str());
//		
//		i += 1;
//	}	
//	
//	and1.and(and2);
//	println!("bigand {} {}", and1.toStr().as_str(), and1.count());
//	
//	let or1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
//	let or2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
//	let mut or1 = daybits4r::parse(or1Raw);
//	let or2 = daybits4r::parse(or2Raw);
//	or1.or(or2);
//	
//	println!("bigor {} ", or1.toStr().as_str());

//	let yeartext = "ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==;";
//	daybits = daybits4r::parse(yeartext);
//	
//	println!("parsed {}", daybits.toStr().as_str());
//
//	// add corner cases
//	let mut blankDaybits = daybits4r::parse("");
//
//	assert_eq!(blankDaybits.count(), 0);
//	
//	let blankDaybits2 = daybits4r::parse("");
//	blankDaybits.and(blankDaybits2);
//	
//	println!("blank.and {} ", blankDaybits.toStr().as_str());
//	let blankDaybits3 = daybits4r::parse("");
//
//	//blankDaybits.or(blankDaybits3);
//
//	println!("blank.or {} {}", blankDaybits.toStr().as_str(), blankDaybits.first());
//	
//	let daybits5 = daybits4r::parse(";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	println!("blank.or.daybits5.before {} {}", daybits5.toStr().as_str(), daybits5.first());
//	blankDaybits.or(daybits5);
//	println!("blank.or.daybits5.after {} {}", blankDaybits.toStr().as_str(), blankDaybits.first());
//
//	let blankDaybits4 = daybits4r::parse(";;;;,,AAAAAAgAIA==");
//	println!("blank.or {} {}", blankDaybits4.toStr().as_str(), blankDaybits4.first());
//
//	println!("blank.or.before {} {}", blankDaybits4.toStr().as_str(), blankDaybits4.first());
//	blankDaybits.or(blankDaybits4);
//	println!("blank.or.after {} {}", blankDaybits.toStr().as_str(), blankDaybits.first());
//	
//    let mut text1 = ";;,,AAAAAAgAgIcQACA=,AgAAAg==";
//    let text2 = ";,,,AAAAAAAAAAAAAg==;AAAAEAAAAAAAIAAC,AAIAAIA4AAAAAAAD,Aw==,AAAAMAAE";
//
//    let or_text = daybits4r::or(text1, text2);
//	println!("blank.or.after.text {}", or_text);
//        let mut date1 = daybits4r::parse(";,,,AAAAAAAAAACAASAI;ZQJA,,AAAAACAAAEICAEI=,BDE=");
//        let date2 = daybits4r::parse(";;,,,BBjYCiAAAAAAAAg=;AAAAAAAAAOjv/4EH,if4N,AAAAABw=");
//        date1.and(date2);
//        println!("last.and!!! {} {}", ";;,,,BBA=", date1.toStr());
//	
//	text1 = "";
//    let text2 = ";;;;,,AAAAAAgAIA==,";
//	
//    let or_text = daybits4r::or(text1, text2);
//    println!("blank.or.after.text {} {}", or_text, daybits4r::parse(text2).toStr());
//    
//	println!("blank.and.after.text {}", daybits4r::and(";;;,,AAAAAAAAAAAAWNsG,25d9G7Zt27Zty7YN;MiFQAAgAAgQIEAE=", ";;;;,,OB++mXfg0Vc=,CQwAAAE="));
//	
//    let mut x = 12;
//    let opt_x = Some(&mut x);
//    
//    assert_eq!(opt_x, Some(&mut 12));
//    
//    let cloned = opt_x.cloned();
//    assert_eq!(cloned, Some(12));	
//	let date1 = parse(";;,,,IA==");
//	
//	println!("date1 {}", date1.toStr());
//
//	let date2 = parse(";,,,AAAAAAAAAAAAACA=");
//	println!("date2 {}", date2.toStr());	
//	
//	let date = date1.or(date2);
//	println!("date {}", date.toStr());	
//    
//    let before = Utc::now();
//
//	let daybits = parse(";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	assert_eq!(daybits.toStr(), ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
//	assert_eq!(daybits.first(), 20160916);
//	assert_eq!(daybits.last(), 20161110);
//	assert_eq!(daybits.count(), 5);
//	
//    let after = Utc::now();
//    
//	println!("cost {}", after.nanosecond() - before.nanosecond());
//	
//	let mut cost : u32 = 0;
//	cost += 10;
//	cost = (cost + 11) / 2;
//
//	println!("cost {}", cost);

	//orTest();
	compareJava();
//	lifetime();
	//testCompact();
}

fn testCompact() {
	let mut toBeCompacted : Vec<u8> = Vec::new();
	toBeCompacted.push(1);
	toBeCompacted.push(2);
	toBeCompacted.push(0);

	println!("compact {}", toBeCompacted.len());
//	compact(&mut toBeCompacted);
	println!("compact {}", toBeCompacted.len());	
}

fn lifetime() {
	let mabi1 = Mabi::new(1);
	let mabi2 = Mabi::new(2);
	let mabi3 = Mabi::new(3);
	
	let mut mabis : Vec<Mabi> = Vec::new();
	mabis.push(mabi1);
	mabis.push(mabi2);

	let mut mabis2 : Vec<Mabi> = Vec::new();
	mabis2.push(mabi3);
	
	let moreMabis1 = MoreMabis::new(&mut mabis);
	let mut moreMabis2 = MoreMabis::new(&mut mabis2);
	
	moreMabis1.sort(&moreMabis2);
}

fn orTest() {
	let or1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
	let or2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
	let now = Instant::now();	
	for i in 0..1000000 {
		let mut or1 = parse(or1Raw);
		let or2 = parse(or2Raw);
		or1 = or1.or(or2);
	} 
	let elapsed = now.elapsed();
	let cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case2_or-> cost {}", cost);	
}

fn compareJava() {
	let count : u32 = 1000000;
	let mut cost : f64;

	let now = Instant::now();
	for i in 0..1000000 {
		let daybits = parse(";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
	}
	let elapsed = now.elapsed();
	
	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case1_parse-> cost {}", cost);
	
	let mut daybits = parse(";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");

	let now = Instant::now();
	for i in 0..1000000 {
		daybits.toStr();
	}
	let elapsed = now.elapsed();
	
	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case1_output-> cost {}", cost);
	
	let now = Instant::now();	
	for i in 0..1000000 {
		daybits.first();
	}
	let elapsed = now.elapsed();
	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case1_first-> cost {}", cost);
	
	let now = Instant::now();	
	for i in 0..1000000 {
		daybits.last();
	}    
	let elapsed = now.elapsed();
	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case1_last-> cost {}", cost);	
	
	let now = Instant::now();	
	for i in 0..1000000 {
		daybits.count();		
	} 
	let elapsed = now.elapsed();
	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case1_count-> cost {}", cost);	
	
	let or1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
	let or2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";

	let now = Instant::now();	
	for i in 0..1000000 {
		let mut or1 = parse(or1Raw);
		let or2 = parse(or2Raw);
		or1 = or1.or(or2);
	} 
	let elapsed = now.elapsed();
	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case2_or-> cost {}", cost);	

	let and1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
	let and2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
	let now = Instant::now();	
	for i in 0..1000000 {
		let mut and1 = parse(and1Raw);
		let and2 = parse(and2Raw);
		and1 = and1.and(and2);
	} 
	let elapsed = now.elapsed();

	cost = (elapsed.as_secs() as f64 * 1e9 + elapsed.subsec_nanos() as f64) / 1e6;

	println!("case2_and-> cost {}", cost);	
}


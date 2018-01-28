extern crate chrono;
extern crate daybits4r;

use daybits4r::*;

use chrono::Utc;
use chrono::TimeZone;

#[test]
fn case01_parse() {
	let before = Utc::now();
	
	let daybits = parse(";AAAAAChCywMgAg==");
	assert_eq!(daybits.toStr(), ";AAAAAChCywMgAg==");
	
	let after = Utc::now();
	
	println!("timespan {}", after.timestamp() - before.timestamp());
}


#[test]
fn case02_first_last_count() {
	let daybits = parse(";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
	assert_eq!(daybits.toStr(), ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
	assert_eq!(daybits.first(), 20160916);
	assert_eq!(daybits.last(), 20161110);
	assert_eq!(daybits.count(), 5);
}

#[test]
fn case03_parse_count_first() {
	let mut blankDaybits = parse("");
	let blankDaybits3 = parse("");
	blankDaybits = blankDaybits.or(blankDaybits3);
	assert_eq!(blankDaybits.count(), 0);
	assert_eq!(blankDaybits.toStr().as_str(), "");
	assert_eq!(blankDaybits.first(), -1);
}

#[test]
fn case04_first() {
	let mut daybits = parse(";AAAAAChCywMgAg==");
	let daybitsFirst = daybits.first();
	let daybitsLast = daybits.last();

	assert_eq!(daybitsFirst, 20140205);
	assert_eq!(daybitsLast, 20140315);
	assert_eq!(daybits.count(), 13);
}

#[test]
// wenshao's original tests
fn case05_first_last() {
	let mut text = ",,,AAAAAAAAAAAAAAAI;AQ==";
	let daybits = parse(text);

	assert_eq!(daybits.first(), 20131231);
	assert_eq!(daybits.last(), 20140101);
}


#[test]
fn case06_count() {
	let text = ";,AAMAIAAAAAAAAEA=,";
	let daybits = parse(text);
	assert_eq!(daybits.first(), 20140409);
	assert_eq!(daybits.last(), 20140626);
	// test failed
	assert_eq!(daybits.count(), 4);
}


#[test]
fn case07_misc() {
	let mut blankDaybits = parse("");
	let blankDaybits2 = parse("");
	blankDaybits = blankDaybits.and(blankDaybits2);
	assert_eq!(blankDaybits.count(), 0);
	assert_eq!(blankDaybits.toStr().as_str(), "");

	blankDaybits = parse("");
	let blankDaybits3 = parse("");
	blankDaybits = blankDaybits.or(blankDaybits3);
	assert_eq!(blankDaybits.count(), 0);
	assert_eq!(blankDaybits.toStr().as_str(), "");
	assert_eq!(blankDaybits.first(), -1);

	blankDaybits = parse("");
	let blankDaybits4 = parse(";AAAAAChCywMgAg==");
	let blankDaybits = blankDaybits.or(blankDaybits4);
	assert_eq!(blankDaybits.first(), 20140205);
}

#[test]
fn case08_and_or_cornercase() {
	// add corner case tests
	let and1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
	let and2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
	let mut and1 = parse(and1Raw);
	let and2 = parse(and2Raw);
	and1 = and1.and(and2);
	assert_eq!(and1.toStr().as_str(), ";;AAAAAKwiAMDI3r8D,g/wnvvff+f/5X/cH,/4EHX+z3/LtvvZYG,gQKMDY8SgG5FYWIC;gEscAAAAQAjcIagB,uhpioJMgChB6fIcC,ScjeGRizAQ==");

	let or1Raw = ";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==";
	let or2Raw = ";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==";
	let mut or1 = parse(or1Raw);
	let or2 = parse(or2Raw);
	or1 = or1.or(or2);
	assert_eq!(or1.toStr().as_str(), ";,,ABA=,AAAGAAAQAui99acD;wQYlwv//+/f///8D,///////f//////8H,//////////////4P,//////////////8G;lt/f//92+P/97+4H,//9/vff/P/b7/s8H,//z///u3mV9FAQ==");

	// add blank tests
	let mut blankDaybits = parse("");
	assert_eq!(blankDaybits.count(), 0);
	assert_eq!(blankDaybits.toStr().as_str(), "");
}

#[test]
fn case09_and() {
	let merge1 = "IIGAEAAAECEgAw==";
	let merge2 = "IICAEAAAECEAAw==";
	let mut daybitsMerge1 = parse(merge1);
	let daybitsMerge2 = parse(merge2);
	daybitsMerge1 = daybitsMerge1.and(daybitsMerge2);

	assert_eq!(daybitsMerge1.toStr().as_str(), "IICAEAAAECEAAw==");
}

#[test]
fn case10_and() {
	let mut date1 = parse(";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==");
	let date2 = parse(";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), ";;AAAAAKwiAMDI3r8D,g/wnvvff+f/5X/cH,/4EHX+z3/LtvvZYG,gQKMDY8SgG5FYWIC;gEscAAAAQAjcIagB,uhpioJMgChB6fIcC,ScjeGRizAQ==");
}

#[test]
fn case11_and() {
	let mut date1 = parse(";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==");
	let date2 = parse(";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), ";;AAAAAKwiAMDI3r8D,g/wnvvff+f/5X/cH,/4EHX+z3/LtvvZYG,gQKMDY8SgG5FYWIC;gEscAAAAQAjcIagB,uhpioJMgChB6fIcC,ScjeGRizAQ==");
}

#[test]
fn case12_and() {
	let mut date1 = parse("");
	let date2 = parse(";;,,,BBjYCiAAAAAAAAg=;AAAAAAAAAOjv/4EH,if4N,AAAAABw=");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), "");
}

#[test]
fn case13_and() {
	let mut date1 = parse(";,,,AAAAAAAAAACAASAI;ZQJA,,AAAAACAAAEICAEI=,BDE=");
	let date2 = parse(";;,,,BBjYCiAAAAAAAAg=;AAAAAAAAAOjv/4EH,if4N,AAAAABw=");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), ";;,,,BBA=");
}

#[test]
fn case14_and() {
	let mut date1 = parse(";,,,AAAAAAAAAACAASAI;ZQJA,,AAAAACAAAEICAEI=,BDE=");
	let date2 = parse("");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), "");
}


fn and_test() {
	let mut date1 = parse(";;AAAAAO//+/f//78D,///n///f//////8H,/////////////f4P,/////////v/v//8G;ll/f//92eO3cK+wB,/j9nrdfzP7R6fI8C,acz/X/mziV9FAQ==");
	let date2 = parse(";,,ABA=,AAAGAAAQAui99acD;wQYlwrwiAMDI3v8D,g/w/vvff+f/5X/cH,/4EHX+z3/Ltvv5YG,gQKMDY8SgW5VYWIC;gMscAAAAwBr95aoH,u9p6sLMsClL7/scH,3/jeuRq3EQ==");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), ";;AAAAAKwiAMDI3r8D,g/wnvvff+f/5X/cH,/4EHX+z3/LtvvZYG,gQKMDY8SgG5FYWIC;gEscAAAAQAjcIagB,uhpioJMgChB6fIcC,ScjeGRizAQ==");

	let mut date1 = parse(";,,,AAAAAAAAAACAASAI;ZQJA,,AAAAACAAAEICAEI=,BDE=");
	let date2 = parse(";;,,,BBjYCiAAAAAAAAg=;AAAAAAAAAOjv/4EH,if4N,AAAAABw=");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), ";;,,,BBA=");

	let mut date1 = parse("");
	let date2 = parse(";;,,,BBjYCiAAAAAAAAg=;AAAAAAAAAOjv/4EH,if4N,AAAAABw=");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), "");

	let mut date1 = parse(";,,,AAAAAAAAAACAASAI;ZQJA,,AAAAACAAAEICAEI=,BDE=");
	let date2 = parse("");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), "");
}

#[test]
fn case15_or() {
    let text1 = ";;,,AAAAAAgAgIcQACA=,AgAAAg==";
    let text2 = ";,,,AAAAAAAAAAAAAg==;AAAAEAAAAAAAIAAC,AAIAAIA4AAAAAAAD,Aw==,AAAAMAAE";

    let or_text1 = or(text1, text2);
    let or_text2 = or(text2, text1);

	let daybits = parse(";;,,AAAAAAgAgIcQACA=,AgAAAg==");
	assert_eq!(daybits.toStr(), ";;,,AAAAAAgAgIcQACA=,AgAAAg==");
    assert_eq!(daybits.first(), 20150805);
    assert_eq!(daybits.last(), 20151026);

	let daybits2 = parse(";,,,AAAAAAAAAAAAAg==;AAAAEAAAAAAAIAAC,AAIAAIA4AAAAAAAD,Aw==,AAAAMAAE");
	assert_eq!(daybits2.toStr(), ";,,,AAAAAAAAAAAAAg==;AAAAEAAAAAAAIAAC,AAIAAIA4AAAAAAAD,Aw==,AAAAMAAE");
    assert_eq!(daybits2.first(), 20141213);
    assert_eq!(daybits2.last(), 20151112);

    assert_eq!(or_text1, ";,,,AAAAAAAAAAAAAg==;AAAAEAAAAAAAIAAC,AAIAAIA4AAAAAAAD,AwAAAAgAgIcQACA=,AgAAMgAE");
    assert_eq!(or_text2, ";,,,AAAAAAAAAAAAAg==;AAAAEAAAAAAAIAAC,AAIAAIA4AAAAAAAD,AwAAAAgAgIcQACA=,AgAAMgAE");
}


#[test]
fn case16_or() {
    let text1 = "";
    let text2 = ";;;;,,AAAAAAgAIA==,";

    let or_text = or(text1, text2);
    assert_eq!(or_text, ";;;;,,AAAAAAgAIA==");
}

#[test]
fn case17_or() {
    let text1 = ";;;;,,AAAAAAgAIA==,";
    let text2 = "";

    let or_text = or(text1, text2);
    assert_eq!(or_text, ";;;;,,AAAAAAgAIA==");
}

#[test]
fn case18_or() {
	let merged = or(";AAAAAChCywMgAg==", ";;;,,AAAAAAAAAAAAIIEI,AAAAAAAB");
	let daybits = parse(merged.as_str());

	assert_eq!(daybits.first(), 20140205);
}

#[test]
// wenshao's original
fn case19_or_wenshao() {
	let mut merge1 = "IIGAEAAAECEgAw==";
	let mut merge2 = "IICAEAAAECEAAw==";
	let mut daybitsMerge1 = parse(merge1);
	let mut daybitsMerge2 = parse(merge2);
	daybitsMerge1 = daybitsMerge1.or(daybitsMerge2);

	assert_eq!(daybitsMerge1.toStr().as_str(), "IIGAEAAAECEgAw==");
}

#[test]
fn case20_java_nil_test() {
	let nilText = and(";;;,,AAAAAAAAAAAAWNsG,25d9G7Zt27Zty7YN;MiFQAAgAAgQIEAE=", ";;;;,,OB++mXfg0Vc=,CQwAAAE=");
    assert_eq!(nilText, "");
}

#[test]
fn case21_nil_corner() {
	let mut date1 = parse(";;;,,AAAAAAAAAAAAWNsG,25d9G7Zt27Zty7YN;MiFQAAgAAgQIEAE=");
	let date2 = parse(";;;;,,OB++mXfg0Vc=,CQwAAAE=");
	date1 = date1.and(date2);
	assert_eq!(date1.toStr(), "");
}

#[test]
fn case22_or_test() {
	let date1 = parse(";;,,,IA==");
	let date2 = parse(";,,,AAAAAAAAAAAAACA=");
	let date = date1.or(date2);
	assert_eq!(date.toStr(), ";,,,AAAAAAAAAAAAACA=;,,,IA==");
}

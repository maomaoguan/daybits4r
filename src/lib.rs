// daybits lib rust version originated from DayBits java created by wenshao(gaotie)
//
//! daybits takes day of a year/quarter as a bit in a bit of sequence.
//! as bits accumulate, it consists quarter, year and years so that multiple dates can be placed and assembled into single short possible field.
//! in this way, there are two major advantages comparting with current timestamp/date way of processing as below:
//! 1> the date storage is minimized (ten times over)
//! 2> date functions like, detecting whether a day is within and find out the maximum time span between dates, are optimized enormously comparting with which is operated normally through date/calendar
//!
//! Example
//!     let daybits = parse(";AAAAAChCywMgAg==");
//!     assert_eq!("AAAAAChCywMgAg==#", daybits.toStr());
//!     let mut daybits = parse(";AAAAAChCywMgAg==");
//!     let daybitsFirst = daybits.first();
//!     let daybitsLast = daybits.last();
//!     assert_eq!(daybitsFirst, 20130205);
//!     assert_eq!(daybitsLast, 20130315);
//!     assert_eq!(daybits.count(), 13);
//
#[macro_use]
extern crate lazy_static;

extern crate chrono;

use chrono::Utc;
use chrono::TimeZone;
use chrono::naive::{NaiveDateTime};
use chrono::Datelike;

const BOUND: usize = 10 * 12 * 31;

pub const ANSI: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub const ANSI_EQUAL: u8 = '=' as u8;

pub const START: i32 = 19700101;

pub const END: i32 = 23991231;

pub const MAX_CHAR: u8 = 76;

pub const COMMA: u8 = 44;

pub const SHARP: u8 = 35;

pub const SEMICOLON: u8 = 59;

//
// initialize ansi chars into packed sequence
///
lazy_static! {
///

pub static ref CALENDAR_CACHE : CalendarCache = {
	let ansiText = String::from(ANSI);
    let ansiBytes = ansiText.into_bytes();
	let mut ansiReserved: [i16; 256] = [-1; 256];

    let mut i = 0;
    while i < ansiBytes.len() as usize {
        let ansiByte = ansiBytes[i];

        unsafe {
            ansiReserved[ansiByte as usize] = i as i16;
        }

        i += 1;
    }

    unsafe {
        ansiReserved['=' as usize] = 0;
    }	
    
    println!("ansi initialized");

	let mut dayCache: [i32; BOUND] = [0; BOUND];
	let mut secondsCache: [i64; BOUND] = [0; BOUND];
	
    let timeBase = Utc.ymd(2010, 1, 1).and_hms(0, 0, 0);
    let timeBaseSecs = timeBase.timestamp() / 1000;

    let mut i = 0;
    while i < 366 * 10 {
        let millis: i64 = timeBaseSecs * 1000 + i * 24 * 3600;
        let seconds: i64 = millis;

        let dateTime = NaiveDateTime::from_timestamp(millis, 0);

        let year = dateTime.year();
        let month = dateTime.month();
        let day = dateTime.day();

        if year >= 2020 {
            break;
        }

        let valueIndex = (year as i32 - 2010) * (12 * 31) + ((month - 1) as i32 * 31) + (day as i32 - 1);

        unsafe {
            secondsCache[valueIndex as usize] = seconds;
        }

        let quarterFirstDaySeconds = secods(&secondsCache, year as i16, firstMonthOfQuarter(month as i8), 1 as i8);
        let dayOfQuarter = (seconds - quarterFirstDaySeconds) / (3600 * 24);
        let quarterIndex = quarterIndex(month as i8);
        let dayCacheIndex = (year - 2010) * 31 * 12 + quarterIndex as i32 * 31 * 3 + dayOfQuarter as i32;
        let dateValue = (year * 10000) + (month as i32) * 100 + day as i32;

        unsafe {
            dayCache[dayCacheIndex as usize] = dateValue;
        }

        i += 1;
    }	
    
    println!("CalendarCache initialized");
    
    CalendarCache{dayCache: dayCache, secondsCache: secondsCache, ansiReserved: ansiReserved}
};
}

pub struct CalendarCache {
	dayCache: [i32; BOUND], 
	secondsCache: [i64; BOUND],
	ansiReserved: [i16; 256]
}

pub struct Quarter {
    bytes: Vec<u8>,
    none: bool
}

pub struct Year {
    spring: Quarter,
    summer: Quarter,
    automn: Quarter,
    winter: Quarter,
    none: bool
}

pub struct YearsOption{
	pub years: Vec<Year>,
	pub none: bool
}

pub struct DayBits {
    pub beforeYearOptions: YearsOption,
    pub yearOptions: YearsOption,
    pub none: bool
}

impl Clone for Quarter {
	fn clone(&self) -> Quarter {
        Quarter{bytes: self.bytes.clone(), none: self.none}
	}
}

impl Clone for Year {
	fn clone(&self) -> Year {
        Year{spring: self.spring.clone(), summer: self.summer.clone(), automn: self.automn.clone(), winter: self.winter.clone(), none: self.none}
	}
}

impl Clone for YearsOption {
	fn clone(&self) -> YearsOption {
        YearsOption{years: self.years.clone(), none: self.none}
	}
}

impl Clone for DayBits {
	fn clone(&self) -> DayBits {
        DayBits{beforeYearOptions: self.beforeYearOptions.clone(), yearOptions: self.yearOptions.clone(), none: self.none}
	}
}

impl YearsOption {
	pub fn createNone() -> YearsOption {
		YearsOption{years: Vec::new(), none: true}
	}

	pub fn create() -> YearsOption {
		YearsOption{years: Vec::new(), none:false}
	}

	pub fn new(years : Vec<Year>) -> YearsOption {
		YearsOption{years: years, none:false}
	}

	pub fn isNone(&self) -> bool {
		return self.none;
	}
}

impl Quarter {
    pub fn createNone() -> Quarter {
        Quarter { bytes: Vec::new(), none: true }
    }

    pub fn create() -> Quarter {
        Quarter { bytes: Vec::new(), none: false }
    }

    pub fn new(bytes: Vec<u8>) -> Quarter {
        Quarter { bytes, none: false }
    }

	pub fn isNone(&self) -> bool {
		return self.none;
	}

    pub fn count(&self) -> i32 {
    	if self.none || self.bytes.len() == 0 {
    		return 0;
    	}

        let mut count = 0;

        for value in &self.bytes {
            let mut j = 0;
            while j < 8 {
                if (value & (1 << j)) != 0 {
                    count += 1;
                }

                j += 1;
            }
        }

        return count;
    }

    pub fn countByRange(&self, year: i16, quarterIndex: i16, start: i32, end: i32) -> i32 {
    	if self.none {
    		return 0;
    	}

        let mut count = 0;
        let mut i = 0;
        let mut j = 0;

        while i < self.bytes.len() {
            let value = self.bytes[i];

            j = 0;
            while j < 8 {
                if (value & (1 << j)) != 0 {
                    let dayOfQuarter = i * 8 + j;
                    let dateValue = getDateValue(year, quarterIndex, dayOfQuarter as i16);

                    if dateValue > end {
                        break;
                    }

                    if (dateValue >= start) && (dateValue <= end) {
                        count += 1;
                    }
                }

                j += 1;
            }

            i += 1;
        }

        return count;
    }

    pub fn first(&self, year: i16, quarterIndex: i16, start: i32, end: i32) -> i32 {
    	if self.none {
    		return -1;
    	}

        let mut i = 0;
        let mut j = 0;

        while i < self.bytes.len() {
            let value = self.bytes[i];

            j = 0;
            while j < 8 {
                if (value & (1 << j)) != 0 {
                    let dayOfQuarter = i * 8 + j;

                    let dateValue = getDateValue(year, quarterIndex, dayOfQuarter as i16);

                    if dateValue >= start && dateValue <= end {
                        return dateValue;
                    }
                }

                j += 1;
            }

            i += 1;
        }

        return -1;
    }

    pub fn last(&self, year: i16, quarterIndex: i16, start: i32, end: i32) -> i32 {
        if self.none {
            return -1;
        }

        let mut i: i32 = self.bytes.len() as i32 - 1;
        let mut j = 7;

        while i >= 0 {
            let value = self.bytes[i as usize];

            j = 7;
            while j >= 0 {
                if (value & (1 << j)) != 0 {
                    let dayOfQuarter = i * 8 + j;
                    let dateValue = getDateValue(year, quarterIndex, dayOfQuarter as i16);

                    if dateValue >= start && dateValue <= end {
                        return dateValue;
                    }
                }

                if j == 0 {
                    break;
                }

                j -= 1;
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }

        return -1;
    }

    pub fn toStr(&self) -> String {
        if self.none || self.bytes.len() == 0 {
            return String::from("");
        }

        let mut output = String::new();
        let bytesLen = self.bytes.len();
        let numFullGroups = bytesLen / 3;
        let numBytesInPartialGroup = bytesLen - 3 * numFullGroups;
        let resultLen = 4 * ((bytesLen + 2) / 3);

        let mut i = 0;
        let mut inCursor = 0;

        let bytes = String::from(ANSI).into_bytes();

        while i < numFullGroups {
            let byte0 = self.bytes[inCursor] & 0xff;
            let byte1 = self.bytes[inCursor + 1] & 0xff;
            let byte2 = self.bytes[inCursor + 2] & 0xff;

            unsafe {
                output.push((bytes[(byte0 >> 2) as usize]) as char);
                output.push((bytes[((byte0 << 4) & 0x3f | (byte1 >> 4)) as usize]) as char);
                output.push(bytes[((byte1 << 2) & 0x3f | (byte2 >> 6)) as usize] as char);
                output.push(bytes[(byte2 & 0x3f) as usize] as char);
            }

            i += 1;
            inCursor += 3;
        }

        if numBytesInPartialGroup != 0 {
            let byte0 = (self.bytes[inCursor]) & 0xff;

            unsafe {
                output.push(bytes[(byte0 >> 2) as usize] as char);
            }

            if numBytesInPartialGroup == 1 {
                unsafe {
                    output.push(bytes[((byte0 << 4) & 0x3f) as usize] as char);
                }

                output.push(ANSI_EQUAL as char);
                output.push(ANSI_EQUAL as char);
            } else {
                let byte1 = (self.bytes[inCursor + 1]) & 0xff;

                unsafe {
                    output.push(bytes[((byte0 << 4) & 0x3f | (byte1 >> 4)) as usize] as char);
                    output.push(bytes[((byte1 << 2) & 0x3f) as usize] as char);
                    output.push(ANSI_EQUAL as char);
                }
            }
        }

        return output;
    }

    pub fn and(&mut self, quarter: Vec<u8>) {
        let mut quarterBytes = quarter;

        if self.bytes.len() > quarterBytes.len() {
            let mut i = 0;
            while i < quarterBytes.len() as usize {
                quarterBytes[i] &= self.bytes[i];

                i += 1;
            }

            self.bytes = compact(&mut quarterBytes);
        } else {
            let mut i = 0;
            while i < self.bytes.len() as usize {
                self.bytes[i] &= quarterBytes[i];

                i += 1;
            }

            self.bytes = compact(&mut self.bytes);
        }
    }

    pub fn or(&mut self, quarter: Vec<u8>) {
        let mut quarterBytes = quarter;
        let quarterLen = quarterBytes.len();

        if quarterLen == 0 {
            return;
        }

        if self.bytes.len() == 0 {
            self.bytes = quarterBytes;

            return;
        }

        let mut shorter = self.bytes.len();
        if shorter > quarterLen {
            shorter = quarterLen;
        }

        let mut i = 0;
        while i < shorter {
            self.bytes[i] |= quarterBytes[i];

            i += 1;
        }

        // merge additionals
        if self.bytes.len() < quarterBytes.len() {
            while i < quarterBytes.len() {
                self.bytes.push(quarterBytes[i]);

                i += 1;
            }
        }
    }

    pub fn parse(raw: &str, offset: i32, quarterSpan: i32) -> Quarter {
        let mut quarter = Quarter::create();

        if quarterSpan == 0 {
            return quarter;
        }

        let bytes = raw.as_bytes();

        // pivot pointing at quarter start and end
        let mut leftPivot = offset;
        let mut rightPivot = offset + quarterSpan - 1;

        unsafe {
            // trim illegal bytes from start
            while leftPivot < rightPivot && CALENDAR_CACHE.ansiReserved[bytes[leftPivot as usize] as usize] < 0 {
                leftPivot += 1;
            }

            // trim invalid bytes from end
            while rightPivot > 0
                && CALENDAR_CACHE.ansiReserved[bytes[rightPivot as usize] as usize] < 0 {
                rightPivot -= 1;
            }
        }

        let mut pad: i32 = 0;
        if ANSI_EQUAL == bytes[rightPivot as usize] {
            if ANSI_EQUAL == bytes[(rightPivot - 1) as usize] {
                pad = 2;
            } else {
                pad = 1;
            }
        }

        let lengthBase = rightPivot - leftPivot + 1;
        let mut separatorCount = 0;

        if quarterSpan > (MAX_CHAR as i32) {
            if bytes[MAX_CHAR as usize] == ('\r' as u8) {
                separatorCount = (lengthBase / (MAX_CHAR + 2) as i32) << 1;
            }
        }

        let quarterLength = ((lengthBase - separatorCount) * 6 >> 3) - pad;
        let quarterBytes: Vec<u8> = Vec::new();

        // decode all but the last &pad bytes
        let mut separatorPivot = 0;
        let trunkLength = (quarterLength / 3) * 3;

        while (quarter.bytes.len() as i32) < trunkLength {
            unsafe {
                // assemble three bytes into an int from four valid characters
                let assemble: i32 = ((CALENDAR_CACHE.ansiReserved[bytes[leftPivot as usize] as usize] as i32) << 18)
                    | ((CALENDAR_CACHE.ansiReserved[bytes[(leftPivot + 1) as usize] as usize] as i32) << 12)
                    | (CALENDAR_CACHE.ansiReserved[bytes[(leftPivot + 2) as usize] as usize] << 6) as i32
                    | (CALENDAR_CACHE.ansiReserved[bytes[(leftPivot + 3) as usize] as usize]) as i32;

                leftPivot += 4;

                quarter.bytes.push((assemble >> 16) as u8);
                quarter.bytes.push((assemble >> 8) as u8);
                quarter.bytes.push(assemble as u8);
            }

            separatorPivot += 1;

            // jump over line separators
            if (separatorCount > 0) && (separatorPivot == 19) {
                leftPivot += 2;
                separatorPivot = 0;
            }
        }

        if (quarter.bytes.len() as i32) < quarterLength {
            let mut merged: i32 = 0;
            let mut i = 0;

            while leftPivot <= rightPivot - pad {
                unsafe {
                    // avoid left shift overflows
                    if CALENDAR_CACHE.ansiReserved[bytes[leftPivot as usize] as usize] as u8 > 0 {
                        merged |= ((CALENDAR_CACHE.ansiReserved[bytes[leftPivot as usize] as usize] as i32) << ((18 - (i * 6)) as i32)) as i32;
                    }
                }

                leftPivot += 1;
                i += 1;
            }

            i = 16;
            while ((quarter.bytes.len() as i32) < quarterLength) && i >= 0 {
                // avoid bit right shift overflow
                if merged == 0 {
                    quarter.bytes.push(0 as u8);
                } else {
                    quarter.bytes.push((merged >> i) as u8);
                }

                i -= 8;
            }
        }

        return quarter;
    }
}

impl Year {
    pub fn createNone() -> Year {
        Year { spring: Quarter::createNone(), summer: Quarter::createNone(), automn: Quarter::createNone(), winter: Quarter::createNone(), none: true }
    }

    pub fn create() -> Year {
        Year { spring: Quarter::createNone(), summer: Quarter::createNone(), automn: Quarter::createNone(), winter: Quarter::createNone(), none: false }
    }

    pub fn count(&self) -> i32 {
        return self.spring.count() + self.summer.count() + self.automn.count() + self.winter.count();
    }

    pub fn countByRange(&self, year: i16, start: i32, end: i32) -> i32 {
        let mut count = self.countByRangeOfQuarter(year, 0, &self.spring, start, end);
        count += self.countByRangeOfQuarter(year, 1, &self.summer, start, end);
        count += self.countByRangeOfQuarter(year, 2, &self.automn, start, end);
        count += self.countByRangeOfQuarter(year, 3, &self.winter, start, end);

        return count;
    }

    pub fn countByRangeOfQuarter(&self, year: i16, quarterIndex: i16, quarter: &Quarter, start: i32, end: i32) -> i32 {
        return quarter.countByRange(year, quarterIndex, start, end);
    }

    fn countQuarter(&self, quarter: &Quarter) -> i32 {
        return quarter.count();
    }

    fn firstQuarter(&self, year: i16, quarterIndex: i16, quarter: &Quarter, start: i32, end: i32) -> i32 {
        return quarter.first(year, quarterIndex, start, end);
    }

    fn lastQuarter(&self, year: i16, quarterIndex: i16, quarter: &Quarter, start: i32, end: i32) -> i32 {
        return quarter.last(year, quarterIndex, start, end);
    }

    pub fn isNone(&self) -> bool {
        return self.none;
    }

    pub fn first(&self, year: i16, start: i32, end: i32) -> i32 {
        let mut firstVal = self.firstQuarter(year, 0, &self.spring, start, end);

        if firstVal == -1 {
            firstVal = self.firstQuarter(year, 1, &self.summer, start, end);
        }

        if firstVal == -1 {
            firstVal = self.firstQuarter(year, 2, &self.automn, start, end);
        }

        if firstVal == -1 {
            firstVal = self.firstQuarter(year, 3, &self.winter, start, end);
        }

        return firstVal;
    }

    pub fn last(&self, year: i16, start: i32, end: i32) -> i32 {
        let mut lastVal = self.lastQuarter(year, 3, &self.winter, start, end);

        if lastVal == -1 {
            lastVal = self.lastQuarter(year, 2, &self.automn, start, end);
        }

        if lastVal == -1 {
            lastVal = self.lastQuarter(year, 1, &self.summer, start, end);
        }

        if lastVal == -1 {
            lastVal = self.lastQuarter(year, 0, &self.spring, start, end);
        }

        return lastVal;
    }

    pub fn and(mut self, year: Year) -> Year {
        self.spring.and(year.spring.bytes);
        self.spring.none = self.spring.none && year.spring.none;

        self.summer.and(year.summer.bytes);
        self.summer.none = self.summer.none && year.summer.none;

        self.automn.and(year.automn.bytes);
        self.automn.none = self.automn.none && year.automn.none;

        self.winter.and(year.winter.bytes);
        self.winter.none = self.winter.none && year.winter.none;

        return self;
    }

    pub fn or(mut self, year: Year) -> Year{
    	if !year.spring.isNone() {
			self.spring.or(year.spring.bytes);
	        self.spring.none = year.spring.none;
    	}

        if !year.summer.isNone() {
	        self.summer.or(year.summer.bytes);
	        self.summer.none = year.summer.none;
        }

        if !year.automn.isNone() {
	        self.automn.or(year.automn.bytes);
	        self.automn.none = year.automn.none;
        }

		if !year.winter.isNone() {
	        self.winter.or(year.winter.bytes);
	        self.winter.none = year.winter.none;
		}

        self.none = self.none || year.none;

        return self;
    }

	fn quarterToStr(quarter : &Quarter) -> String{
		if quarter.isNone() {
			return String::from("");
		}

		return quarter.toStr();
	}

    pub fn toStr(&self) -> String {
        let mut output: String = String::new();

        if !self.winter.isNone() {
            output.push_str(Year::quarterToStr(&self.spring).as_str());
            output.push(',');
            output.push_str(Year::quarterToStr(&self.summer).as_str());
            output.push(',');
            output.push_str(Year::quarterToStr(&self.automn).as_str());
            output.push(',');
            output.push_str(Year::quarterToStr(&self.winter).as_str());
        } else if !self.automn.isNone() {
            output.push_str(Year::quarterToStr(&self.spring).as_str());
            output.push(',');
            output.push_str(Year::quarterToStr(&self.summer).as_str());
            output.push(',');
            output.push_str(Year::quarterToStr(&self.automn).as_str());
        } else if !self.summer.isNone() {
            output.push_str(Year::quarterToStr(&self.spring).as_str());
            output.push(',');
            output.push_str(Year::quarterToStr(&self.summer).as_str());
        } else {
            output.push_str(Year::quarterToStr(&self.spring).as_str());
        }

        return output;
    }

    pub fn parse(raw: &str, startPosition: &mut i32) -> Year {
        let mut yearNone = Year::createNone();
        let bytes = raw.as_bytes();

        // ending situations
        if bytes[*startPosition as usize] == SHARP {
            if *startPosition == (bytes.len() - 1) as i32 {
                *startPosition += 1;

                return yearNone;
            }

            *startPosition += 1;
        }

        if bytes[*startPosition as usize] == SEMICOLON {
            if *startPosition == (bytes.len() - 1) as i32 {
                *startPosition += 1;

                return yearNone;
            }

            *startPosition += 1;

            return yearNone;
        }

        let mut year = Year::create();
        let mut i = 0;
        while i < 4 {
            if *startPosition >= bytes.len() as i32 {
                break;
            }

            if (bytes[*startPosition as usize] == SHARP)
                || (bytes[*startPosition as usize] == SEMICOLON || isEOF(raw, *startPosition)) {
                break;
            }

			let mut quarter = Quarter::createNone();
			if bytes[*startPosition as usize] != COMMA {
				let mut nextPosition = *startPosition;
	            while nextPosition < (bytes.len() as i32) {
	                let byte = bytes[nextPosition as usize];

	                if (byte == COMMA) || (byte == SEMICOLON) || (byte == SHARP) {
	                    break;
	                }

	                nextPosition += 1;
	            }

	            // skip comma
	            let quarterSpan = nextPosition - *startPosition;

	            quarter = Quarter::parse(raw, *startPosition, quarterSpan);

	            *startPosition = nextPosition;

	            if *startPosition < (bytes.len() as i32) {
	                let byte = bytes[*startPosition as usize];

	                if byte == COMMA {
	                    *startPosition += 1;
	                }
	            }
			}
			else {
				*startPosition += 1;
			}

            if i == 0 {
                year.spring = quarter;
            } else if i == 1 {
                year.summer = quarter;
            } else if i == 2 {
                year.automn = quarter;
            } else if i == 3 {
                year.winter = quarter;
            }

            i += 1;
        }

        if !isEOF(raw, *startPosition) {
            if bytes[*startPosition as usize] == SHARP {
                // skip
            } else {
                if bytes[*startPosition as usize] != SEMICOLON {
                    panic!("illegal state found");
                }

                *startPosition += 1;
            }
        }

        return year;
    }
}

impl DayBits {
	pub fn createNone() -> DayBits{
		DayBits{beforeYearOptions:YearsOption::createNone(), yearOptions:YearsOption::createNone(), none:true }
	}

	pub fn isNone(&self) -> bool {
		return self.none;
	}

	pub fn new() -> DayBits{
		DayBits{beforeYearOptions:YearsOption::create(), yearOptions:YearsOption::create(), none:false }
	}

    /**
    * count available days
    */
    pub fn count(&self) -> i32 {
        let mut count = 0;

        for beforeYear in self.beforeYearOptions.years.iter() {
            count += beforeYear.count();
        }

        for year in self.yearOptions.years.iter() {
            count += year.count();
        }

        return count;
    }

    /**
    * find first from daybits
    */
    pub fn first(&self) -> i32 {
        return self.firstByRange(START, END);
    }

    /**
    * find last from daybits
    */
    pub fn last(&self) -> i32 {
        return self.lastByRange(START, END);
    }

    fn firstByRange(&self, start: i32, end: i32) -> i32 {
        let mut first = -1;

        if (self.beforeYearOptions.years.len() as i32) > 0 {
            let mut i = (self.beforeYearOptions.years.len() - 1) as i32;
            while i >= 0 {
                let year = &self.beforeYearOptions.years[i as usize];

                first = year.first((2012 - i) as i16, start, end);
                if first > -1 {
                    return first;
                }

                if i == 0 {
                    break;
                }

                i -= 1;
            }
        }

        if (self.yearOptions.years.len() as i32) > 0 {
            let mut i = 0;

            while i < self.yearOptions.years.len() as i32 {
                let year = &self.yearOptions.years[i as usize];

                first = year.first((2013 + i) as i16, start, end);
                if first > -1 {
                    return first;
                }

                i += 1;
            }
        }

        return first;
    }


    fn lastByRange(&self, start: i32, end: i32) -> i32 {
        let mut last = -1;

        if (self.yearOptions.years.len() as i32) > 0 {
            let mut i = self.yearOptions.years.len() - 1;

            while i >= 0 {
                let year = &self.yearOptions.years[i as usize];

                last = year.last((2013 + i) as i16, start, end);
                if last > -1 {
                    return last;
                }

                if i == 0 {
                    break;
                }

                i -= 1;
            }
        }

        if (self.beforeYearOptions.years.len() as i32) > 0 {
            let mut i = 0;

            while i < self.beforeYearOptions.years.len() {
                let year = &self.beforeYearOptions.years[i as usize];

                last = year.last((2012 - i) as i16, start, end);
                if last > -1 {
                    return last;
                }

                i += 1;
            }
        }

        return last;
    }

    pub fn and(mut self, daybits: DayBits) -> DayBits{
    	if self.yearOptions.none || daybits.yearOptions.none {
    		return DayBits::createNone();
    	}

        let mut i: u16 = 0;
		let mut thisYears = self.yearOptions.years;
		let mut thatYears = daybits.yearOptions.years;

		let mut newYears : Vec<Year> = Vec::new();
        if thisYears.len() <= thatYears.len() {
        	for mut thisYear in thisYears.into_iter() {
				let thatYear = thatYears[i as usize].clone();

				if thisYear.isNone() || thatYear.isNone() {
					newYears.push(Year::createNone());
				}
				else {
					thisYear = thisYear.and(thatYear);
					newYears.push(thisYear);
				}

				i += 1;        		
        	}

			compactYears(&mut newYears);
			return DayBits{beforeYearOptions: YearsOption::createNone(), yearOptions:YearsOption::new(newYears), none:false};
        } else {
			for thatYear in thatYears.into_iter() {
				let newYear = thisYears[i as usize].clone().and(thatYear);
				newYears.push(newYear);

				i += 1;
			}

			compactYears(&mut newYears);
			return DayBits{beforeYearOptions: YearsOption::createNone(), yearOptions:YearsOption::new(newYears), none:false};
        }
    }

    pub fn or(mut self, mut daybits: DayBits) -> DayBits{
		if daybits.yearOptions.isNone() {
			return self;
		}

		if self.yearOptions.isNone() {
			self.yearOptions = daybits.yearOptions.clone();
			self.none = daybits.none;

			return self;
		}

		let mut i : u16 = 0;
		let mut thatYears = daybits.yearOptions.years;		
		let mut thisYears = self.yearOptions.years;
		let mut newYears : Vec<Year> = Vec::new();

		if thatYears.len() == 0 {
			return DayBits{beforeYearOptions: YearsOption::createNone(), yearOptions:YearsOption::new(thisYears), none:daybits.yearOptions.none || self.yearOptions.none};
		}
		else {
			let mut longer : Vec<Year>;
			let mut shorter : Vec<Year>;
			if thisYears.len() <= thatYears.len() {
				longer = thatYears;
				shorter = thisYears;
			}
			else {
				longer = thisYears;
				shorter = thatYears;
			}

			let shorterCounter = shorter.len();
			let longerCounter = longer.len();
			for mut longerYear in longer.into_iter() {
				if i < shorterCounter as u16 {
					let mut shorterYear = shorter[i as usize].clone();

					if shorterYear.isNone() {
						i += 1;
						newYears.push(longerYear);

						continue;
					}

					if longerYear.isNone() {
						newYears.push(shorterYear);
					}
					else {
						shorterYear = shorterYear.or(longerYear);
						newYears.push(shorterYear);
					}
				}
				else if i < longerCounter as u16 && i >= shorterCounter as u16 {
					newYears.push(longerYear);
				}
				
				i += 1;
			}

			return DayBits{beforeYearOptions: YearsOption::createNone(), yearOptions:YearsOption::new(newYears), none:daybits.yearOptions.none || self.yearOptions.none};
		}
    }

    pub fn toStr(&self) -> String {
        let mut output: String = String::new();

        if !self.beforeYearOptions.isNone() {
            let mut i = 0;
            while i < self.beforeYearOptions.years.len() {
                if i > 0 && !output.is_empty() {
                    output.push(SEMICOLON as char);
                }

                let year = &self.beforeYearOptions.years[i];

                if !year.isNone() {
					output.push_str(year.toStr().as_str());
                }

                i += 1;
            }

            if !output.is_empty() {
                output.push(SHARP as char);
            }
        }

        if !self.yearOptions.isNone() {
            let mut i = 0;
            while i < self.yearOptions.years.len() {
                if i > 0 {
                    output.push(SEMICOLON as char);
                }

                let year = &self.yearOptions.years[i];
                if !year.isNone() {
					output.push_str(year.toStr().as_str());
                }

                i += 1;
            }
        }

        return output;
    }
}

fn secods(secondsCache: &[i64; BOUND], year: i16, month: i8, dayOfMonth: i8) -> i64 {
    if year >= 2010 && year < 2020 {
        let valueIndex: i32 = (year as i32 - 2010) * (12 * 31) + (month as i32 - 1) * 31 + (dayOfMonth as i32 - 1);

        unsafe {
            return secondsCache[valueIndex as usize];
        }
    }

    let calendar = Utc.ymd(year as i32, (month) as u32, dayOfMonth as u32).and_hms(0, 0, 0);

    return calendar.timestamp() / 1000;
}

fn firstMonthOfQuarter(month: i8) -> i8 {
    if month < 1 || month > 12 {
        return -1;
    }

    if month <= 3 {
        return 1;
    }

    if month <= 6 {
        return 4;
    }

    if month <= 9 {
        return 7;
    }

    return 10;
}

fn isEOF(raw: &str, position: i32) -> bool {
    return position >= (raw.len() as i32);
}

fn quarterIndex(month: i8) -> i8 {
    if month < 1 || month > 12 {
        return -1;
    }

    if month <= 3 {
        return 0;
    }

    if month <= 6 {
        return 1;
    }

    if month <= 9 {
        return 2;
    }

    return 3;
}

/**
* parse raw string into daybits type
*/
pub fn parse(text: &str) -> DayBits {
    let bytes = text.as_bytes();

    let mut startPosition: i32 = 0;
    let mut daybits = DayBits::new();

    while startPosition < (bytes.len() as i32) {
        let year = Year::parse(text, &mut startPosition);

		daybits = add(daybits, year);
    }

    return daybits;
}

pub fn add(mut daybits : DayBits, year: Year) -> DayBits{
	daybits.yearOptions.years.push(year);

	return daybits;
}

pub fn and(left: &str, right: &str) -> String {
    let mut daybitsLeft = parse(left);
    let daybitsRight = parse(right);

    daybitsLeft = daybitsLeft.and(daybitsRight);

    return daybitsLeft.toStr();
}

pub fn or(left: &str, right: &str) -> String {
    let mut daybitsLeft = parse(left);
    let daybitsRight = parse(right);

    daybitsLeft = daybitsLeft.or(daybitsRight);

    return daybitsLeft.toStr();
}


fn dayOfQuarter(year: i16, month: i16, dayOfMonth: i16) -> i32 {
    if month < 1 || month > 12 {
        return -1;
    }

    let quarterFirstMonth: i8;
    if month < 4 {
        quarterFirstMonth = 1;
    } else if month < 7 {
        quarterFirstMonth = 4;
    } else if month < 10 {
        quarterFirstMonth = 7;
    } else {
        quarterFirstMonth = 10;
    }

    return 0;
}

fn compactYears(years : &mut Vec<Year>){
	let mut i : i16 = years.len() as i16 -1;

	while i >= 0 {
		if !years[i as usize].isNone() && years[i as usize].count() > 0 {
			break;
		}

		years.remove(i as usize);

		i -= 1;
	}
}

fn compact(bytes: &mut Vec<u8>) -> Vec<u8> {
    let mut newBytes: Vec<u8> = Vec::new();

    if bytes.len() == 0 {
        return newBytes;
    }

    let mut compacted = (bytes.len() - 1) as i32;

    while compacted >= 0 {
        if bytes[compacted as usize] != 0 {
            break;
        }
		
        compacted -= 1;
    }

    let mut i = 0;
    while i < (compacted + 1) {
        newBytes.push(bytes[i as usize]);

        i += 1;
    }

    return newBytes;
}

fn getDateValue(year: i16, quarterIndex: i16, dayOfQuarter: i16) -> i32 {
    if year >= 2010 && year < 2020 {
        let dayCacheIndex = (year - 2010) * 31 * 12 + quarterIndex * 31 * 3 + dayOfQuarter;

        if dayCacheIndex < BOUND as i16 {
            unsafe {
                return CALENDAR_CACHE.dayCache[dayCacheIndex as usize];
            }
        }
    }

    return -1;
}

pub fn daybits_and(str1: &str, str2: &str) -> String {
    let date1 = parse(str1);
    let mut date2 = parse(str2);
    let mut date = date1.and(date2);

    date.toStr()
}

pub fn daybits_or(str1: &str, str2: &str) -> String {
    let date1 = parse(str1);
    let date2 = parse(str2);
    let mut date = date1.or(date2);

    date.toStr()
}

pub fn daybits_count(text: &String) -> i64 {
    parse(text).count() as i64
}

pub fn daybits_first(text: &String) -> i64 {
    let first = parse(text).first() as i64;
    if first > 0 {
        first
    } else { 0 }
}

pub fn daybits_last(text: &String) -> i64 {
    let last = parse(text).last() as i64;
    if last > 0 {
        last
    } else { 0 }
}

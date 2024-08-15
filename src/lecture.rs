#[derive(Debug, PartialEq)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednsday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Unknown,
}

impl From<u8> for WeekDay {
    fn from(val: u8) -> WeekDay {
        match val {
            0 => WeekDay::Monday,
            1 => WeekDay::Tuesday,
            2 => WeekDay::Wednsday,
            3 => WeekDay::Thursday,
            4 => WeekDay::Friday,
            5 => WeekDay::Saturday,
            6 => WeekDay::Sunday,
            _ => WeekDay::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TimePoint {
    hour: u8,
    minute: u8,
}

impl TimePoint {
    pub fn new(hour: u8, minute: u8) -> TimePoint {
        TimePoint { hour, minute }
    }

    pub fn to_int(self) -> u16 {
        (self.hour as u16) * 60 + (self.minute as u16)
    }
}

#[derive(Debug, PartialEq)]
pub struct LectureTime {
    week_day: WeekDay,
    start: TimePoint,
    end: TimePoint,
}

impl LectureTime {
    pub fn new(week_day: WeekDay, start: TimePoint, end: TimePoint) -> LectureTime {
        LectureTime {
            week_day,
            start,
            end,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LectureSchedule {
    name: String,
    code: String,
    times: Vec<LectureTime>,
}

impl LectureSchedule {
    pub fn new(name: String, code: String, times: Vec<LectureTime>) -> LectureSchedule {
        LectureSchedule { name, code, times }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_lecture_schedule(
        num: usize,
        first: LectureTime,
        second: LectureTime,
    ) -> LectureSchedule {
        LectureSchedule::new(
            String::from(format!("lecture-{}", num)),
            String::from(format!("lecture-code-{}", num)),
            vec![first, second],
        )
    }

    #[test]
    fn test_lecture_sechedule() {
        let first = LectureTime::new(WeekDay::Monday, TimePoint::new(9, 0), TimePoint::new(10, 0));
        let second = LectureTime::new(
            WeekDay::Wednsday,
            TimePoint::new(12, 0),
            TimePoint::new(13, 0),
        );

        let lecture_1 = make_lecture_schedule(1, first, second);

        assert_eq!(lecture_1.code, "lecture-code-1");
        assert_eq!(
            lecture_1.times[0],
            LectureTime::new(WeekDay::Monday, TimePoint::new(9, 0), TimePoint::new(10, 0))
        );
        assert_ne!(
            lecture_1.times[1],
            LectureTime::new(
                WeekDay::Monday,
                TimePoint::new(12, 0),
                TimePoint::new(13, 0)
            )
        );
    }

    #[test]
    fn test_lecture_is_conflict() {}

    // fuzzer
}

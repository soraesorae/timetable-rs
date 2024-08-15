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
    total_minutes: u16,
}

impl TimePoint {
    pub fn new(hour: u8, minute: u8) -> TimePoint {
        let total_minutes = (hour as u16) * 60 + (minute as u16);
        TimePoint {
            hour,
            minute,
            total_minutes,
        }
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

    // TODO: check other.end == start
    pub fn is_conflict(&self, other: &LectureSchedule) -> bool {
        for lecture_time in &self.times {
            let (day, start, end) = (
                &lecture_time.week_day,
                lecture_time.start.total_minutes,
                lecture_time.end.total_minutes,
            );
            let res = other.times.iter().any(|other_lecture| {
                let (other_day, other_start, other_end) = (
                    &other_lecture.week_day,
                    other_lecture.start.total_minutes,
                    other_lecture.end.total_minutes,
                );
                // start <= other_end && other_start <= end
                day == other_day && !(other_end < start || end < other_start)
            });
            if res {
                return true;
            }
        }
        false
    }
}

// TODO: fuzzer
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
    fn test_lecture_is_conflict() {
        let first = LectureTime::new(
            WeekDay::Monday,
            TimePoint::new(10, 0),
            TimePoint::new(12, 30),
        );
        let second = LectureTime::new(
            WeekDay::Wednsday,
            TimePoint::new(10, 0),
            TimePoint::new(12, 30),
        );
        let lecture_1 = make_lecture_schedule(1, first, second);

        let first = LectureTime::new(
            WeekDay::Monday,
            TimePoint::new(12, 20),
            TimePoint::new(13, 30),
        );
        let second = LectureTime::new(
            WeekDay::Wednsday,
            TimePoint::new(12, 20),
            TimePoint::new(13, 30),
        );
        let lecture_2 = make_lecture_schedule(2, first, second);

        let first = LectureTime::new(
            WeekDay::Monday,
            TimePoint::new(13, 20),
            TimePoint::new(15, 30),
        );
        let second = LectureTime::new(
            WeekDay::Wednsday,
            TimePoint::new(13, 20),
            TimePoint::new(15, 30),
        );
        let lecture_3 = make_lecture_schedule(3, first, second);

        assert_eq!(lecture_1.is_conflict(&lecture_2), true);
        assert_eq!(lecture_2.is_conflict(&lecture_1), true);

        assert_eq!(lecture_1.is_conflict(&lecture_3), false);
        assert_eq!(lecture_3.is_conflict(&lecture_1), false);

        assert_eq!(lecture_2.is_conflict(&lecture_3), true);
        assert_eq!(lecture_3.is_conflict(&lecture_2), true);
    }
}

struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl From<Date> for String {
    fn from(date: Date) -> Self {
        format!("{:02}/{:02}/{:04}", date.day, date.month, date.year)
    }
}

fn main() {
    let my_date = Date { day: 31, month: 1, year: 2024 };
    let formatted_date: String = String::from(my_date); // Implicit conversion from Date to String

    println!("Formatted date: {}", formatted_date);
}
use std::collections::HashMap;

static NO_DATA_WARNING: &'static str = "No time track data found";

pub fn display(data: &HashMap<String, u64>) {
    println!("{}", format(data));
}

fn format(data: &HashMap<String, u64>) -> String {
    let mut output_lines = vec![];

    for (project, time_in_seconds) in data.iter() {
        let hours = time_in_seconds / (60 * 60);
        let minutes = (time_in_seconds - (hours * 60 * 60)) / 60;
        let seconds = time_in_seconds - (hours * 60 * 60) - (minutes * 60);

        let output_line = match (hours, minutes, seconds) {
            (0, 0, seconds) => format!("{} - {} seconds", project, seconds),
            (0, minutes, seconds) => format!("{} - {} minutes {} seconds", project, minutes, seconds),
            (hours, minutes, _) => format!("{} - {} hours {} minutes", project, hours, minutes),
        };

        output_lines.push(output_line);
    }

    if output_lines.is_empty() {
        return String::from(NO_DATA_WARNING);
    }

    output_lines.sort(); // alphabetize the output by project name
    output_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_no_data(){
        let data = HashMap::new();
        assert_eq!(NO_DATA_WARNING, format(&data));
    }

    #[test]
    fn format_seconds(){
        let mut data = HashMap::new();
        data.insert(String::from("project1"), 30);

        assert_eq!("project1 - 30 seconds", format(&data));
    }

    #[test]
    fn format_minutes(){
        let mut data = HashMap::new();
        data.insert(String::from("project1"), 330);

        assert_eq!("project1 - 5 minutes 30 seconds", format(&data));
    }

    #[test]
    fn format_hours(){
        let mut data = HashMap::new();
        data.insert(String::from("project1"), (5*60*60) + (10*60) + 30);

        assert_eq!("project1 - 5 hours 10 minutes", format(&data));
    }

    #[test]
    fn format_multiple_projects(){
        let mut data = HashMap::new();
        data.insert(String::from("project1"), 30);
        data.insert(String::from("project2"), 330);

        assert_eq!("project1 - 30 seconds\nproject2 - 5 minutes 30 seconds", format(&data));
    }
}

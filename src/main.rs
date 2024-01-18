use unindent::unindent;
use std::fs::{self};

fn main() {
    let input = unindent(&fs::read_to_string("./input.txt").unwrap());

    let lines = input.lines();

    let mut output_lines: Vec<String> = vec!["".to_owned(); 5000];

    let mut sources: Vec<Source> = vec![];

    for line in lines.enumerate() {
        if line.1.starts_with("    \"") {
        
            output_lines.insert(line.0, "\n".to_owned() + &unindent(&format!(
                "
                InternalDatabaseVerificationInfo(
                {}
                "
                ,
                line.1.to_owned()
            )));
        } else if line.1.starts_with("    listOf(") {
            output_lines.insert(line.0, line.1.to_owned());
        } else if line.1.starts_with("        // ") {
            sources = line.1.trim().trim_start_matches("//").trim().split('/').map(|item| match item {
                "Google Play Store" => Source::GOOGLE_PLAY_STORE,
                "Google Pixel OS" => Source::GOOGLE_PIXEL_OS,
                "GitHub" => Source::GITHUB,
                "Accrescent" => Source::ACCRESCENT,
                "Codeberg" => Source::CODEBERG,
                "F-Droid" => Source::FDROID,
                "Website" => Source::WEBSITE,
                "GitLab" => Source::GITLAB,
                _ => Source::APP_FDROID_REPO,
            }).collect()
        } else if line.1.starts_with("        Hashes(") {
            output_lines.insert(line.0, format!("\n{}", line.1.to_owned()));
        } else if line.1.starts_with("            listOf(") {
            output_lines.insert(line.0, format!(
                "
            listOf(
                {}
            ),
            listOf("
            , sources.iter().map(|source| match source {
                Source::GOOGLE_PLAY_STORE => "Source.GOOGLE_PLAY_STORE".to_owned(),
                Source::GOOGLE_PIXEL_OS => "Source.GOOGLE_PIXEL_OS".to_owned(),
                Source::GITHUB => "Source.GITHUB".to_owned(),
                Source::ACCRESCENT => "Source.ACCRESCENT".to_owned(),
                Source::CODEBERG => "Source.CODEBERG".to_owned(),
                Source::FDROID => "Source.FDROID".to_owned(),
                Source::APP_FDROID_REPO => "Source.APP_FDROID_REPO".to_owned(),
                Source::WEBSITE => "Source.WEBSITE".to_owned(),
                Source::GITLAB => "Source.GITLAB".to_owned(),
            }).collect::<Vec<String>>().join(",\n                ")));
        } else if line.1.starts_with("                \"") {
            output_lines.insert(line.0, format!("\n                {}", line.1.to_owned().trim()));
        } else if line.1.starts_with("            ),") {
            output_lines.insert(line.0, format!("\n            {}", line.1.to_owned().trim()))
        } else if line.1.starts_with("            false") || line.1.starts_with("            true") {
            output_lines.insert(line.0, format!("\n            {}", line.1.to_owned().trim()))
        } else if line.1.starts_with("        )") {
            output_lines.insert(line.0, format!("\n        {}", line.1.to_owned().trim()))
        } else if line.1.starts_with("    )") {
            output_lines.insert(line.0, format!("\n    {}", line.1.to_owned().trim()))
        } else if line.1.starts_with("),") {
            output_lines.insert(line.0, format!("\n{}", line.1.to_owned().trim()))
        }
    }

    fs::write("./output.txt", output_lines.join("").lines().map(|s| s.to_owned()).collect::<Vec<String>>().join("\n    ")).unwrap();
}

enum Source {
    GOOGLE_PLAY_STORE,
    GOOGLE_PIXEL_OS,
    GITHUB,
    ACCRESCENT,
    CODEBERG,
    FDROID,
    APP_FDROID_REPO,
    WEBSITE,
    GITLAB,
}
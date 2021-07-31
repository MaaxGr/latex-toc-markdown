
use regex::Regex;

pub fn line_to_md(line: &str) -> String {

    let toc_layer = get_toc_layer(line);

    if line.contains("nonumberline") {
        let regex = Regex::new(r"\\contentsline \{[^{]+}\{\\nonumberline ([^}]+)}.+").unwrap();
        let caps = regex.captures(line).unwrap();

        let text = caps.get(1).unwrap().as_str();

        return format_text_with_layer(toc_layer, text)
    }

    if line.contains("numberline") {
        let regex = Regex::new(r"\\contentsline \{[^{]+}\{\\numberline \{(\d+[.*\d]*)}([^}]+)}.+").unwrap();
        let caps = regex.captures(line).unwrap();

        let number = caps.get(1).unwrap().as_str();
        let text = caps.get(2).unwrap().as_str();

        return format_text_with_layer(toc_layer, format!("{} {}", number, text).as_str())
    } else {

        let regex = Regex::new(r"\\contentsline \{[^{]+}\{([^{]+)}.+").unwrap();
        let caps = regex.captures(line).unwrap();

        let text = caps.get(1).unwrap().as_str();

        return format_text_with_layer(toc_layer, text)

    }

}

fn get_toc_layer(line: &str) -> usize {
    if line.contains("{chapter}") {
        return 1
    }

    if line.contains("{section}") {
        return 2
    }

    if line.contains("{subsection}") {
        return 3
    }

    if line.contains("{subsubsection}") {
        return 4
    }

    if line.contains("{paragraph}") {
        return 5
    }

    return 0
}

fn format_text_with_layer(layer: usize, text: &str) -> String {
    let spaces = (layer - 1) * 2;
    return format!("{}* {}", " ".repeat(spaces), text)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn line_to_md_chapter() {
        let expected = "* Abbildungsverzeichnis";
        let subject_under_test = "\\contentsline {chapter}{Abbildungsverzeichnis}{4}{chapter*.2}%";

        assert_eq!(expected, line_to_md(subject_under_test));
    }

    #[test]
    fn line_to_md_chapter_numberline() {
        let expected = "* 1 Einführung";
        let subject_under_test = "\\contentsline {chapter}{\\numberline {1}Einführung}{8}{chapter.1}%";

        assert_eq!(expected, line_to_md(subject_under_test));
    }

    #[test]
    fn line_to_md_section_numberline() {
        let expected = "  * 1.1 Einleitung";
        let subject_under_test = "\\contentsline {section}{\\numberline {1.1}Einleitung}{8}{section.1.1}%";

        assert_eq!(expected, line_to_md(subject_under_test));
    }

    #[test]
    fn line_to_md_subsection_numberline() {
        let expected = "    * 2.4.1 Allgemeines";
        let subject_under_test = "\\contentsline {subsection}{\\numberline {2.4.1}Allgemeines}{13}{subsection.2.4.1}%";

        assert_eq!(expected, line_to_md(subject_under_test));
    }

    #[test]
    fn line_to_md_paragraph_nonumberline() {
        let expected = "        * Auswahl des Webservers";
        let subject_under_test = "\\contentsline {paragraph}{\\nonumberline Auswahl des Webservers}{31}{section*.7}%";

        assert_eq!(expected, line_to_md(subject_under_test));
    }

}
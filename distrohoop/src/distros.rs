use colored::Color;

pub struct Distro {
    pub name: String,
    pub description: String,
    pub color: colored::Color,
    pub is_bold: bool
}

pub fn get_distros() -> Vec<Distro> {
    vec![
        Distro {
            name: "Ubuntu".to_string(),
            description: "Lorem ipsum".to_string(),
            color: Color::BrightYellow,
            is_bold: true
        },
        Distro {
            name: "Arch Linux".to_string(),
            description: "Lorem ipsum".to_string(),
            color: Color::Blue,
            is_bold: false
        },
        Distro {
            name: "Fedora".to_string(),
            description: "Lorem Ipsum".to_string(),
            color: Color::Blue,
            is_bold: true
        },
        Distro {
            name: "Linux Mint".to_string(),
            description: "Lorem ipsum".to_string(),
            color: Color::Green,
            is_bold: false
        },
        Distro {
            name: "EndeavourOS".to_string(),
            description: "Lorem ipsum".to_string(),
            color: Color::Magenta,
            is_bold: false
        },
        Distro {
            name: "OpenSUSE".to_string(),
            description: "Lorem ipsum".to_string(),
            color: Color::Green,
            is_bold: true
        }
    ]
}
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
            description: "Ubuntu is a popular Linux distribution based on Debian. It used to be really nice, but then they added snaps and a lot of bloat. Still a solid choice".to_string(),
            color: Color::Red,
            is_bold: false,
        },
        Distro {
            name: "Fedora".to_string(),
            description: "Fedora is a cutting-edge Linux distribution sponsored by Red Hat. Solid choice, even Linus Torvalds uses it!".to_string(),
            color: Color::Blue,
            is_bold: true,
        },
        Distro {
            name: "Debian".to_string(),
            description: "Debian is a stable and versatile Linux distribution. Pretty much the holy grail of modern distros.".to_string(),
            color: Color::Red,
            is_bold: false,
        },
        Distro {
            name: "Arch Linux".to_string(),
            description: "Arch Linux is a lightweight and flexible Linux distribution. Go ahead, say the words, you use Arch btw..".to_string(),
            color: Color::Cyan,
            is_bold: true,
        },
        Distro {
            name: "openSUSE".to_string(),
            description: "openSUSE is a versatile and stable Linux distribution. And I like the little chameleon".to_string(),
            color: Color::BrightGreen,
            is_bold: true,
        },
        Distro {
            name: "Linux Mint".to_string(),
            description: "Linux Mint is a user-friendly Linux distribution based on Ubuntu. Really a solid choice if you want to get work done. Used this one a long time myself! :D".to_string(),
            color: Color::Green,
            is_bold: false,
        },
        Distro {
            name: "Gentoo".to_string(),
            description: "Gentoo is a flexible and source-based Linux distribution. Have fun compling stuff for hours haha!".to_string(),
            color: Color::Magenta,
            is_bold: true,
        },
        Distro {
            name: "Void Linux".to_string(),
            description: "Void Linux is an independent Linux distribution focused on simplicity. Extra points because this one sounds really cool".to_string(),
            color: Color::Green,
            is_bold: true,
        },
        Distro {
            name: "EndeavourOS".to_string(),
            description: "EndeavourOS is Arch but without all the pain. Really nice choice! :)".to_string(),
            color: Color::Magenta,
            is_bold: true,
        },
        Distro {
            name: "NixOS".to_string(),
            description: "NixOS is a Linux distribution based on Nix package manager. It's really cool if you like to manage your system with code and stuff".to_string(),
            color: Color::BrightMagenta,
            is_bold: true,
        },
        Distro {
            name: "Linux From Scratch".to_string(),
            description: "Linux From Scratch huh? So you have chosen death. Just kidding, this one is really cool if you want to learn how Linux works from the ground up".to_string(),
            color: Color::BrightRed,
            is_bold: false,
        },
    ]
}

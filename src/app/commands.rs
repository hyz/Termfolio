mod fetch;
mod texts;
pub use fetch::get_prompt;

pub async fn run_command(command: &str, args: &Vec<String>) -> Result<String, String> {
    let result = match command {
        "help" | "termfolio" => texts::HELP,
        "cd" => "Nowhere to go.",
        "mkdir" | "touch" => "Nowhere to create.",
        "rm" | "rmdir" => "Nothing to destroy.",
        "cp" => "Nothing to duplicate.",
        "mv" => "Nowhere to move.",
        "ls" | "cat" => "Nothing to see.",
        "grep" | "which" | "find" => "Nowhere to search.",
        "pwd" => "You are here.",
        "nano" | "vi" | "vim" | "nvim" | "hx" => "Great editor.",
        "emacs" => "Great mail client",
        "su" | "sudo" | "chmod" => "With great power comes great responsibility.",
        "whoami" => "Despite everything, it's still you.",
        "exit" => "Hasta la vista.",
        "links" => fetch::get_contacts(),
        "credits" => texts::CREDITS,
        "" => "",
        _ => {
            return Ok(match command {
                "about" => fetch::get_about(),
                "github" | "neofetch" | "fastfetch" => fetch::get_github().await,
                "repos" | "onefetch" => fetch::get_repos().await,
                "echo" => args.join(" "),
                _ => return Err(format!("{command}: command not found")),
            })
        }
    };
    Ok(result.trim().into())
}

pub fn autocomplete(inp: &str) -> &str {
    let inp = inp.trim();

    let comms = [
        "help",
        "history",
        "about",
        "github",
        "repos",
        "links",
        "theme",
        "wal",
        "credits",
        "onefetch",
        "neofetch",
        "fastfetch",
    ];

    if !inp.is_empty() {
        for &c in comms.iter() {
            if c.starts_with(inp) {
                return c;
            }
        }
    }

    inp
}

pub fn banner() -> String {
    String::from(texts::HELP)
}

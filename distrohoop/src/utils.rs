use colored::Colorize;
use rand::{seq::SliceRandom, Rng};
use crossterm::{
    cursor, execute, style::{Print, ResetColor, SetBackgroundColor, Color}, terminal::{Clear, ClearType}
};
use std::{
    ffi::OsString, io::{stdout, Result, Write}, thread::sleep, time::Duration
};
use crate::distros::{self, Distro};

struct Star {
    x: u16,
    y: u16,
    char: char,
}

fn get_star(rng: &mut rand::rngs::ThreadRng, cols: u16, rows: u16) -> Star {
    let x = rng.gen_range(0..cols);
    let y = rng.gen_range(0..rows);
    let star_chars = ['*', '.', '+', '·', '°'];
    let star_char = star_chars[rng.gen_range(0..star_chars.len())];
    Star { x, y, char: star_char}
}

fn get_messages() -> Vec<String> {
    let mut distros = distros::get_distros();
    let mut rng = rand::thread_rng();
    distros.shuffle(&mut rng);

    let selected_distros = distros.iter().take(3).collect::<Vec<&Distro>>();
    let selected_distro = selected_distros[rng.gen_range(0..selected_distros.len())];
    
    let mut messages = vec![
        "The stars are thinking...".to_string(),
        "A decision is forming...".to_string(),
        "The next distro will be decided soon...".to_string(),
        "Patience is a virtue in the cosmic dance...".to_string(),
        "The universe is aligning...".to_string(),
        
    ];

    for dist in selected_distros {
        let colored_name: String = if dist.is_bold {
            format!("I am thinking about.. {}?", dist.name.color(dist.color).bold().to_string())
        } else {
            format!("I am thinking about.. {}?", dist.name.color(dist.color).to_string())
        };
        messages.push(colored_name);
    }

    messages.push("A decision has been made..!".to_string());

    let final_message = if selected_distro.is_bold {
        let name_part = format!(
            "Your destined distribution is: {}",
            selected_distro.name.color(selected_distro.color).bold()
        );
        format!(
            "{}\n{}",
            name_part,
            selected_distro.description.to_string()
        )
    } else {
        let name_part = format!(
            "Your destined distribution is: {}",
            selected_distro.name.color(selected_distro.color)
        );
        format!(
            "{}\n{}",
            name_part,
            selected_distro.description.to_string()
        )
    };
    messages.push(final_message);

    messages
}


pub fn play_animation() -> Result<()> { 
    let mut stdout = stdout();
    let (cols, rows) = crossterm::terminal::size()?;
    let mut rng = rand::thread_rng();
    
    // create a mutable vector that stores the position of the stars and the char/representative for the star
    let mut stars: Vec<Star> =  Vec::new();
    
    // Initialize some random stars into the vec
    for _ in 0..200 {
        // create a value for the x and y based on the users INITIAL terminal size - I think if they make the Terminal bigger it gets messed up
        stars.push(get_star(&mut rng, cols, rows));
    }

    //Get the Messages

    let messages = get_messages();

    // Animation loop - Run animation for x frames
    for (i, message) in messages.iter().enumerate() {
        // Determine frames based on message position
        let frames = if i == messages.len() - 1 {
            50  
        } else {
            20  
        };
    
        for _ in 0..frames {
            execute!(stdout, Clear(ClearType::All))?;
            
            // Draw the generated stars
            for star in &stars {
                execute!(
                    stdout,
                    cursor::MoveTo(star.x, star.y),
                    Print(star.char.to_string().yellow())
                )?;
            }
    
            // Display the messages
            if message.contains('\n') {
                let parts: Vec<&str> = message.split('\n').collect();
                let part0_x = (cols as i16 / 2 - parts[0].len() as i16 / 2).max(0) as u16;
                let part1_x = (cols as i16 / 2 - parts[1].len() as i16 / 2).max(0) as u16;
                execute!(
                    stdout,
                    cursor::MoveTo(part0_x, rows / 2),
                    Print(parts[0]),
                    cursor::MoveTo(part1_x, rows / 2 + 1),
                    Print(parts[1])
                )?;
            } else {
                let message_x = (cols as i16 / 2 - message.len() as i16 / 2).max(0) as u16;
                execute!(
                    stdout,
                    cursor::MoveTo(message_x, rows / 2),
                    Print(message)
                )?;
            }
    
            // Update star positions (make them twinkle by moving slightly)
            for star in &mut stars {
                if rng.gen_bool(0.2) { // 20% chance to move a star
                    star.x = (star.x as i16 + rng.gen_range(-1..=1))
                        .max(0)
                        .min(cols as i16 - 1) as u16;
                    star.y = (star.y as i16 + rng.gen_range(-1..=1))
                        .max(0)
                        .min(rows as i16 - 1) as u16;
                }
            }
    
            // Add occasional new stars
            if rng.gen_bool(0.1) && stars.len() < 100 {
                stars.push(get_star(&mut rng, cols, rows));
            }
            // Remove occasional stars
            if rng.gen_bool(0.1) && stars.len() > 40 {
                let index = rng.gen_range(0..stars.len());
                stars.remove(index);
            }
    
            stdout.flush()?;
            sleep(Duration::from_millis(100));
        }
    }
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, ResetColor)?;
    Ok(())
    }
   

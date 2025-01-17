use colored::Colorize;
use rand::{seq::SliceRandom, Rng};
use crossterm::{
    cursor, execute, style::{Print, ResetColor, SetBackgroundColor, Color}, terminal::{Clear, ClearType}
};
use std::{
    io::{stdout, Result, Write}, thread::sleep, time::Duration
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

    messages
}


pub fn play_animation() -> Result<()> { 
    let mut stdout = stdout();
    let (cols, rows) = crossterm::terminal::size()?;
    let mut rng = rand::thread_rng();
    
    // create a mutable vector that stores the position of the stars and the char/representative for the star
    let mut stars: Vec<Star> =  Vec::new();
    
    // Initialize some random stars into the vec
    for _ in 0..50 {
        // create a value for the x and y based on the users INITIAL terminal size - I think if they make the Terminal bigger it gets messed up
        stars.push(get_star(&mut rng, cols, rows));
    }

    //Get the Messages

    let messages = get_messages();
    let message_duration = Duration::from_secs(2);
    let frames_per_msg = (message_duration.as_secs_f64()/0.1) as usize;

    // Animation loop - Run animation for x frames

    for message in messages.iter() {
        for _ in 0..frames_per_msg {
            execute!(stdout, Clear(ClearType::All))?;
            execute!(stdout, SetBackgroundColor(Color::Rgb { r:0, g:2, b:21}))?;
            
            // Draw the generated stars
            for star in &stars {
                execute!(
                    stdout,
                    cursor::MoveTo(star.x, star.y),
                    Print(star.char.to_string().yellow())
                )?;
            }

        // Display the messages
        execute!(
            stdout,
            cursor::MoveTo(cols / 2 - (message.len() as u16 / 2), rows / 2),
            Print(message)
        )?;

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
        if rng.gen_bool(0.1) && stars.len() > 30 {
            let index = rng.gen_range(0..stars.len());
                stars.remove(index);
        }
        stdout.flush()?;
        sleep(Duration::from_millis(100));
        }
    }
    execute!(stdout, ResetColor)?;
    Ok(())
    }
   

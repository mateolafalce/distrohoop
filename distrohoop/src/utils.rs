use colored::Colorize;
use rand::Rng;
use crossterm::{
    cursor, execute, style::{Print, ResetColor, SetBackgroundColor, Color}, terminal::{Clear, ClearType}
};
use std::{
    io::{stdout, Result, Write}, thread::sleep, time::Duration
};
use crate::distros::{self, Distro};

fn get_star(rng: &mut rand::rngs::ThreadRng, cols: u16, rows: u16) -> (u16,u16,char) {
    let x = rng.gen_range(0..cols);
    let y = rng.gen_range(0..rows);
    let star_chars = ['*', '.', '+', '·', '°'];
    let star_char = star_chars[rng.gen_range(0..star_chars.len())];
    (x,y,star_char)
}

fn get_messages() -> Vec<String> {
    let distros = distros::get_distros();
    let selected_distros = distros.iter().take(3).collect::<Vec<&Distro>>();

    let mut messages = vec![
        "The stars are thinking...".to_string(),
        "A decision is forming...".to_string(),
        "The next distro will be decided soon...".to_string(),
        "Patience is a virtue in the cosmic dance...".to_string(),
        "The universe is aligning...".to_string(),
    ];

    for dist in selected_distros {
        let colored_name = dist.name.color(dist.color).to_string();
        messages.push(colored_name);
    }

    messages
}


pub fn play_animation() -> Result<()> { 
    let mut stdout = stdout();
    let (cols, rows) = crossterm::terminal::size()?;
    let mut rng = rand::thread_rng();
    
    // create a mutable vector that stores the position of the stars and the char/representative for the star
    let mut stars: Vec<(u16, u16, char)> =  Vec::new();
    
    // Initialize some random stars into the vec
    for _ in 0..50 {
        // create a value for the x and y based on the users INITIAL terminal size - I think if they make the Terminal bigger it gets messed up
        stars.push(get_star(&mut rng, cols, rows));
    }

    // Animation loop - Run animation for x frames

    for _ in 0..55 {
        execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, SetBackgroundColor(Color::Rgb { r:0, g:2, b:21}))?;
        // draw the generated stars
        for(x,y,star_char) in &stars {
            execute!(
                stdout,
                cursor::MoveTo(*x,*y),
                Print(star_char.to_string().yellow())
            )?;
        }

            // Update star positions (make them twinkle by moving slightly)
        for star in &mut stars {
            if rng.gen_bool(0.2) { // 20% chance to move a star
                star.0 = (star.0 as i16 + rng.gen_range(-1..=1))
                    .max(0)
                    .min(cols as i16 - 1) as u16;
                star.1 = (star.1 as i16 + rng.gen_range(-1..=1))
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
            stars.remove(rng.gen_range(0..stars.len()));
        }
        stdout.flush()?;
        sleep(Duration::from_millis(100));
    }
    execute!(stdout, ResetColor)?;
    Ok(())
    }
   

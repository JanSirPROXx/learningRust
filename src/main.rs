use std::collections;

fn main() {
    
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut _snake: Vec<(u16, u16)> = vec![(5, 5)];

    let mut _fruit: (u16, u16) = (10, 10);

    #[derive(PartialEq)] //Alows != comparison
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let terminal_size: (u16, u16) = crossterm::terminal::size().unwrap();

    let mut direction: Direction = Direction::Right;

    //game loop

    loop {
        // clear terminal
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();

        // draw snake
        for segment in &_snake {
            crossterm::execute!(
                std::io::stdout(),
                crossterm::cursor::MoveTo(segment.0, segment.1),
                crossterm::style::Print("O")
            )
            .unwrap();
        }

        // draw fruit
        crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::MoveTo(_fruit.0, _fruit.1),
            crossterm::style::Print("@")
        )
        .unwrap();

        //get key input
        if crossterm::event::poll(std::time::Duration::from_millis(0)).unwrap()
            && let crossterm::event::Event::Key(key_event) = crossterm::event::read().unwrap()
        {
            match key_event.code {
                crossterm::event::KeyCode::Up => {
                    if direction != Direction::Down {
                        direction = Direction::Up;
                    }
                }
                crossterm::event::KeyCode::Down => {
                    if direction != Direction::Up {
                        direction = Direction::Down;
                    }
                }
                crossterm::event::KeyCode::Left => {
                    if direction != Direction::Right {
                        direction = Direction::Left;
                    }
                }
                crossterm::event::KeyCode::Right => {
                    if direction != Direction::Left {
                        direction = Direction::Right;
                    }
                }
                crossterm::event::KeyCode::Esc => {
                    crossterm::terminal::disable_raw_mode().unwrap();
                    break;
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        //update snake states
        let newhead = match direction {
            Direction::Up => (_snake[0].0, _snake[0].1.saturating_sub(1)),
            Direction::Down => (_snake[0].0, _snake[0].1.saturating_add(1)),
            Direction::Left => (_snake[0].0.saturating_sub(1), _snake[0].1),
            Direction::Right => (_snake[0].0.saturating_add(1), _snake[0].1),
        };

        // wall collision
        if newhead.0 >= terminal_size.0 || newhead.1 >= terminal_size.1 {
            crossterm::terminal::disable_raw_mode().unwrap();
            break; // End game if snake hits the wall
        }

        //self collision
        for segment in &_snake {
            if newhead == *segment {
                crossterm::terminal::disable_raw_mode().unwrap();
                return; 
            }
        }

        // fruit collision
        if newhead == _fruit {
            _fruit = (
                rand::random::<u16>() % terminal_size.0,
                rand::random::<u16>() % terminal_size.1,
            );
        } else {
            //remove snake tail
            _snake.pop();
        }

        //add new head
        _snake.insert(0, newhead);
    }
}

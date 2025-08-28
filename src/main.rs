use minifb::{Key, Window, WindowOptions};
use rand::Rng;
const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const CELL_SIZE: usize = 20; // size of each cell in pixels

struct Game {
    grid: Vec<Vec<bool>>,
    first_display: bool // only run on first intial state
}

impl Game {

    // at start of game a random grid/initial state is created
    fn new() -> Game {
        
        let mut rng = rand::thread_rng();
        let grid: Vec<Vec<bool>> = (0..32)
        .map(|_| {
            (0..32)
                .map(|_| rng.gen_bool(0.10)) // 10% chance true/false as alive cells should be rarer initially
                .collect()
        })
        .collect();

        Game {
            grid,
            first_display: false
        }

    }
    // mainly called for testing drawing - just call reset to test how fast frame rate looks
    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.grid = (0..32)
        .map(|_| {
            (0..32)
                .map(|_| rng.gen_bool(0.10)) // 10% chance true/false as alive cells should be rarer initially
                .collect()
        })
        .collect();
        self.first_display = false;
    }
}

fn main() {
    // minifb lib schema - basic buffer and window setup
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Game of Life Simulation - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(10);

    // game setup - init and create some randomized initial state
    let mut game = Game::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        draw_grid(&mut buffer);
        
        draw_state(&mut game, &mut buffer);
        

        if window.is_key_down(Key::R) {
            game.reset();
        }

        // minifb lib schema
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }



}

fn draw_grid(buffer: &mut [u32]) {
    // clear screen to black
    for i in buffer.iter_mut() {
        *i = 0x000000; 
    }

    // draw vertical grid lines
    for x in (0..WIDTH).step_by(CELL_SIZE) {
        for y in 0..HEIGHT {
            buffer[y * WIDTH + x] = 0xFFFFFF; 
        }
    }
    
    // draw horizontal grid lines
    for y in (0..HEIGHT).step_by(CELL_SIZE) {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = 0xFFFFFF; 
        }
    }
}

fn draw_cell(buffer: &mut [u32], cell_y: usize, cell_x: usize, color: u32) {
    for x in 0..CELL_SIZE {
        for y in 0..CELL_SIZE {
            let py = cell_y * CELL_SIZE + y;
            let px = cell_x * CELL_SIZE + x;
            if py < WIDTH && px < HEIGHT {
                buffer[px * WIDTH + py] = color;
            }
        }
    }
}

fn draw_state(game:&mut Game,buffer: &mut [u32]) {
    // we just draw initial state
    if !game.first_display {

        for x in 0..32 {
            for y in 0..32 {
                if game.grid[x][y] {
                    draw_cell(buffer, x, y, 0xFFFFFF); // white cell
                }
            }
        };

        // inital state not drawn again
        // after initial draw - the else branch will always be run 
        // to draw the next sequences that appear in the game
        game.first_display = true;
    } else {
        // to replace with actual state update function 
        //game.reset();

        for x in 0..32 {
            for y in 0..32 {
                if game.grid[x][y] {
                    draw_cell(buffer, x, y, 0xFFFFFF); // white cell
                }
            }
        };
    }
}

//        draw_cell(&mut buffer, 10, 10, 0xFFFFFF); // white cell

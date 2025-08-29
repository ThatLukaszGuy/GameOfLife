mod glyph;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use glyph::Glyphs;


const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const CELL_SIZE: usize = 20; // size of each cell in pixels
const CELL_COUNT:usize = 32; // DIM / CELL_SIZE have this 32x32 grid (cellcount per row/col) 
const NEIGHBORS: [(isize, isize); 8] = [ 
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),         (1,  0),
    (-1,  1), (0,  1), (1,  1),
]; // easier to check live neighbors
const PROB: f64 = 0.05;

struct Game {
    grid: Vec<Vec<bool>>,
    first_display: bool, // only run on first intial state
    generation: usize
}

impl Game {

    // at start of game a random grid/initial state is created
    fn new() -> Game {
        
        let mut rng = rand::thread_rng();
        let grid: Vec<Vec<bool>> = (0..CELL_COUNT)
        .map(|_| {
            (0..32)
                .map(|_| rng.gen_bool(PROB)) // 10% chance true/false as alive cells should be rarer initially
                .collect()
        })
        .collect();

        Game {
            grid,
            first_display: false,
            generation: 0
        }

    }
    // reset board to some fresh random state
    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.grid = (0..CELL_COUNT)
        .map(|_| {
            (0..CELL_COUNT)
                .map(|_| rng.gen_bool(PROB)) // 10% chance true/false as alive cells should be rarer initially
                .collect()
        })
        .collect();
        self.first_display = false;
        self.generation = 0;
    }

    fn compute_next_state(&mut self) {
        // to ensure a "synchronous" update keep a snapshot of old state and from it sequentially while referincng it 
        // compute next state in a seperate grid -> then set that grid as the new grid

        let mut updated_grid:Vec<Vec<bool>> = vec![vec![false; CELL_COUNT]; CELL_COUNT];

        // each entry in the new grid is constructed from viewing the {entire/partly} old grid 
        // RULES: (taken from https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
        // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // 2. Any live cell with two or three live neighbours lives on to the next generation.
        // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
        // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        
        for x in 0..CELL_COUNT {
            for y in 0..CELL_COUNT {
                
                let alive_count = self.live_neighbors(x as isize, y as isize);
                    
                if self.grid[x][y] {
                    // rule 1.
                    if alive_count < 2 { updated_grid[x][y] = true }
                    // rule 2.
                    if alive_count == 2 || alive_count == 3 { updated_grid[x][y] = true }
                    // rule 3.
                    if alive_count > 3 { updated_grid[x][y] = false }
                }
                // rule 4.
                if !self.grid[x][y] && alive_count == 3 { updated_grid[x][y] = true }
            }
        }

        self.grid = updated_grid;
        self.generation += 1;
    }

    fn live_neighbors(&mut self,x:isize,y:isize) -> usize {
        NEIGHBORS.iter().filter(|&(dx,dy)| {
            let row: isize = x + dx;
            let col: isize= y + dy;
                   
            if row >= 0 && row < CELL_COUNT as isize && col >= 0 && col < CELL_COUNT as isize {
                self.grid[row as usize][col as usize]
            } else {
                false
            }
        }).count()
    }

}

fn main() {
    // minifb lib schema - basic buffer and window setup
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    

    let mut window = Window::new(
        "Game of Life Simulation - ESC to exit - R to reset",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(5);

    // game setup - init and create some randomized initial state
    let mut game = Game::new();
    let mut speed = 1;
    let glyphs = Glyphs::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {

        for _ in 0..speed {
            draw_grid(&mut buffer);
        
            draw_state(&mut game, &mut buffer);
            
            draw_text(&mut buffer, &mut game, &glyphs);
        }
        
        // controls

        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) { speed+=1; }

        if ( window.is_key_down(Key::Down)|| window.is_key_down(Key::S) ) && speed > 1 { speed-=1; }

        if window.is_key_down(Key::R) { game.reset(); }

        // minifb lib schema
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }



}

fn draw_grid(buffer: &mut [u32]) {
    // clear screen to light beige
    for i in buffer.iter_mut() {
        *i = 0xF4EBD3; 
    }

    // line color is darker beige
    // draw vertical grid lines
    for x in (0..WIDTH).step_by(CELL_SIZE) {
        for y in 0..HEIGHT {
            buffer[y * WIDTH + x] = 0xDED3C4; 
        }
    }
    
    // draw horizontal grid lines
    for y in (0..HEIGHT).step_by(CELL_SIZE) {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = 0xDED3C4; 
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

        for x in 0..CELL_COUNT {
            for y in 0..CELL_COUNT {
                if game.grid[x][y] {
                    draw_cell(buffer, x, y, 0x555879); 
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
        game.compute_next_state();

        for x in 0..CELL_COUNT {
            for y in 0..CELL_COUNT {
                if game.grid[x][y] {
                    draw_cell(buffer, x, y, 0x555879); 
                }
            }
        };
    }
}


fn draw_text(buffer: &mut [u32], game: &mut Game, glyphs: &Glyphs) {
    
    let generation = game.generation;

    let color = 0x98A1BC; 
    // "generation:"
    draw_letter(&glyphs.g, 10, color, buffer);
    draw_letter(&glyphs.e, 20, color, buffer);
    draw_letter(&glyphs.n, 30, color, buffer);
    draw_letter(&glyphs.e, 40, color, buffer);
    draw_letter(&glyphs.r, 50, color, buffer);
    draw_letter(&glyphs.a,60, color, buffer);
    draw_letter(&glyphs.t, 70, color, buffer);
    draw_letter(&glyphs.i, 80, color, buffer);
    draw_letter(&glyphs.o, 90, color, buffer);
    draw_letter(&glyphs.n, 100, color, buffer);
    draw_letter(&glyphs.double_colon, 110, color, buffer);


    // "count" - pull apart and render each digit
    let mut dx = 0;
    generation.to_string().chars().for_each(|digit| {
        match digit.to_digit(10) {
            Some(9) => {draw_letter(&glyphs.d9, 120+ dx, color, buffer); dx+=10;},
            Some(8) => {draw_letter(&glyphs.d8, 120+ dx, color, buffer); dx+=10;},
            Some(7) => {draw_letter(&glyphs.d7, 120+ dx, color, buffer); dx+=10;},
            Some(6) => {draw_letter(&glyphs.d6, 120+ dx, color, buffer); dx+=10;},
            Some(5) => {draw_letter(&glyphs.d5, 120+ dx, color, buffer); dx+=10;},
            Some(4) => {draw_letter(&glyphs.d4, 120+ dx, color, buffer); dx+=10;},
            Some(3) => {draw_letter(&glyphs.d3, 120+ dx, color, buffer); dx+=10;},
            Some(2) => {draw_letter(&glyphs.d2, 120+ dx, color, buffer); dx+=10;},
            Some(1) => {draw_letter(&glyphs.d1, 120+ dx, color, buffer); dx+=10;},
            Some(0) => {draw_letter(&glyphs.d0, 120+ dx, color, buffer); dx+=10;},
            _ => {}
        }
    });

    draw_instructions(buffer, &glyphs);
}

fn draw_letter(letter: &[[u8;5]; 5] , offset: usize, color: u32, buffer: &mut [u32] ) {

    for (y, row) in letter.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            if pixel == 1 {
                buffer[(y + 10) * WIDTH + (x + offset)] = color; // offset 10,10
            }
        }
    }
}

// draw_cell(&mut buffer, 10, 10, 0xFFFFFF); // white cell
// 0x555879 navy
// 0x98A1BC lighter blue

fn draw_instructions(buffer: &mut [u32], glyphs: &Glyphs) {

    let color = 0x98A1BC; 
    // Speed UP DOWN : W S
    
    fn draw_letter_instr(letter: &[[u8;5]; 5] , offset: usize, color: u32, buffer: &mut [u32] ) {

        for (y, row) in letter.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel == 1 {
                    buffer[(y + 20) * WIDTH + (x + offset)] = color; // offset 10,10
                }
            }
        }
    }

    draw_letter_instr(&glyphs.s, 10, color, buffer);
    draw_letter_instr(&glyphs.p, 20, color, buffer);
    draw_letter_instr(&glyphs.e, 30, color, buffer);
    draw_letter_instr(&glyphs.e, 40, color, buffer);
    draw_letter_instr(&glyphs.d, 50, color, buffer);

    draw_letter_instr(&glyphs.u, 70, color, buffer);
    draw_letter_instr(&glyphs.p, 80, color, buffer);
    draw_letter_instr(&glyphs.double_colon, 90, color, buffer);
    draw_letter_instr(&glyphs.w, 100, color, buffer);

    draw_letter_instr(&glyphs.d, 120, color, buffer);
    draw_letter_instr(&glyphs.o, 130, color, buffer);
    draw_letter_instr(&glyphs.w, 140, color, buffer);
    draw_letter_instr(&glyphs.n, 150, color, buffer); 
    draw_letter_instr(&glyphs.double_colon, 160, color, buffer);
    draw_letter_instr(&glyphs.s, 170, color, buffer);

}
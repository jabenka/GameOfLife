use simple::{Key, MouseButton, Rect, Window};
use std::thread;
use std::time::Duration;

const FPS: u32 = 2;
const SCREEN_HEIGHT: u16 = 1080;
const SCREEN_WIDTH: u16 = 1920;
const DRAW_TIME: u64 = (1000 / FPS) as u64;
const TILE_SIZE: i32 = 24;
const COL_NUM: usize = (SCREEN_WIDTH / TILE_SIZE as u16) as usize; //80
const ROW_NUM: usize = (SCREEN_HEIGHT / TILE_SIZE as u16) as usize; // 45
const TILE_COLOR: (u8, u8, u8, u8) = (100, 200, 170, 255);
const BACKGROUND_COLOR: (u8, u8, u8, u8) = (30, 30, 40, 255);
#[derive(Clone, Copy)]
struct Tile {
    x_pos: i32,
    y_pos: i32,
    alive: bool,
}

fn main() {
    let mut app = Window::new("Game of Life", SCREEN_WIDTH, SCREEN_HEIGHT);
    app.clear_to_color(255, 255, 255);
    app.set_color(255, 255, 255, 255);
    let mut tiles: [[Tile; COL_NUM]; ROW_NUM] = get_start_points();
    let mut stop = true;
    let mut stop_pressed_before = false;
    let mut mouse_pressed_before = false;
    while app.next_frame() {
        draw_screen(&mut app, &tiles, BACKGROUND_COLOR, TILE_COLOR);
        let stop_pressed_now = app.is_key_down(Key::Space);
        if stop_pressed_now && !stop_pressed_before {
            stop = !stop;
        }
        stop_pressed_before = stop_pressed_now;
        let mouse_pressed_now = app.is_mouse_button_down(MouseButton::Left);
        if mouse_pressed_now && !mouse_pressed_before {
            let pos = app.mouse_position();
            let col = (pos.0 / TILE_SIZE) as usize;
            let row = (pos.1 / TILE_SIZE) as usize;
            let alive = tiles[row][col].alive;
            tiles[row][col].alive = !alive;
        }
        mouse_pressed_before = mouse_pressed_now;
        if !stop {
            update(&mut tiles);
        }
    }
}

fn get_start_points() -> [[Tile; COL_NUM]; ROW_NUM] {
    let mut points: [[Tile; COL_NUM]; ROW_NUM] = [[Tile {
        x_pos: 0,
        y_pos: 0,
        alive: true,
    }; COL_NUM]; ROW_NUM];
    for i in 0..ROW_NUM as i32 {
        for j in 0..COL_NUM as i32 {
            let tile = Tile {
                x_pos: j * TILE_SIZE,
                y_pos: i * TILE_SIZE,
                alive: false,
            };
            points[i as usize][j as usize] = tile;
        }
    }
    //init_glider_gun(&mut points);
    points
}

fn init_glider_gun(points: &mut [[Tile; COL_NUM]; ROW_NUM]) {
    let gun_cells: [(usize, usize); 36] = [
        (5, 1), (5, 2), (6, 1), (6, 2),
        (5, 11), (6, 11), (7, 11),
        (4, 12), (8, 12),
        (3, 13), (9, 13),
        (3, 14), (9, 14),
        (6, 15),
        (4, 16), (8, 16),
        (5, 17), (6, 17), (7, 17),
        (6, 18),
        (3, 21), (4, 21), (5, 21),
        (3, 22), (4, 22), (5, 22),
        (2, 23), (6, 23),
        (1, 25), (2, 25), (6, 25), (7, 25),
        (3, 35), (4, 35),
        (3, 36), (4, 36),
    ];

    for (row, col) in gun_cells.iter() {
        points[*row][*col].alive = true;
    }
}


fn update(tiles: &mut [[Tile; COL_NUM]; ROW_NUM]) {
    let mut to_alive: Vec<Tile> = Vec::new();
    let mut to_die: Vec<Tile> = Vec::new();
    for i in 0..ROW_NUM {
        for j in 0..COL_NUM {
            let tile = tiles[i][j];
            let alive_neighbours = check_alive_neighbours(&tiles, tile);
            if (!tile.alive && alive_neighbours == 3) {
                to_alive.push(tile);
            } else if tile.alive {
                if alive_neighbours > 3 || alive_neighbours < 2 {
                    to_die.push(tile);
                }
            }
        }
    }
    for tile in to_alive {
        let col = tile.x_pos / TILE_SIZE;
        let row = tile.y_pos / TILE_SIZE;
        tiles[row as usize][col as usize].alive = true;
    }
    for tile in to_die {
        let col = tile.x_pos / TILE_SIZE;
        let row = tile.y_pos / TILE_SIZE;
        tiles[row as usize][col as usize].alive = false;
    }
    thread::sleep(Duration::from_millis(DRAW_TIME));
}

fn check_alive_neighbours(tiles: &&mut [[Tile; COL_NUM]; ROW_NUM], tile: Tile) -> u8 {
    let tile_col = tile.x_pos / TILE_SIZE;
    let tile_row = tile.y_pos / TILE_SIZE;
    let mut alive_neighbours: u8 = 0;

    if tile_col - 1 >= 0 && tiles[tile_row as usize][(tile_col - 1) as usize].alive
    //left
    {
        alive_neighbours += 1;
    }
    if tile_col + 1 < COL_NUM as i32 && tiles[tile_row as usize][(tile_col + 1) as usize].alive
    //right
    {
        alive_neighbours += 1;
    }
    if tile_row + 1 < ROW_NUM as i32 && tiles[(tile_row + 1) as usize][tile_col as usize].alive
    //up
    {
        alive_neighbours += 1;
    }
    if tile_row - 1 >= 0 && tiles[(tile_row - 1) as usize][tile_col as usize].alive
    //down
    {
        alive_neighbours += 1;
    }
    if tile_col - 1 >= 0
        && tile_row + 1 < ROW_NUM as i32
        && tiles[(tile_row + 1) as usize][(tile_col - 1) as usize].alive
    // upper_left
    {
        alive_neighbours += 1;
    }
    if tile_col + 1 < COL_NUM as i32
        && tile_row + 1 < ROW_NUM as i32
        && tiles[(tile_row + 1) as usize][(tile_col + 1) as usize].alive
    // upper_right
    {
        alive_neighbours += 1;
    }
    if tile_col - 1 >= 0
        && tile_row - 1 >= 0
        && tiles[(tile_row - 1) as usize][(tile_col - 1) as usize].alive
    // lower_left
    {
        alive_neighbours += 1
    }
    if tile_col + 1 < COL_NUM as i32
        && tile_row - 1 >= 0
        && tiles[(tile_row - 1) as usize][(tile_col + 1) as usize].alive
    // lower_right
    {
        alive_neighbours += 1;
    }
    alive_neighbours
}

fn draw_screen(
    app: &mut Window,
    tiles: &[[Tile; COL_NUM]; ROW_NUM],
    background_color: (u8, u8, u8, u8),
    tile_color: (u8, u8, u8, u8),
) {
    for i in 0..ROW_NUM {
        for j in 0..COL_NUM {
            let tile = tiles[i][j];
            let color = if tile.alive {
                tile_color
            } else {
                background_color
            };
            app.set_color(color.0, color.1, color.2, color.3);

            app.fill_rect(Rect::new(
                tile.x_pos,
                tile.y_pos,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ));
            app.set_color(60, 60, 70, 255);
            app.draw_rect(Rect::new(
                tile.x_pos,
                tile.y_pos,
                TILE_SIZE as u32,
                TILE_SIZE as u32,
            ));
        }
    }
}

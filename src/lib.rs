
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index))
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
       self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx - 1) % self.size;
    }
}

// wasm-pack build --target web

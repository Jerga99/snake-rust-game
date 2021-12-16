/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Direction {
  Up,
  Right,
  Down,
  Left,
}
/**
*/
export enum GameStatus {
  Won,
  Lost,
  Played,
}
/**
*/
export class World {
  free(): void;
/**
* @param {number} width
* @param {number} snake_idx
* @returns {World}
*/
  static new(width: number, snake_idx: number): World;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  points(): number;
/**
* @returns {number | undefined}
*/
  reward_cell(): number | undefined;
/**
* @returns {number}
*/
  snake_head_idx(): number;
/**
*/
  start_game(): void;
/**
* @returns {number | undefined}
*/
  game_status(): number | undefined;
/**
* @returns {string}
*/
  game_status_text(): string;
/**
* @param {number} direction
*/
  change_snake_dir(direction: number): void;
/**
* @returns {number}
*/
  snake_length(): number;
/**
* @returns {number}
*/
  snake_cells(): number;
/**
*/
  step(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_world_free: (a: number) => void;
  readonly world_new: (a: number, b: number) => number;
  readonly world_width: (a: number) => number;
  readonly world_points: (a: number) => number;
  readonly world_reward_cell: (a: number, b: number) => void;
  readonly world_snake_head_idx: (a: number) => number;
  readonly world_start_game: (a: number) => void;
  readonly world_game_status: (a: number) => number;
  readonly world_game_status_text: (a: number, b: number) => void;
  readonly world_change_snake_dir: (a: number, b: number) => void;
  readonly world_snake_length: (a: number) => number;
  readonly world_snake_cells: (a: number) => number;
  readonly world_step: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

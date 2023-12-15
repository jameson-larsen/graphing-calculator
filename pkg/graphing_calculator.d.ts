/* tslint:disable */
/* eslint-disable */
/**
* @param {number} x_start
* @param {number} x_end
* @param {number} y_start
* @param {number} y_end
*/
export function run(x_start: number, x_end: number, y_start: number, y_end: number): void;
/**
*/
export function reset(): void;
/**
* @param {any} expressions
* @returns {any}
*/
export function initialize(expressions: any): any;
/**
* @returns {boolean}
*/
export function expand_cache(): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: (a: number, b: number, c: number, d: number) => void;
  readonly reset: () => void;
  readonly initialize: (a: number) => number;
  readonly expand_cache: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

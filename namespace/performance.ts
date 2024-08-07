// deno-lint-ignore-file no-unused-vars
/**
 * Performance namespace.
 */
const performance = {
    /**
     * The `now` function returns the current time in milliseconds.
     *
     * @example
     * ```ts
     * console.log(Performance.now());
     * ```
     */
    now() {
        return internal_now();
    },
};

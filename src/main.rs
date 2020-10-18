/* intended workflow:
 *
 * - Read rules from convoy.toml files
 * - Construct DAG from rules
 * - Check target
 * - Check dependencies
 * - Run rules
 */

mod action;
mod rule;
mod target;

fn main() {
    println!("Hello, world!");
}

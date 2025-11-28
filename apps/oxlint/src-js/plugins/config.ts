import { setOptions } from "./options.js";

/**
 * Populates Rust-resolved configuration options on the JS side.
 * Called from Rust side after all configuration options have been resolved.
 *
 * Note: the name `setupConfigs` is currently incorrect, as we only populate rule options.
 * The intention is for this function to transfer all configurations in a multi-config workspace.
 * The configuration relevant to each file would then be resolved on the JS side.
 *
 * @param optionsJSON - JSON serialization of an array containing all rule options across all configurations.
 * @returns "ok" on success, or error message on failure
 */
export function setupConfigs(optionsJSON: string): string {
  // TODO: setup settings using this function
  try {
    setOptions(optionsJSON);
    // TODO: flesh out error handling.
    // Currently, the only procedure that may fail is `JSON.parse()`
    // in `setOptions()`, but it won't because the JSON string from
    // the rust side is serde serialized.
    return "ok";
  } catch (err) {
    return err instanceof Error ? err.message : String(err);
  }
}

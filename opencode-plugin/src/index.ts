/**
 * Bottle - Cloud Atlas AI Core Stack for OpenCode
 *
 * Meta-plugin bundling ba, wm, and superego in one npm package.
 */

import type { Plugin } from "@opencode-ai/plugin";
import { BA } from "./ba.js";
import { WM } from "./wm.js";
import { Superego } from "./superego.js";

export const Bottle: Plugin[] = [BA, WM, Superego];
export { BA, WM, Superego };
export default Bottle;

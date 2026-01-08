/**
 * Bottle - Cloud Atlas AI Core Stack for OpenCode
 *
 * Two things:
 * 1. npm meta-package - pulls in ba-opencode, wm-opencode, superego-opencode as dependencies
 * 2. OpenCode plugin - provides setup/orchestration tools (bottle:init, bottle:status)
 */

import type { Plugin } from "@opencode-ai/plugin";
import { tool } from "@opencode-ai/plugin";
import { existsSync, writeFileSync } from "fs";
import { join } from "path";
import { spawnSync } from "child_process";

const Bottle: Plugin = async ({ directory }) => {
  return {
    tool: {
      "bottle-init": tool({
        description: "Initialize the full Cloud Atlas AI stack (ba, wm, superego) in one command",
        args: {},
        async execute() {
          const results: string[] = [];

          // Initialize ba
          if (!existsSync(join(directory, ".ba"))) {
            try {
              const ba = spawnSync("ba", ["init"], { cwd: directory, encoding: "utf-8" });
              results.push(ba.status === 0 ? "✓ ba initialized" : `✗ ba init failed: ${ba.stderr}`);
            } catch (e) {
              results.push(`✗ ba init failed: ${e}`);
            }
          } else {
            results.push("✓ ba already initialized");
          }

          // Initialize wm
          if (!existsSync(join(directory, ".wm"))) {
            try {
              const wm = spawnSync("wm", ["init"], { cwd: directory, encoding: "utf-8" });
              results.push(wm.status === 0 ? "✓ wm initialized" : `✗ wm init failed: ${wm.stderr}`);
            } catch (e) {
              results.push(`✗ wm init failed: ${e}`);
            }
          } else {
            results.push("✓ wm already initialized");
          }

          // Initialize superego (via superego tool, not CLI)
          if (!existsSync(join(directory, ".superego"))) {
            results.push("⚠ superego not initialized - use 'superego init' tool");
          } else {
            results.push("✓ superego already initialized");
          }

          // Update AGENTS.md
          const agentsFile = join(directory, "AGENTS.md");
          if (!existsSync(agentsFile)) {
            const content = `# Cloud Atlas AI Stack

Initialized with bottle.

## Tools Available

- **ba**: Task tracking for multi-agent workflows
- **wm**: Working memory for knowledge accumulation
- **superego**: Metacognitive advisor for session evaluation

## Usage

- Task tracking: Use ba tools (ba-init, ba-status, ba-quickstart)
- Working memory: Use wm tool with commands (init, show, compile, distill)
- Metacognition: Use superego tool (status, init, enable, disable)
`;
            writeFileSync(agentsFile, content);
            results.push("✓ AGENTS.md created");
          } else {
            results.push("✓ AGENTS.md already exists");
          }

          return results.join("\n");
        },
      }),

      "bottle-status": tool({
        description: "Check initialization status of all Cloud Atlas AI components",
        args: {},
        async execute() {
          const ba = existsSync(join(directory, ".ba")) ? "✓ initialized" : "✗ not initialized";
          const wm = existsSync(join(directory, ".wm")) ? "✓ initialized" : "✗ not initialized";
          const sg = existsSync(join(directory, ".superego")) ? "✓ initialized" : "✗ not initialized";

          return `Cloud Atlas AI Stack Status:\n\nba: ${ba}\nwm: ${wm}\nsuperego: ${sg}\n\nUse 'bottle-init' to initialize all components.`;
        },
      }),
    },
  };
};

export default Bottle;

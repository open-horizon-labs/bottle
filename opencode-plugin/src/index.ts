/**
 * Bottle - Cloud Atlas AI Core Stack for OpenCode
 *
 * Two things:
 * 1. npm meta-package - pulls in ba-opencode, wm-opencode, superego-opencode as dependencies
 * 2. OpenCode plugin - provides setup/orchestration tools (bottle-init, bottle-install, bottle-status)
 *    AND re-exports child plugins so users get all tools in one package
 */

import type { Plugin } from "@opencode-ai/plugin";
import { tool } from "@opencode-ai/plugin";
import { existsSync, writeFileSync } from "fs";
import { join } from "path";
import { spawnSync } from "child_process";

// Import child plugins to re-export
// @ts-expect-error - ba-opencode doesn't generate .d.ts files yet
import BA from "ba-opencode";
// @ts-expect-error - wm-opencode doesn't generate .d.ts files yet
import WM from "wm-opencode";
// @ts-expect-error - superego-opencode doesn't generate .d.ts files yet
import Superego from "superego-opencode";

// Helper: Check if a binary is available
function checkBinary(name: string): boolean {
  try {
    const result = spawnSync("command", ["-v", name], { encoding: "utf-8" });
    return result.status === 0;
  } catch {
    return false;
  }
}

// Helper: Detect available package managers
function detectPackageManagers(): { homebrew: boolean; cargo: boolean } {
  return {
    homebrew: checkBinary("brew"),
    cargo: checkBinary("cargo") || existsSync(`${process.env.HOME}/.cargo/bin/cargo`),
  };
}

const BottleOrchestration: Plugin = async ({ directory }) => {
  return {
    tool: {
      "bottle-init": tool({
        description: "Initialize the full Cloud Atlas AI stack (ba, wm, superego). Detects missing binaries and guides installation.",
        args: {},
        async execute() {
          const results: string[] = [];

          // Check which binaries are available
          const binaries = {
            ba: checkBinary("ba"),
            wm: checkBinary("wm"),
            sg: checkBinary("sg"),
          };

          // Detect missing binaries (regardless of project initialization state)
          const missing: string[] = [];
          if (!binaries.ba) missing.push("ba");
          if (!binaries.wm) missing.push("wm");
          if (!binaries.sg) missing.push("sg");

          // If binaries are missing, guide installation
          if (missing.length > 0) {
            const pkgManagers = detectPackageManagers();
            const available: string[] = [];
            if (pkgManagers.homebrew) available.push("homebrew");
            if (pkgManagers.cargo) available.push("cargo");

            results.push(`⚠️  Missing binaries: ${missing.join(", ")}`);
            results.push("");

            if (available.length > 0) {
              results.push(`Available installation methods: ${available.join(", ")}`);
              results.push("");
              results.push("To install, use the bottle-install tool:");
              results.push(`  bottle-install --binary=<name> --method=<${available.join("|")}>`);
              results.push("");
              results.push("Example:");
              missing.forEach((bin) => {
                results.push(`  bottle-install --binary=${bin} --method=${available[0]}`);
              });
              results.push("");
              results.push("After installation, run bottle-init again to complete setup.");
            } else {
              results.push("⚠️  No package manager found (homebrew or cargo).");
              results.push("");
              results.push("Install options:");
              results.push("1. Homebrew (macOS):");
              results.push('   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"');
              results.push("2. Rust/Cargo (cross-platform):");
              results.push("   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
            }

            return results.join("\n");
          }

          // All binaries available - initialize projects
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

          // Initialize superego
          if (!existsSync(join(directory, ".superego"))) {
            try {
              const sg = spawnSync("sg", ["init"], { cwd: directory, encoding: "utf-8" });
              results.push(sg.status === 0 ? "✓ superego initialized" : `✗ superego init failed: ${sg.stderr}`);
            } catch (e) {
              results.push(`✗ superego init failed: ${e}`);
            }
          } else {
            results.push("✓ superego already initialized");
          }

          // Handle AGENTS.md - create if missing, provide template for merge if exists
          const agentsFile = join(directory, "AGENTS.md");
          const agentsContent = `# Cloud Atlas AI Stack

This project uses Cloud Atlas AI tools. Follow these protocols:

## Task Tracking (ba)

**When to use:**
- At session start: Use \`ba-status\` to see active tasks
- Before starting work: Use \`ba list\` to check what's ready
- When creating tasks: Use \`ba create\` for each distinct piece of work
- During work: Use \`ba claim\` to take ownership, \`ba finish\` when done
- For dependencies: Use \`ba block\` to mark blockers

**Protocol:** Always track non-trivial work in ba. If a task has multiple steps or will take >5 minutes, create a task.

## Working Memory (wm)

**When to use:**
- When you need context: Use \`wm compile\` to get relevant knowledge for current work
- If you don't know why/how something works: Check \`wm show state\` or encourage user to prep a dive pack
- After completing work: Use \`wm distill\` to extract learnings from the session
- Before answering questions about past work: Check \`wm compile\` first

**Protocol:** Treat wm as your external memory. Don't guess at past decisions - check wm first.

## Metacognition (superego)

**Mode:** Pull mode - evaluates only when explicitly requested, not automatically.

**When to use:**
- Before committing significant work: Proactively request evaluation
- When uncertain about approach: Ask for feedback
- If you receive SUPEREGO FEEDBACK: critically evaluate it and either incorporate or escalate to user

**Protocol:** Superego is opt-in. Use it for high-stakes decisions, architectural choices, or when you want a second opinion. It catches premature commitment, scope creep, and misalignment.
`;

          if (!existsSync(agentsFile)) {
            // File doesn't exist - create it
            writeFileSync(agentsFile, agentsContent);
            results.push("✓ AGENTS.md created");
          } else {
            // File exists - provide template for AI to merge
            results.push("");
            results.push("AGENTS.md exists. AI: update these sections while preserving other content:");
            results.push("");
            results.push("```markdown");
            results.push(agentsContent);
            results.push("```");
          }

          return results.join("\n");
        },
      }),

      "bottle-install": tool({
        description: "Get installation commands for Cloud Atlas AI binaries (ba, wm, sg) via homebrew or cargo",
        args: {
          binary: tool.schema.enum(["ba", "wm", "sg"]).describe("Which binary to install"),
          method: tool.schema.enum(["homebrew", "cargo"]).describe("Installation method"),
        },
        async execute({ binary, method }) {
          // Package names for each method
          // Note: Not all homebrew taps exist yet - ba and wm need to be published
          const packages = {
            homebrew: {
              ba: "cloud-atlas-ai/ba/ba",  // TODO: Publish tap
              wm: "cloud-atlas-ai/wm/wm",  // TODO: Publish tap
              sg: "cloud-atlas-ai/superego/superego",  // ✓ Published
            },
            cargo: {
              ba: "ba",  // ✓ Published
              wm: "working-memory",  // ✓ Published as working-memory
              sg: "superego",  // ✓ Published
            },
          };

          const pkg = packages[method][binary];
          const binaryNames = { ba: "ba", wm: "wm", sg: "superego" };

          const results: string[] = [];
          results.push(`Installation command for ${binary} (${binaryNames[binary]}):`);
          results.push("");

          if (method === "homebrew") {
            results.push(`  brew install ${pkg}`);
          } else {
            results.push(`  cargo install ${pkg}`);
          }

          results.push("");
          results.push("Run this command in your terminal, then run 'bottle-init' again to initialize.");
          results.push("");
          results.push("Note: Cargo installations build from source and may take 5-10 minutes.");

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

          const binaries = {
            ba: checkBinary("ba") ? "✓ installed" : "✗ not installed",
            wm: checkBinary("wm") ? "✓ installed" : "✗ not installed",
            sg: checkBinary("sg") ? "✓ installed" : "✗ not installed",
          };

          return `Cloud Atlas AI Stack Status:

Binaries:
  ba: ${binaries.ba}
  wm: ${binaries.wm}
  sg: ${binaries.sg}

Projects:
  ba: ${ba}
  wm: ${wm}
  superego: ${sg}

Use 'bottle-init' to initialize all components.`;
        },
      }),
    },
  };
};

// Export orchestration plugin + child plugins as array
// This ensures users get bottle-* tools AND ba-*/wm-*/superego-* tools
export const Bottle: Plugin[] = [BottleOrchestration, BA, WM, Superego];

export default Bottle;

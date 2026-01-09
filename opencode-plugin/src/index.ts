/**
 * Bottle - Cloud Atlas AI Core Stack for OpenCode
 *
 * Two things:
 * 1. npm meta-package - pulls in ba-opencode, wm-opencode, superego-opencode as dependencies
 * 2. OpenCode plugin - provides setup/orchestration tools (bottle-init, bottle-install, bottle-status)
 *    bottle-init updates opencode.json to add child plugins automatically
 */

import type { Plugin } from "@opencode-ai/plugin";
import { tool } from "@opencode-ai/plugin";
import { existsSync, writeFileSync, readFileSync } from "fs";
import { join } from "path";
import { spawnSync } from "child_process";

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

          // Step 1: Check if child plugins are in opencode.json
          const opencodeConfigPath = join(directory, "opencode.json");
          if (existsSync(opencodeConfigPath)) {
            try {
              const configContent = readFileSync(opencodeConfigPath, "utf-8");
              const config = JSON.parse(configContent);
              const childPlugins = ["ba-opencode", "wm-opencode", "superego-opencode"];
              const pluginArray = Array.isArray(config.plugin) ? config.plugin : [];

              const missingPlugins = childPlugins.filter(p => !pluginArray.includes(p));

              if (missingPlugins.length > 0) {
                results.push("⚠️  Child plugins missing from opencode.json");
                results.push("");
                results.push("📋 Setup Steps:");
                results.push(`1. AI will now add ${missingPlugins.join(", ")} to opencode.json`);
                results.push("2. User: Restart OpenCode to load the new plugins");
                results.push("3. Run 'bottle-init' again to complete initialization");
                results.push("");
                results.push(`AI: Add ${missingPlugins.join(", ")} to the plugin array in opencode.json`);
                return results.join("\n");
              }
            } catch (e) {
              results.push(`⚠️  Could not read opencode.json: ${e}`);
              results.push("");
            }
          }

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

This project uses Cloud Atlas AI tools integrated with OpenCode.

## Task Tracking (ba)

**Available tools:** ba-status, ba-list, ba-create, ba-claim, ba-finish, ba-block

**Protocol:**
- At session start: Check ba-status for active tasks
- Track non-trivial work (multi-step or >5 minutes) as ba tasks
- Claim tasks before starting work, finish when done

## Working Memory (wm)

**Available tools:** wm compile, wm show (state|working|sessions), wm distill, wm compress

**Protocol:**
- Use wm compile to get relevant context for current work
- If you don't know why/how something works: Check wm show state or encourage user to prep a dive pack
- After completing work: Use wm distill to extract learnings
- Working memory accumulates tacit knowledge across sessions automatically

## Metacognition (superego)

**Available tools:** sg-review, sg-mode

**Protocol:**
- Superego is in pull mode - only runs when explicitly requested
- Use sg-review before major decisions or when uncertain about approach
- Review feedback critically and incorporate or discuss with user
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

          // Install key commands in .opencode/command/
          const commandDir = join(directory, ".opencode", "command");
          const superegCommand = join(commandDir, "superego-review.md");
          const wmDiveCommand = join(commandDir, "wm-dive-prep.md");

          const commandsToCreate = [];
          if (!existsSync(superegCommand)) commandsToCreate.push("superego-review.md");
          if (!existsSync(wmDiveCommand)) commandsToCreate.push("wm-dive-prep.md");

          if (commandsToCreate.length > 0) {
            results.push("");
            results.push(`AI: Create .opencode/command/ directory (if missing) and add these commands: ${commandsToCreate.join(", ")}`);
            results.push("");

            if (commandsToCreate.includes("superego-review.md")) {
              results.push("📝 .opencode/command/superego-review.md:");
              results.push("```markdown");
              results.push("---");
              results.push("description: Run metacognitive review of current work");
              results.push("---");
              results.push("");
              results.push("Run metacognitive review:");
              results.push("1. Use sg-review to get feedback on current approach");
              results.push("2. Review the feedback critically");
              results.push("3. Either incorporate suggestions or discuss with user if feedback seems off-track");
              results.push("```");
              results.push("");
            }

            if (commandsToCreate.includes("wm-dive-prep.md")) {
              results.push("📝 .opencode/command/wm-dive-prep.md:");
              results.push("```markdown");
              results.push("---");
              results.push("description: Prepare grounded dive session with context");
              results.push("---");
              results.push("");
              results.push("Prepare a grounded dive session with context from multiple sources:");
              results.push("1. Use wm show state to see what knowledge is available");
              results.push("2. Use wm compile to get relevant context for current work");
              results.push("3. Ask user what specific area they want to dive into");
              results.push("4. Gather context from: past sessions, code, docs, and working memory");
              results.push("5. Present synthesized dive pack with key insights and open questions");
              results.push("```");
            }
          } else {
            results.push("✓ Convenience commands installed");
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

// Export only the orchestration plugin
// Child plugins (ba, wm, superego) should be loaded separately in opencode.json
export default BottleOrchestration;

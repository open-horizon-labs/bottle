/**
 * Bottle - Cloud Atlas AI Thin Wrapper Plugin for OpenCode
 *
 * This plugin provides thin wrapper tools that invoke the bottle CLI.
 * All logic lives in the CLI - this plugin just passes through commands.
 */

import type { Plugin } from "@opencode-ai/plugin";
import { tool } from "@opencode-ai/plugin";
import { spawnSync } from "child_process";

// Helper: Check if bottle binary is available
function checkBottle(): { exists: boolean; error?: string } {
  try {
    const result = spawnSync("command", ["-v", "bottle"], {
      encoding: "utf-8",
      shell: true,
    });
    return { exists: result.status === 0 };
  } catch {
    return { exists: false };
  }
}

// Helper: Run bottle command and return output
function runBottle(args: string[], cwd: string): string {
  const check = checkBottle();
  if (!check.exists) {
    return `The bottle CLI is not installed.

Install with Homebrew:
  brew install oh-labs/tap/bottle

Or with Cargo:
  cargo install bottle

Then run this command again.`;
  }

  try {
    const result = spawnSync("bottle", args, {
      cwd,
      encoding: "utf-8",
      timeout: 60000,
    });

    if (result.error) {
      return `Error running bottle: ${result.error.message}`;
    }

    const output = (result.stdout || "") + (result.stderr || "");
    return output.trim() || "Command completed successfully.";
  } catch (e) {
    return `Error running bottle: ${e}`;
  }
}

const BottlePlugin: Plugin = async ({ directory }) => {
  return {
    tool: {
      "bottle-install": tool({
        description: "Install a bottle (curated tool stack)",
        args: {
          name: tool.schema.string().optional().describe("Bottle name (default: stable)"),
        },
        async execute({ name }) {
          const args = ["install"];
          if (name) args.push(name);
          return runBottle(args, directory);
        },
      }),

      "bottle-status": tool({
        description: "Show current bottle status and installed tools",
        args: {
          checkUpdates: tool.schema.boolean().optional().describe("Also check for available updates"),
        },
        async execute({ checkUpdates }) {
          const args = ["status"];
          if (checkUpdates) args.push("--check-updates");
          return runBottle(args, directory);
        },
      }),

      "bottle-update": tool({
        description: "Update to the latest bottle snapshot",
        args: {
          yes: tool.schema.boolean().optional().describe("Skip confirmation prompt"),
        },
        async execute({ yes }) {
          const args = ["update"];
          if (yes) args.push("-y");
          return runBottle(args, directory);
        },
      }),

      "bottle-switch": tool({
        description: "Switch to a different bottle",
        args: {
          name: tool.schema.string().describe("Bottle name to switch to"),
          yes: tool.schema.boolean().optional().describe("Skip confirmation prompt"),
        },
        async execute({ name, yes }) {
          const args = ["switch", name];
          if (yes) args.push("-y");
          return runBottle(args, directory);
        },
      }),

      "bottle-list": tool({
        description: "List available bottles (curated and bespoke)",
        args: {},
        async execute() {
          return runBottle(["list"], directory);
        },
      }),

      "bottle-create": tool({
        description: "Create a new bespoke bottle",
        args: {
          name: tool.schema.string().describe("Name for the new bottle"),
          from: tool.schema.string().optional().describe("Copy manifest from an existing bottle"),
        },
        async execute({ name, from }) {
          const args = ["create", name];
          if (from) args.push("--from", from);
          return runBottle(args, directory);
        },
      }),

      "bottle-integrate": tool({
        description: "Add or remove platform integrations",
        args: {
          platform: tool.schema.string().optional().describe("Platform: claude_code, opencode, or codex"),
          list: tool.schema.boolean().optional().describe("List available integrations"),
          remove: tool.schema.boolean().optional().describe("Remove instead of add"),
        },
        async execute({ platform, list, remove }) {
          const args = ["integrate"];
          if (list) {
            args.push("--list");
          } else if (platform) {
            if (remove) args.push("--remove");
            args.push(platform);
          }
          return runBottle(args, directory);
        },
      }),

      "bottle-eject": tool({
        description: "Eject from bottle management (keep tools, manage manually)",
        args: {
          yes: tool.schema.boolean().optional().describe("Skip confirmation prompt"),
        },
        async execute({ yes }) {
          const args = ["eject"];
          if (yes) args.push("-y");
          return runBottle(args, directory);
        },
      }),

      "bottle-init": tool({
        description: "Initialize all Cloud Atlas AI tools with recommended defaults",
        args: {},
        async execute() {
          // Note: There's no `bottle init` CLI command - initialization is done by
          // running ba init, wm init, and sg init individually. This tool provides
          // guidance for the agent to follow.
          return `To initialize the Cloud Atlas AI stack:

1. Initialize ba (task tracking):
   ba init

2. Initialize wm (working memory):
   wm init

3. Initialize superego (metacognition):
   sg init

After initialization, optionally configure superego to pull mode:
   Edit .superego/config.yaml and set mode: pull

For detailed guidance, see the bottle-init skill documentation.`;
        },
      }),

      "bottle-help": tool({
        description: "Show available bottle commands",
        args: {},
        async execute() {
          return runBottle(["--help"], directory);
        },
      }),
    },
  };
};

export default BottlePlugin;

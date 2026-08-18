const vscode = require("vscode");
const { execFile, exec } = require("child_process");
const path = require("path");
const fs = require("fs");

let languageClient;

/** @returns {string | undefined} */
function workspaceRoot() {
  const folders = vscode.workspace.workspaceFolders;
  if (!folders || folders.length === 0) {
    return undefined;
  }
  return folders[0].uri.fsPath;
}

/** @param {string} root */
function resolveRunner(root) {
  const config = vscode.workspace.getConfiguration("vpp");
  const configured = config.get("compilerPath", "");
  if (configured && fs.existsSync(configured)) {
    return { kind: "exe", path: configured };
  }

  const ps1 = path.join(root, "vpp.ps1");
  if (fs.existsSync(ps1)) {
    return { kind: "ps1", path: ps1 };
  }

  const cmd = path.join(root, "vpp.cmd");
  if (fs.existsSync(cmd)) {
    return { kind: "cmd", path: cmd };
  }

  for (const sub of ["target/release/vpp.exe", "target/debug/vpp.exe", "target/release/vppls.exe", "target/debug/vppls.exe", "target/release/vpp", "target/debug/vpp"]) {
    const candidate = path.join(root, sub);
    if (fs.existsSync(candidate)) {
      return { kind: "exe", path: candidate };
    }
  }

  return undefined;
}

/** @param {string} root */
function resolveLanguageServer(root) {
  const config = vscode.workspace.getConfiguration("vpp");
  const configured = config.get("languageServerPath", "vppls");
  if (configured && (path.isAbsolute(configured) ? fs.existsSync(configured) : true)) {
    if (path.isAbsolute(configured) && fs.existsSync(configured)) {
      return configured;
    }
    if (!path.isAbsolute(configured)) {
      for (const sub of [`target/debug/${configured}.exe`, `target/release/${configured}.exe`, `target/debug/${configured}`, `target/release/${configured}`]) {
        const candidate = path.join(root, sub);
        if (fs.existsSync(candidate)) {
          return candidate;
        }
      }
    }
  }
  return configured;
}

/** @param {{ kind: string, path: string }} runner @param {string[]} args @param {string} cwd */
function runRunner(runner, args, cwd) {
  if (runner.kind === "ps1") {
    return {
      command: "powershell",
      argv: ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", runner.path, ...args],
    };
  }
  if (runner.kind === "cmd") {
    return {
      command: "cmd",
      argv: ["/c", runner.path, ...args],
    };
  }
  return {
    command: runner.path,
    argv: args,
  };
}

/** @param {string} subcommand @param {string} filePath */
async function runVpp(subcommand, filePath) {
  const root = workspaceRoot();
  if (!root) {
    vscode.window.showErrorMessage("Open the v++ project folder in Cursor first.");
    return;
  }

  if (!filePath || !filePath.endsWith(".vpp")) {
    vscode.window.showErrorMessage("Open a .vpp file to run it.");
    return;
  }

  let runner = resolveRunner(root);
  if (!runner) {
    const build = "Build v++ compiler now? (one time, ~30 seconds)";
    const choice = await vscode.window.showInformationMessage(build, "Build", "Cancel");
    if (choice !== "Build") {
      return;
    }
    await buildCompiler(root);
    runner = resolveRunner(root);
    if (!runner) {
      vscode.window.showErrorMessage("Build finished but vpp was not found. Run .\\setup.ps1 in the project folder.");
      return;
    }
  }

  const output = vscode.window.createOutputChannel("v++");
  output.clear();
  output.show(true);
  output.appendLine(`> vpp ${subcommand} ${filePath}`);
  output.appendLine("");

  const { command, argv } = runRunner(runner, [subcommand, filePath], root);
  const execOpts = { cwd: root, maxBuffer: 10 * 1024 * 1024 };

  await new Promise((resolve) => {
    execFile(command, argv, execOpts, (err, stdout, stderr) => {
      if (stdout) {
        output.append(stdout);
      }
      if (stderr) {
        output.append(stderr);
      }
      if (err) {
        output.appendLine("");
        output.appendLine(`Exit code: ${err.code ?? 1}`);
        vscode.window.showErrorMessage(
          `v++ ${subcommand} failed — see the "v++" output panel for details`
        );
      } else if (subcommand === "run" && !stdout.trim()) {
        output.appendLine("(program finished with no stdout)");
      } else if (subcommand === "check") {
        vscode.window.showInformationMessage("✓ File type-checks successfully");
      }
      resolve(undefined);
    });
  });
}

/** @param {string} root */
function buildCompiler(root) {
  return new Promise((resolve, reject) => {
    const cargoBin = path.join(process.env.USERPROFILE || process.env.HOME || "", ".cargo", "bin");
    const env = { ...process.env };
    if (fs.existsSync(cargoBin)) {
      env.PATH = `${cargoBin};${env.PATH || ""}`;
    }

    exec("cargo build --features lsp,codegen", { cwd: root, env }, (err, stdout, stderr) => {
      if (err) {
        vscode.window.showErrorMessage(`cargo build failed: ${stderr || err.message}`);
        reject(err);
        return;
      }
      resolve(stdout);
    });
  });
}

function startLanguageServer(context) {
  const root = workspaceRoot();
  if (!root) {
    return;
  }

  const config = vscode.workspace.getConfiguration("vpp");
  if (!config.get("enableLanguageServer", true)) {
    return;
  }

  let LanguageClient;
  let TransportKind;
  try {
    ({ LanguageClient, TransportKind } = require("vscode-languageclient/node"));
  } catch {
    vscode.window.showWarningMessage(
      "v++ LSP: run `npm install` in editor/vscode-vpp to enable language server integration."
    );
    return;
  }

  const serverPath = resolveLanguageServer(root);
  languageClient = new LanguageClient(
    "vppLanguageServer",
    "v++ Language Server",
    {
      command: serverPath,
      args: [],
      transport: TransportKind.stdio,
      options: { cwd: root },
    },
    {
      documentSelector: [{ scheme: "file", language: "vpp" }],
      synchronize: {
        fileEvents: vscode.workspace.createFileSystemWatcher("**/*.vpp"),
      },
    }
  );

  languageClient.start();
  context.subscriptions.push({
    dispose: () => languageClient && languageClient.stop(),
  });
}

function activate(context) {
  startLanguageServer(context);

  context.subscriptions.push(
    vscode.commands.registerCommand("vpp.runFile", () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No file open.");
        return;
      }
      runVpp("run", editor.document.uri.fsPath);
    }),
    vscode.commands.registerCommand("vpp.checkFile", () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No file open.");
        return;
      }
      runVpp("check", editor.document.uri.fsPath);
    }),
    vscode.commands.registerCommand("vpp.testProject", () => {
      const root = workspaceRoot();
      if (!root) {
        return;
      }
      const runner = resolveRunner(root);
      if (!runner) {
        vscode.window.showErrorMessage("v++ compiler not found. Run .\\setup.ps1 once.");
        return;
      }
      const { command, argv } = runRunner(runner, ["test"], root);
      const term = vscode.window.createTerminal({ name: "vpp test", cwd: root });
      term.show(true);
      term.sendText([command, ...argv].join(" "));
    })
  );
}

function deactivate() {
  if (languageClient) {
    return languageClient.stop();
  }
}

module.exports = { activate, deactivate };

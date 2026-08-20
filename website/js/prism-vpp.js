/** V++ syntax + richer shell highlighting for Prism. */
(function (Prism) {
  if (!Prism.languages.clike) return;

  Prism.languages.vpp = Prism.languages.extend("clike", {
    comment: [
      { pattern: /\/\/.*$/m, greedy: true },
      { pattern: /\/\*[\s\S]*?\*\//, greedy: true },
    ],
    string: { pattern: /"(?:\\.|[^"\\])*"/, greedy: true },
    keyword:
      /\b(?:fn|struct|enum|import|return|let|mut|if|else|while|for|match|break|continue|true|false|pub|trait|impl|type|test|as|in|Some|None|Ok|Err|Option|Result|Self|where|loop|async|await|use|mod|const|static|ref|move|dyn|unsafe|extern)\b/,
    builtin:
      /\b(?:print|println|assert|assert_eq|len|push|pop|main|Some|None|Ok|Err)\b/,
    type: /\b(?:int|float|bool|string|void|char|byte|i8|i16|i32|i64|u8|u16|u32|u64|f32|f64|isize|usize)\b/,
    number: /\b0x[\da-fA-F]+\b|\b\d+(?:\.\d+)?\b/,
    operator: /[+\-*/%=<>!&|^~?:]+|\.\./,
  });

  if (Prism.languages.bash) {
    Prism.languages.powershell = Prism.languages.bash;
    Prism.languages.shell = Prism.languages.bash;
    Prism.languages.text = Prism.languages.bash;

    Prism.languages.bash = Prism.languages.extend("bash", {
      "command-name": {
        pattern: /(?:^|[\s|;&(])(?:vpp|cargo|clang|llvm|git|cd|mkdir|rm|cp|mv|python|node|npm|curl|wget|tar|unzip|make|cmake)(?=$|[\s|;&()])/m,
        lookbehind: true,
        alias: "function",
      },
      flag: {
        pattern: /(?:^|\s)-[\w-]+/,
        alias: "keyword",
      },
      variable: /\$[\w@#?*!-]+/,
    });
  }

  if (Prism.languages.toml) {
    Prism.languages.toml = Prism.languages.extend("toml", {
      key: {
        pattern: /(^|[\[\],]\s*)[\w.-]+(?=\s*=)/m,
        lookbehind: true,
        alias: "property",
      },
    });
  }
})(typeof Prism !== "undefined" ? Prism : {});

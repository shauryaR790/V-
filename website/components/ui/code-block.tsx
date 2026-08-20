"use client"

import React from "react"
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter"
import { atomDark } from "react-syntax-highlighter/dist/cjs/styles/prism"
import { Check, Copy } from "lucide-react"

/** V++ site token colors — match website/css/prism-vpp.css */
const vppAtomDark = {
  ...atomDark,
  'code[class*="language-"]': {
    ...atomDark['code[class*="language-"]'],
    color: "#e6edf3",
    background: "#000000",
  },
  comment: { color: "#8b949e", fontStyle: "italic" },
  punctuation: { color: "#e6edf3" },
  keyword: { color: "#ff7b72" },
  string: { color: "#a5d6ff" },
  number: { color: "#ffa657" },
  function: { color: "#d2a8ff" },
  "class-name": { color: "#d2a8ff" },
  property: { color: "#79c0ff" },
  builtin: { color: "#d2a8ff" },
}

type CodeBlockProps = {
  language: string
  filename: string
  highlightLines?: number[]
} & (
  | {
      code: string
      tabs?: never
    }
  | {
      code?: never
      tabs: Array<{
        name: string
        code: string
        language?: string
        highlightLines?: number[]
      }>
    }
)

const MIN_LINES = 5

function padCode(code: string, minimum = MIN_LINES): string {
  const lines = code.split("\n")
  if (lines.length > 1 && lines.at(-1) === "") lines.pop()
  while (lines.length < minimum) lines.push("")
  return lines.join("\n")
}

export const CodeBlock = ({
  language,
  filename,
  code,
  highlightLines = [],
  tabs = [],
}: CodeBlockProps) => {
  const [copied, setCopied] = React.useState(false)
  const [activeTab, setActiveTab] = React.useState(0)

  const tabsExist = tabs.length > 0

  const copyToClipboard = async () => {
    const textToCopy = tabsExist ? tabs[activeTab].code : code
    if (textToCopy) {
      await navigator.clipboard.writeText(textToCopy.trimEnd())
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    }
  }

  const activeCode = padCode(tabsExist ? tabs[activeTab].code : code || "")
  const activeLanguage = tabsExist
    ? tabs[activeTab].language || language
    : language
  const activeHighlightLines = tabsExist
    ? tabs[activeTab].highlightLines || []
    : highlightLines

  return (
    <div className="relative w-full rounded-lg bg-black p-0 font-mono text-sm border border-[#222] overflow-hidden">
      <div className="flex flex-col gap-0 border-b border-[#222] bg-black">
        {tabsExist && (
          <div className="flex overflow-x-auto border-b border-[#222]">
            {tabs.map((tab, index) => (
              <button
                key={index}
                onClick={() => setActiveTab(index)}
                className={`px-3 py-2 text-xs transition-colors font-sans border-b-2 ${
                  activeTab === index
                    ? "text-[#e6edf3] border-[#FBDB5A]"
                    : "text-[#8b949e] border-transparent hover:text-[#e6edf3]"
                }`}
              >
                {tab.name}
              </button>
            ))}
          </div>
        )}
        <div className="flex justify-between items-center py-1 px-3 min-h-[42px]">
          <div className="text-xs text-[#8b949e] font-sans">
            {tabsExist ? tabs[activeTab].name : filename}
          </div>
          <button
            onClick={copyToClipboard}
            className="flex items-center gap-1 text-xs text-[#8b949e] hover:text-[#e6edf3] transition-colors font-sans p-1 rounded hover:bg-white/5"
            aria-label="Copy code"
          >
            {copied ? (
              <Check className="h-3.5 w-3.5 text-[#7ee787]" />
            ) : (
              <Copy className="h-3.5 w-3.5" />
            )}
          </button>
        </div>
      </div>
      <SyntaxHighlighter
        language={activeLanguage}
        style={vppAtomDark}
        customStyle={{
          margin: 0,
          padding: "1rem 0",
          background: "#000000",
          fontSize: "0.875rem",
          lineHeight: "1.65",
        }}
        wrapLines={true}
        showLineNumbers={true}
        lineProps={(lineNumber) => ({
          style: {
            backgroundColor: activeHighlightLines.includes(lineNumber)
              ? "rgba(255,255,255,0.1)"
              : "transparent",
            display: "block",
            width: "100%",
          },
        })}
        PreTag="div"
      >
        {activeCode}
      </SyntaxHighlighter>
    </div>
  )
}

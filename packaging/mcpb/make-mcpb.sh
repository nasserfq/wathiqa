#!/usr/bin/env bash
# Claude desktop extension (.mcpb) for وثيقة on macOS: double-clicking it opens Claude's own
# «Install» window, which adds وثيقة's MCP server to Claude without editing any file.
#
# The extension holds only its description and the وثيقة logo: it runs the MCP server inside the
# installed app (/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli), so the server updates with the
# app and the file stays a few kilobytes. On other systems, and for other AI apps, وثيقة's own
# Help › Connect to AI apps window sets it up.
#
# usage: packaging/mcpb/make-mcpb.sh OUT_DIR VERSION
set -euo pipefail
OUT="${1:?out dir}"
VERSION="${2:?version}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
# Claude recommends a 512 px icon: scaled down with macOS's sips where it exists.
cp "$ROOT/assets/app-icon/pdfcraft-1024.png" "$WORK/icon.png"
if command -v sips >/dev/null; then sips -Z 512 "$WORK/icon.png" >/dev/null; fi
cat > "$WORK/manifest.json" <<JSON
{
  "manifest_version": "0.3",
  "name": "wathiqa",
  "display_name": "وثيقة (Wathiqa)",
  "version": "$VERSION",
  "description": "اقرأ ملفات PDF وعدّلها ووقّعها بكلامك في وثيقة، قارئ ومحرر PDF العربي.",
  "long_description": "وثيقة قارئ ومحرر عربي لملفات PDF مبني على PdfCraft. بعد تثبيت هذه الإضافة يفتح Claude ملفات PDF ويقرأ نصها ويبحث فيها ويدوّر الصفحات ويحذفها ويدمج الملفات ويضيف التعليقات ويعبّئ النماذج ويحفظها عندما تطلب منه ذلك. يحتاج وثيقة مثبتًا في مجلد التطبيقات.\n\nWathiqa is an Arabic PDF reader and editor based on PdfCraft. With this extension Claude can open, read, search, rotate, merge, comment on, fill and save PDFs. Needs Wathiqa installed in Applications.",
  "author": { "name": "Nasser Alqahtani", "url": "https://github.com/nasserfq/wathiqa" },
  "homepage": "https://github.com/nasserfq/wathiqa",
  "repository": { "type": "git", "url": "https://github.com/nasserfq/wathiqa.git" },
  "license": "MIT OR Apache-2.0",
  "icon": "icon.png",
  "server": {
    "type": "binary",
    "entry_point": "/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli",
    "mcp_config": {
      "command": "/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli",
      "args": ["mcp"]
    }
  },
  "keywords": ["pdf", "documents", "forms", "arabic", "وثيقة"],
  "compatibility": { "platforms": ["darwin"] }
}
JSON
python3 -m json.tool "$WORK/manifest.json" >/dev/null
mkdir -p "$OUT"
rm -f "$OUT/wathiqa-claude-extension-macos.mcpb"
(cd "$WORK" && zip -q -X "wathiqa-claude-extension-macos.mcpb" manifest.json icon.png)
mv "$WORK/wathiqa-claude-extension-macos.mcpb" "$OUT/"
echo "$OUT/wathiqa-claude-extension-macos.mcpb"

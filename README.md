<p align="center">
  <img src="assets/app-icon/hicolor/128x128/apps/ai.storyteller.pdfcraft.png" width="128" height="128" alt="وثيقة">
</p>

<h1 align="center">وثيقة</h1>

<p align="center">
  <b>قارئ ومحرر عربي لملفات PDF، مجاني ومفتوح المصدر</b><br>
  An Arabic PDF reader and editor, free and open source
</p>

<p align="center">
  <a href="https://github.com/nasserfq/wathiqa/releases/latest"><b>⬇︎ تنزيل · Download</b></a>
  &nbsp;·&nbsp; macOS &nbsp;·&nbsp; Windows &nbsp;·&nbsp; Linux
</p>

---

<div dir="rtl">

## عن وثيقة

وثيقة تطبيق لملفات PDF مصمم للعربية أولًا: الواجهة عربية ومن اليمين إلى اليسار، وكل شيء يعمل على جهازك دون حساب ولا سحابة.

- **واجهة عربية بالكامل** من اليمين إلى اليسار، مع خيار الإنجليزية.
- **قراءة ملفات PDF** والبحث فيها والتنقل بالعلامات والصفحات المصغّرة.
- **تحرير النص والصور** وإضافة نص عربي مُشكَّل، والتعليقات والتظليل.
- **تنظيم الصفحات**: دمج الملفات وتقسيمها وتدوير الصفحات وحذفها وإعادة ترتيبها.
- **النماذج والتوقيع**: تعبئة النماذج والتوقيع والختم.
- **الحماية**: كلمة مرور، وحجب المحتوى الحساس، وضغط الملفات.
- **التعرّف الضوئي على النصوص** للصفحات الممسوحة ضوئيًا.

## التثبيت

| النظام | الملف | الخطوات |
|---|---|---|
| macOS ‏(Apple Silicon وIntel، الإصدار 11 فأحدث) | `wathiqa-…-macos-universal.dmg` | افتح الملف واسحب وثيقة إلى «التطبيقات». موقّع وموثَّق من Apple. |
| Windows | `wathiqa-…-windows-x64-portable.zip` | فك الضغط وشغّل `Wathiqa.exe`. |
| Linux | `wathiqa-…-linux-x86_64.tar.gz` | فك الضغط وشغّل `wathiqa`. |

التحديث تلقائي: عند صدور إصدار جديد تعرض وثيقة «تحديث وإعادة تشغيل»، وتبقى إعداداتك وملفاتك كما هي.

## التحكم بوثيقة من الذكاء الاصطناعي (MCP)

وثيقة جاهز للذكاء الاصطناعي: يأتي مع خادم [MCP](https://modelcontextprotocol.io)، وهو معيار مفتوح تستخدمه تطبيقات الذكاء الاصطناعي لتشغيل البرامج. بعد ربطه تطلب بكلامك، مثلًا «افتح هذا الملف، ولخّص نصه، وأضف تعليقًا على الصفحة الأولى، وادمجه مع العقد، واحفظه»، فينفّذه الذكاء الاصطناعي في وثيقة خطوة بخطوة: فتح ملفات PDF وقراءة نصها والبحث فيها وتدوير الصفحات وحذفها ودمج الملفات وإضافة التعليقات والعلامات المائية وتعبئة النماذج والتعرف الضوئي على النصوص والحفظ.

يعمل مع أي تطبيق يدعم MCP، مثل Claude ‏(التطبيق وClaude Code)، وCodex من OpenAI، وCursor، وVS Code مع Copilot، وGemini CLI، وLM Studio ‏(نماذج تعمل على جهازك دون إنترنت). الخادم مجاني ويعمل على جهازك، أما الذكاء الاصطناعي فمن التطبيق الذي تختاره وبحسب اشتراكك فيه.

### الطريقة السهلة: زر واحد

- **من داخل وثيقة:** افتح قائمة «مساعدة» ← «ربط مع الذكاء الاصطناعي…»، ثم اضغط «ربط» بجانب التطبيق الذي تستخدمه (Claude، ‏Claude Code، ‏Codex، ‏Cursor، ‏VS Code، ‏Windsurf، ‏Gemini CLI، ‏LM Studio) وأعد تشغيله. تكتب وثيقة الإعداد في ملفات ذلك التطبيق بدلًا منك، وتحتفظ بنسخة احتياطية من الملف قبل أول تعديل. ولأي تطبيق آخر يدعم MCP اضغط «نسخ الإعداد» والصقه في إعداداته.
- **Claude على Mac:** نزّل `wathiqa-claude-extension-macos.mcpb` من [صفحة الإصدارات](https://github.com/nasserfq/wathiqa/releases/latest) وافتحه بنقرتين، فيعرض Claude نافذة «تثبيت». تحتاج وثيقة في مجلد التطبيقات.

**نصيحة:** اذكر اسم البرنامج بالحروف اللاتينية في طلبك، مثل «استخدم أدوات wathiqa لفتح الملف…»، ليستخدم الذكاء الاصطناعي وثيقة لا Acrobat أو Preview. وإن ربطت وثيقة وأنت في محادثة مفتوحة فابدأ محادثة جديدة.

### الطريقة اليدوية

**1. مكان الخادم:** يُثبَّت مع وثيقة ويُحدَّث معه.

| النظام | المسار |
|---|---|
| macOS | `/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli` |
| Windows | `wathiqa-cli.exe` في مجلد وثيقة، بجانب `Wathiqa.exe` |
| Linux | `wathiqa-cli` في مجلد وثيقة، بجانب `wathiqa` |

**2. أضفه إلى تطبيق الذكاء الاصطناعي:**

- **تطبيق Claude:** الإعدادات ← «Developer» ← «Edit Config»، ثم أضف ما يلي إلى الملف `claude_desktop_config.json` وأعد تشغيل Claude:

<div dir="ltr">

```json
{
  "mcpServers": {
    "wathiqa": {
      "command": "/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli",
      "args": ["mcp"]
    }
  }
}
```

</div>

- في Windows ضع مسار `wathiqa-cli.exe` بدل المسار السابق، مع مضاعفة الشرطة المائلة، مثل `"C:\\Users\\اسمك\\Wathiqa\\wathiqa-cli.exe"`.
- **Cursor وGemini CLI وLM Studio** وأغلب التطبيقات الأخرى: الإعداد نفسه (`mcpServers`) في ملف MCP الخاص بكل تطبيق (`~/.cursor/mcp.json`، ‏`~/.gemini/settings.json`، ‏`mcp.json` في LM Studio).
- **Claude Code:** <code dir="ltr">claude mcp add wathiqa -- "/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli" mcp</code>
- **Codex:** <code dir="ltr">codex mcp add wathiqa -- "/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli" mcp</code>

بهذا يعمل الذكاء الاصطناعي على الملفات في الخلفية دون نافذة، ثم يحفظها حيث تطلب.

## ما أضافه ناصر القحطاني إلى PdfCraft

- الواجهة العربية افتراضيًا، وعكس الواجهة كلها لتُقرأ من اليمين إلى اليسار، مع بقاء صفحات PDF كما هي.
- خادم MCP ضمن التنزيلات لتتحكم تطبيقات الذكاء الاصطناعي بوثيقة.
- الاسم والشعار، ونافذة التثبيت العربية في Mac.
- تطبيق Mac موقّع وموثَّق لأجهزة Apple Silicon وIntel، وتنزيلات Windows وLinux.
- التحديث من داخل التطبيق بنافذة عربية «تحديث وإعادة تشغيل».

## شكر

وثيقة مبنية على [PdfCraft](https://github.com/storytold/pdfcraft)، مع الشكر لفريق ArtCraft ومساهمي PdfCraft على عملهم الأصلي. وثيقة مشروع مستقل، وليست من إنتاج فريق ArtCraft ولا برعايته.

</div>

---

## About

Wathiqa (وثيقة, "document") is an Arabic-first PDF reader and editor by Nasser Alqahtani: an Arabic,
right-to-left interface (English available) over PdfCraft's PDF workbench: reading, search,
editing text and images, comments, organizing pages, forms and signing, protection, redaction and
OCR, all local.

Nasser's additions to PdfCraft: the Arabic interface as the default, the whole interface mirrored
right to left (PDF pages are shown as they are), the name and logo, the Arabic Mac installer
window, the signed universal Mac app and Windows and Linux downloads, and in-app updates from this
repository.

It is based on [PdfCraft](https://github.com/storytold/pdfcraft) by the ArtCraft team and the
PdfCraft contributors. Wathiqa is an independent project, not made, sponsored or endorsed by the
ArtCraft team, and carries none of the ArtCraft trademarks.

## Use it from AI apps (MCP)

Wathiqa ships an [MCP](https://modelcontextprotocol.io) server, so Claude and any other MCP app
(Codex, Cursor, VS Code, Gemini CLI, LM Studio…) can open, read, search, rotate, merge, comment on, fill and save PDFs. It is installed and updated with
the app: `/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli` on macOS, `wathiqa-cli.exe` beside `Wathiqa.exe` on Windows, `wathiqa-cli` beside `wathiqa`
on Linux. The easiest way to add it is Help ▸ Connect to AI apps in Wathiqa (one button per AI app; it writes that app's settings for you), or on a Mac the `wathiqa-claude-extension-macos.mcpb` from the releases page for Claude. By hand, add it to your client as that command with the argument `mcp` (for example
`claude mcp add wathiqa -- "/Applications/وثيقة.app/Contents/MacOS/wathiqa-cli" mcp`). The server
works on PDF files directly, without a window.

## Build from source

Install [Rust](https://rustup.rs), then build with the interface translations
([storytold/craft-i18n](https://github.com/storytold/craft-i18n), at the commit in `craft-i18n.rev`)
and, for the best Arabic text, the interface fonts ([storytold/craft-fonts](https://github.com/storytold/craft-fonts)):

```sh
git clone https://github.com/storytold/craft-i18n ../craft-i18n
git -C ../craft-i18n checkout "$(cat craft-i18n.rev)"
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_I18N_DIR="$(cd ../craft-i18n && pwd)" CRAFT_FONTS_DIR="$(cd ../craft-fonts && pwd)" \
  cargo run --release -p pdfcraft
```

`CRAFT_I18N_DIR` must be an absolute path, or the interface is English only.

Releases: pushing to the `release` branch builds, signs and publishes the downloads
(`.github/workflows/release.yml`).

## License

MIT OR Apache-2.0, like PdfCraft: see [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE),
[NOTICE](NOTICE) and [ATTRIBUTION.md](ATTRIBUTION.md).

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

## ما أضافه ناصر القحطاني إلى PdfCraft

- الواجهة العربية افتراضيًا، وعكس الواجهة كلها لتُقرأ من اليمين إلى اليسار، مع بقاء صفحات PDF كما هي.
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

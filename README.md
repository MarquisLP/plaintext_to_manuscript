# plaintext_to_manuscript

## The Situation

I'm [a writer](https://www.markjjpadilla.com/writing), and one whose writing space spans more than one device. If I'm not typing up a new sci-fi saga on my Linux desktop with Gedit, then I'm revising that same saga on my work commute using a handful of different text editing apps on my phone. To make things quick and easy to sync between devices, as well as easy to view and edit on a mobile screen, I mainly work with plaintext files.

The benefit of this approach? A _huge_ boost to productivity. The Procrastinating Agent of the brain can no longer use the excuse of: "I'm keeping the file hostage at home so you can't do anything." \*cocks gun\* Late night edits in bed are also a fun way to fall asleep. (Or delay it…)

The downside? _Formatting._ When it comes time to submit these stories for publication, publishers generally prefer or even require that the typesetting be in proper manuscript format. This means proper em dashes `—` instead of double hyphens `--`, quotation marks `“”` instead of ticks `""`, and so on.

Before writing this tool, I would copy and paste my stories into a word processor and manually fix the formatting. Some things are easy to fix (find-and-replace makes quick work of em dashes) while other things (opening and closing quotation marks, oh my) are very time-consuming and finicky.

## The Solution

That brings us to the `plaintext_to_manuscript` tool. It's a simple CLI tool written in Rust that takes any plaintext file as input and spits it back out—but with all the proper manuscript formatting properly applied!

Right now, it formats the following:
- Apostrophes
- Em-dashes
- Ellipsis
- Single quotation marks
- Double quotation marks
- Double newlines --> Single newlines

After running the tool and getting the output, all you need to do is post it into a word processor, add indentation and headings, and you're good to go!

Unfortunately, if you're a fan of Markdown like I am, this tool won't do anything about italics and boldings and such, so you'll still need to apply those manually in a word processor. But perhaps someday, this disclaimer won't need to be here anymore…

## Running It

Go to the [Releases page](https://github.com/MarquisLP/plaintext_to_manuscript/releases), click on the latest release, and download the application file. The file to download will differ depending on your operating system:
- **Linux** - `plaintext_to_manuscript_ubuntu-latest`
- **Linux** - `plaintext_to_manuscript_macos-latest`
- **Windows** - `plaintext_to_manuscript.exe`

Next, on Linux or Mac, make the application executable:

`chmod +x plaintext_to_manuscript`

And to actually run the application, use:

`./plaintext_to_manuscript path_to_file`

This will print the output straight to stdout. But you'll probably want to output it to a file. To do that, you can redirect stdout to a filepath:

`./plaintext_to_manuscript path_to_file > /home/me/my_awesome_manuscript_formatted_file.txt`

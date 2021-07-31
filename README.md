# TOC to Markdown Converter

Convert a .toc from pdflatex to markdown.  
Example see below! 

## Getting Started

Download and run binary files

|OS|Link|
|---|---|
|Windows|[latex-toc-markdown.exe](release/windows/latex-toc-markdown.exe)|
|Unix (x86)|[latex-toc-markdown](release/unix-x86/latex-toc-markdown)|
|Unix (ARM)|TODO|



## Command Syntax

```
./latex-toc-markdown Input.toc Output.toc
```

## Sample Files

### Input.toc

```
\contentsline {chapter}{Abbildungsverzeichnis}{4}{chapter*.2}%
\contentsline {chapter}{\numberline {1}Einführung}{8}{chapter.1}%
\contentsline {section}{\numberline {1.1}Einleitung}{8}{section.1.1}%
\contentsline {section}{\numberline {1.2}ImageMagick}{8}{section.1.2}%
\contentsline {chapter}{\numberline {2}Hintergrund}{11}{chapter.2}%
\contentsline {section}{\numberline {2.1}Remote Code Execution}{11}{section.2.1}%
\contentsline {section}{\numberline {2.2}Shell Grundlagen}{11}{section.2.2}%
\contentsline {section}{\numberline {2.3}Datei-Typen}{12}{section.2.3}%
\contentsline {section}{\numberline {2.4}ImageMagick}{13}{section.2.4}%
\contentsline {subsection}{\numberline {2.4.1}Allgemeines}{13}{subsection.2.4.1}%
\contentsline {subsection}{\numberline {2.4.2}Installation}{13}{subsection.2.4.2}%
\contentsline {subsection}{\numberline {2.4.3}Delegates}{17}{subsection.2.4.3}%
```

### Output.md

```
* Abbildungsverzeichnis
* 1 Einführung
  * 1.1 Einleitung
  * 1.2 ImageMagick
* 2 Hintergrund
  * 2.1 Remote Code Execution
  * 2.2 Shell Grundlagen
  * 2.3 Datei-Typen
  * 2.4 ImageMagick
    * 2.4.1 Allgemeines
    * 2.4.2 Installation
    * 2.4.3 Delegates
```
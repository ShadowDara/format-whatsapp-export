# WA Chat Export

*currently in develepment, not usable yet, but **soon !!!***

Early Versions are working.

<!--
git tag v0.1.0 && git push origin v0.1.0
-->

## How

[Download](https://github.com/ShadowDara/format-whatsapp-export/releases) the programm for your OS and follow the intructions from the programm

or use ... to install the Program
```sh
go install ithub.com/shadowdara/format-whatsapp-export@latest
```

## TODO / Ideas
- [ ] add Description and Explanation to the Program
- [ ] HTML and CSS Compressor
- [ ] Color Themes
- [X] intern Pages (virtual DOM?)
- [ ] Search Option for dates and time
- [ ] add images and other files too
- [x] calculate messages and other statistics
- [ ] need to listen on change for the settings
- [ ] add Dark Mode
- [ ] add Hover CSS
- [ ] Build for Linux ARM
- [ ] add more Languages
- [ ] saving page in localstorage
- [ ] saving settings in localstorage

## Functionality

go writes the output in the html files as js which will then
be formatted by js

## Changes
- [see CHANGELOG](/CHANGELOG.md)

## Problems
- [ ] Programm makes an undefined behaviour when the folder with the chat export
has multipple `.txt` files contained
- [ ] Messages which are using [ENTER] do not work

## LICENSES
dialog - ISC Dialog Authors - https://github.com/sqweek/dialog
